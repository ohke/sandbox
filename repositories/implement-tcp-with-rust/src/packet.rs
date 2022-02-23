use crate::tcpflags;
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::util;

use std::fmt::{self, Debug};
use std::net::Ipv4Addr;
const TCP_HEADER_SIZE: usize = 20;

#[derive(Clone)]
pub struct TCPPacket {
    // ※ ビッグエンディアン
    // 20 byte: Header
    //   2 byte: 送信元ポート番号
    //   2 byte: 宛先ポート番号
    //   4 byte: シーケンス番号
    //   4 byte: 確認応答番号
    //   4 bit: データオフセット
    //   6 bit: (予約)
    //   6 bit: コントロールフラグ
    //   2 byte: ウィンドウサイズ
    //   2 byte: チェックサム
    //   2 byte: 緊急ポインタ
    //   4 byte: オプション + パディング
    // 0~ byte: Payload
    buffer: Vec<u8>,
}

impl TCPPacket {
    pub fn new(payload_len: usize) -> Self {
        Self {
            buffer: vec![0; TCP_HEADER_SIZE + payload_len],
        }
    }

    pub fn set_src(&mut self, port: u16) {
        self.buffer[0..2].copy_from_slice(&port.to_be_bytes());
    }

    pub fn set_dest(&mut self, port: u16) {
        self.buffer[2..4].copy_from_slice(&port.to_be_bytes());
    }

    pub fn set_offset(&mut self, offset: u8) {
        self.buffer[12] |= offset << 4;
    }

    pub fn set_flag(&mut self, flag: u8) {
        // コントロールフラグは厳密には6ビットだが、使われていない予約領域の後ろ2ビットも含めて1バイトとしている
        self.buffer[13] = flag;
    }
}

impl Packet for TCPPacket {
    fn packet(&self) -> &[u8] {
        &self.buffer
    }
    fn payload(&self) -> &[u8] {
        &self.buffer[TCP_HEADER_SIZE..]
    }
}
