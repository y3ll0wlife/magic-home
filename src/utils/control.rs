use std::io::prelude::*;
use std::net::TcpStream;

fn set_power(mut stream: &TcpStream, on: bool) {
    stream
        .write(&[0x71, if on { 0x23 } else { 0x24 }, 0x0F])
        .expect("failed to change the power of the lights");
}

pub fn turn_on(stream: &TcpStream) {
    set_power(stream, true);
}

pub fn turn_off(stream: &TcpStream) {
    set_power(stream, false);
}

pub fn change_color(mut stream: &TcpStream, red: u8, green: u8, blue: u8, white: u8) {
    let buffer: &[u8; 8] = &[0x31, red, green, blue, white, 0x00, 0x0f, 0x00];
    stream.write(buffer).expect("failed to write to the light");
}
