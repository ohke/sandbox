use crate::packet::TCPPacket;
use crate::socket::{SockID, Socket, TcpStatus};
use crate::tcpflags;
use anyhow::{Context, Result};
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::transport::{self, TransportChannelType};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr};
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex, RwLock, RwLockWriteGuard};
use std::time::{Duration, SystemTime};
use std::{cmp, ops::Range, str, thread};

const UNDETERMINED_IP_ADDR: std::net::Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
const UNDETERMINED_PORT: u16 = 0;
const MAX_TRANSMITTION: u8 = 5;
const RETRANSMITTION_TIMEOUT: u64 = 3;
const MSS: usize = 1460;
const PORT_RANGE: Range<u16> = 40000..60000;

pub struct TCP {
    // 3スレッドでRead/Write
    sockets: RwLock<HashMap<SockID, Socket>>,
    // イベントまで待機するため
    event_condvar: (Mutex<Option<TCPEvent>>, Condvar),
}

impl TCP {
    pub fn new() -> Arc<Self> {
        let sockets = RwLock::new(HashMap::new());
        let tcp = Arc::new(Self {
            sockets,
            event_condvar: (Mutex::new(None), Condvar::new()),
        });

        let cloned_tcp = tcp.clone();
        std::thread::spawn(move || {
            // パケット受信用のスレッド
            cloned_tcp.receive_handler().unwrap();
        });

        tcp
    }

    fn select_unused_port(&self, rng: &mut ThreadRng) -> Result<u16> {
        // start .. end で未使用のポート番号を選択
        for _ in 0..(PORT_RANGE.end - PORT_RANGE.start) {
            let local_port = rng.gen_range(PORT_RANGE);
            let table = self.sockets.read().unwrap();
            if table.keys().all(|k| local_port != k.2) {
                return Ok(local_port);
            }
        }
        anyhow::bail!("no available port found.")
    }

    fn receive_handler(&self) -> Result<()> {
        dbg!("begin recv thread.");

        // IP層のチャネルを作成
        let (_, mut receiver) = transport::transport_channel(
            65535,
            TransportChannelType::Layer3(IpNextHeaderProtocols::Tcp),
        )
        .unwrap();

        // パケットを待ち受ける
        let mut packet_iter = transport::ipv4_packet_iter(&mut receiver);
        loop {
            let (packet, remote_addr) = match packet_iter.next() {
                Ok((p, r)) => (p, r),
                Err(_) => continue,
            };
            let local_addr = packet.get_destination();
            let tcp_packet = match TcpPacket::new(packet.payload()) {
                Some(p) => p,
                None => continue,
            };
            let packet = TCPPacket::from(tcp_packet);
            let remote_addr = match remote_addr {
                IpAddr::V4(addr) => addr,
                _ => continue,
            };
            let mut table = self.sockets.write().unwrap();
            let socket = match table.get_mut(&SockID(
                local_addr,
                remote_addr,
                packet.get_dest(),
                packet.get_src(),
            )) {
                Some(socket) => socket, // 接続済みソケット
                None => match table.get_mut(&SockID(
                    local_addr,
                    UNDETERMINED_IP_ADDR,
                    packet.get_dest(),
                    UNDETERMINED_PORT,
                )) {
                    Some(socket) => socket, // リスニングソケット
                    None => continue,       // 以上に該当しないものは無視
                },
            };

            if !packet.is_correct_checksum(local_addr, remote_addr) {
                dbg!("invalid checksum.");
                continue;
            }

            let sock_id = socket.get_sock_id();
            if let Err(error) = match socket.status {
                TcpStatus::Listen => self.listen_handler(table, sock_id, &packet, remote_addr),
                TcpStatus::SynRcvd => self.synrcvd_handler(table, sock_id, &packet),
                TcpStatus::SynSent => self.synsent_handler(socket, &packet),
                _ => {
                    dbg!("not implemented state.");
                    Ok(())
                }
            } {
                dbg!(error);
            }
        }
    }

