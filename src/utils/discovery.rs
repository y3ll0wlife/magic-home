use network_interface::{Addr, NetworkInterface, NetworkInterfaceConfig};
use std::fmt::Display;
use std::fmt::{Error, Formatter};
use std::net::UdpSocket;
use std::str::from_utf8;
use std::time::Duration;

#[derive(Debug)]
pub struct Device {
    pub address: String,
    pub id: String,
    pub model: String,
}

impl Display for Device {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            fmt,
            "{} - Model: {} ({})",
            self.address, self.model, self.id
        )
    }
}

const PORT: usize = 48899;
const BROADCAST_MESSAGE: &str = "HF-A11ASSISTHREAD";

pub fn discover(timeout_seconds: u64) -> Vec<Device> {
    let network_interfaces = NetworkInterface::show().unwrap();
    let mut addresses: Vec<String> = Vec::new();

    for itf in network_interfaces.iter() {
        match itf.addr.unwrap() {
            Addr::V4(addr) => {
                if addr.ip.to_string().starts_with("127.0.0") {
                    continue;
                }
                match addr.broadcast {
                    Some(_) => {
                        let ip = addr.ip.to_string();
                        let mut vec: Vec<&str> = ip.split(".").collect();
                        vec[3] = "255";

                        addresses.push(vec.join("."));
                    }
                    None => continue,
                };
            }
            Addr::V6(_) => continue,
        }
    }

    addresses.push("255.255.255.255".to_string());

    let socket = UdpSocket::bind("0.0.0.0:0").expect("unable to bind to address");
    socket
        .set_broadcast(true)
        .expect("set_broadcast call failed");
    socket
        .set_read_timeout(Some(Duration::new(timeout_seconds, 0)))
        .expect("failed to set_read_timeout");

    for addr in &addresses {
        let full_addr = format!("{}:{}", addr, PORT);

        match socket.send_to(BROADCAST_MESSAGE.as_bytes(), &full_addr) {
            Ok(status) => println!("[DEBUG] OK {} on {}", status, &full_addr),
            Err(err) => println!("failed to broadcast on {} {}", &full_addr, err),
        };
    }

    let mut devices: Vec<Device> = Vec::new();
    let mut buf = [0; 100];

    match socket.recv_from(buf.as_mut_slice()) {
        Ok((amt, _)) => {
            let data = from_utf8(&mut buf[..amt]);
            let vec: Vec<&str> = data.unwrap().split(",").collect();

            devices.push(Device {
                address: vec[0].to_string(),
                id: vec[1].to_string(),
                model: vec[2].to_string(),
            });
        }
        Err(err) => println!("failed to recv_from {}", err),
    }

    devices
}
