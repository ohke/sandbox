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
    sockets: RwLock<HashMap<SockID, SockID>>,
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
        self.wait_event(sock_id, TCPEventKind::ConnetionCompleted);

        Ok(sock_id)
    }
}

fn get_source_addr_to(addr: Ipv4Addr) -> Result<Ipv4Addr> {
    Ok("10.0.0.1".parse().unwrap())
}