    fn listen_handler(
        &self,
        mut table: RwLockWriteGuard<HashMap<SockID, Socket>>,
        listening_socket_id: SockID,
        packet: &TCPPacket,
        remote_addr: Ipv4Addr,
    ) -> Result<()> {
        dbg!("listen handler");

        if packet.get_flag() & tcpflags::ACK > 0 {
            // TODO: 本来ならRSTをsendする
            return Ok(());
        }

        let listening_socket = table.get_mut(&listening_socket_id).unwrap();
        if packet.get_flag() & tcpflags::SYN > 0 {
            // 接続中ソケットを生成して、初期化
            let mut connected_socket = Socket::new(
                listening_socket.local_addr,
                remote_addr,
                listening_socket.local_port,
                packet.get_src(),
                TcpStatus::SynRcvd, // SYNRCVD状態に遷移させておく
            )?;
            connected_socket.recv_param.next = packet.get_seq() + 1;
            connected_socket.recv_param.initial_seq = packet.get_seq();
            connected_socket.send_param.initial_seq = rand::thread_rng().gen_range(1..1 << 31);
            connected_socket.send_param.next = packet.get_seq() + 1;
            connected_socket.send_param.window = packet.get_window();

            // 接続中ソケットから SYN|ACK を送信
            connected_socket.send_tcp_packet(
                connected_socket.send_param.initial_seq,
                connected_socket.send_param.next,
                tcpflags::SYN | tcpflags::ACK,
                &[],
            )?;
            connected_socket.send_param.next = connected_socket.send_param.initial_seq + 1;
            connected_socket.send_param.unacked_seq = connected_socket.send_param.initial_seq;
            connected_socket.listening_socket = Some(listening_socket.get_sock_id()); // 生成元をセット

            dbg!("status: listen -> ", &connected_socket.status);

            // 接続中ソケットを登録
            table.insert(connected_socket.get_sock_id(), connected_socket);
        }

        Ok(())
    }

    fn synrcvd_handler(
        &self,
        mut table: RwLockWriteGuard<HashMap<SockID, Socket>>,
        sock_id: SockID,
        packet: &TCPPacket,
    ) -> Result<()> {
        dbg!("synrcvd handler");

        let socket = table.get_mut(&sock_id).unwrap();

        if packet.get_flag() & tcpflags::ACK > 0
            && socket.send_param.unacked_seq <= packet.get_ack()
            && packet.get_ack() <= socket.send_param.next
        {
            // 接続済みソケットへ昇格
            socket.recv_param.next = packet.get_seq();
            socket.send_param.unacked_seq = packet.get_ack();
            socket.status = TcpStatus::Established;
            dbg!("status: synrcvd -> ", &socket.status);

            // コネクションキューへ登録
            if let Some(id) = socket.listening_socket {
                let ls = table.get_mut(&id).unwrap();
                ls.connected_connection_queue.push_back(sock_id);
                self.publish_event(ls.get_sock_id(), TCPEventKind::ConnectionCompleted);
            }
        }

        Ok(())
    }

    fn synsent_handler(&self, socket: &mut Socket, packet: &TCPPacket) -> Result<()> {
        // SYNSENT状態のソケットに到着したパケットのハンドラ
        dbg!("synsent handler.");

        if packet.get_flag() & tcpflags::ACK > 0 // コネクションを確立したセグメントはACKがセットされている
            && packet.get_flag() & tcpflags::SYN > 0
            // セグメントのack番号は unacked_seq <= ack <= next に含まれる必要がある
            && socket.send_param.unacked_seq <= packet.get_ack()
            && packet.get_ack() <= socket.send_param.next
        {
            socket.recv_param.next = packet.get_seq();
            socket.recv_param.initial_seq = packet.get_seq();
            socket.send_param.unacked_seq = packet.get_ack();
            socket.send_param.window = packet.get_window();

            if socket.send_param.unacked_seq > socket.send_param.initial_seq {
                socket.status = TcpStatus::Established;
                socket.send_tcp_packet(
                    socket.send_param.next,
                    socket.recv_param.next,
                    tcpflags::ACK,
                    &[],
                )?;
                dbg!("status: synsent -> ", &socket.status);
                self.publish_event(socket.get_sock_id(), TCPEventKind::ConnectionCompleted);
            } else {
                socket.status = TcpStatus::SynRcvd;
                socket.send_tcp_packet(
                    socket.send_param.next,
                    socket.recv_param.next,
                    tcpflags::ACK,
                    &[],
                )?;
                dbg!("status: synsent -> ", &socket.status);
            }
        }

        Ok(())
    }

