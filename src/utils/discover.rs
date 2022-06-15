use std::net::UdpSocket;

use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig, V4IfAddr};

pub fn discover() {
    let port = 48899;

    let network_interfaces = NetworkInterface::show().unwrap();
    let mut addresses: Vec<V4IfAddr> = Vec::new();

    for itf in network_interfaces.iter() {
        match itf.addr.unwrap() {
            Addr::V4(addr) => addresses.push(addr),
            Addr::V6(_) => continue,
        }
    }

    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");

    for addr in addresses {
        let full_addr = format!("{}:{}", addr.ip, port);

        match socket.send_to(b"HF-A11ASSISTHREAD", full_addr) {
            Err(_) => println!("ERROR on {}:{}", addr.ip, port),
            Ok(_) => println!("OK on {}:{}", addr.ip, port),
        };
    }

    todo!();
}
