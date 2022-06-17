#![allow(dead_code)]

use std::net::TcpStream;
use std::os::windows::prelude::AsRawSocket;

pub fn status(stream: &TcpStream) {
    println!(
        "{} → {} via socket {}",
        &stream.local_addr().unwrap(),
        &stream.peer_addr().unwrap(),
        &stream.as_raw_socket()
    );
}

pub fn connect(ip: &str, port: usize) -> TcpStream {
    let connect_addr: String = format!("{}:{}", ip, port);
    let stream = TcpStream::connect(connect_addr).expect("unable to connect to light");

    stream
}