    // スリーウェイハンドシェイク
    //   1. クライアント --- SYN (シーケンス番号: i) --> サーバ
    //   2. サーバ --- SYN + ACK (シーケンス番号: i, ACK番号: j=i+1) --> クライアント
    //   3. クライアント --- ACK (シーケンス番号: i+1, ACK番号: j) --> サーバ
    //   以降は、シーケンス番号とACK番号を使って通信
    pub fn connect(&self, addr: Ipv4Addr, port: u16) -> Result<SockID> {
        let mut rng = rand::thread_rng();
        let mut socket = Socket::new(
            get_source_addr_to(addr)?,
            addr,
            self.select_unused_port(&mut rng)?,
            port,
            TcpStatus::SynSent,
        )?;

        // TCPシーケンス番号予測攻撃を防ぐためにランダムにする
        socket.send_param.initial_seq = rng.gen_range(1..1 << 31);
        socket.send_tcp_packet(socket.send_param.initial_seq, 0, tcpflags::SYN, &[])?;
        socket.send_param.unacked_seq = socket.send_param.initial_seq;
        socket.send_param.next = socket.send_param.initial_seq + 1;

        let mut table = self.sockets.write().unwrap();
        let sock_id = socket.get_sock_id();
        table.insert(sock_id, socket);

        drop(table); // ロック解除
        self.wait_event(sock_id, TCPEventKind::ConnectionCompleted);

        Ok(sock_id)
    }

    fn wait_event(&self, sock_id: SockID, kind: TCPEventKind) {
        // 指定したソケットIDと種別のイベントを待機
        let (lock, cvar) = &self.event_condvar;
        let mut event = lock.lock().unwrap();
        loop {
            if let Some(ref e) = *event {
                if e.sock_id == sock_id && e.kind == kind {
                    break;
                }
            }
            event = cvar.wait(event).unwrap();
        }
        dbg!(&event);
        *event = None;
    }

    fn publish_event(&self, sock_id: SockID, kind: TCPEventKind) {
        // 指定のソケットIDにイベントを発行する
        let (lock, cvar) = &self.event_condvar;
        let mut e = lock.lock().unwrap();
        *e = Some(TCPEvent::new(sock_id, kind));
        cvar.notify_all();
    }

    pub fn listen(&self, local_addr: Ipv4Addr, local_port: u16) -> Result<SockID> {
        // リスニングソケットを生成・登録
        let socket = Socket::new(
            local_addr,
            UNDETERMINED_IP_ADDR, // 接続先は未定
            local_port,
            UNDETERMINED_PORT,
            TcpStatus::Listen,
        )?;
        let mut lock = self.sockets.write().unwrap();
        let sock_id = socket.get_sock_id();
        lock.insert(sock_id, socket);
        Ok(sock_id)
    }

    pub fn accept(&self, sock_id: SockID) -> Result<SockID> {
        // 接続済みソケットが生成されるまで待機
        self.wait_event(sock_id, TCPEventKind::ConnectionCompleted);

        // 生成されたSocketIDを返す
        let mut table = self.sockets.write().unwrap();
        Ok(table
            .get_mut(&sock_id)
            .context(format!("no such socket: {:?}", sock_id))?
            .connected_connection_queue
            .pop_front()
            .context("no connected socket.")?)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TCPEvent {
    sock_id: SockID,
    kind: TCPEventKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TCPEventKind {
    ConnectionCompleted,
    Acked,
    DataArrived,
    ConnectionClosed,
}

impl TCPEvent {
    fn new(sock_id: SockID, kind: TCPEventKind) -> Self {
        Self { sock_id, kind }
    }
}

fn get_source_addr_to(addr: Ipv4Addr) -> Result<Ipv4Addr> {
    // 宛先IPアドレスに対する送信元インタフェースのIPアドレスを取得する
    // ex. 10.0.0.1 via 192.168.64.1 dev enp0s2 src 192.168.64.2 uid 0
    //     -> 192.168.64.2
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("ip route get {} | grep src", addr))
        .output()?;
    dbg!(&output);
    let mut output = str::from_utf8(&output.stdout)?
        .trim()
        .split_ascii_whitespace();
    while let Some(s) = output.next() {
        if s == "src" {
            break;
        }
    }
    let ip = output.next().context("failed to get src ip.")?;
    dbg!("source addr: ", ip);
    ip.parse().context("failed to parse source ip.")
}
