#![allow(dead_code)]

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

pub fn change_color_brightness(stream: &TcpStream, red: u8, green: u8, blue: u8, brightness: u8) {
    let mut r = (255 / 100) * brightness;
    let mut g = (255 / 100) * brightness;
    let mut b = (255 / 100) * brightness;

    if red > 0 || green > 0 || blue > 0 {
        r = red / 100 * brightness;
        g = green / 100 * brightness;
        b = blue / 100 * brightness;
    }

    change_color(&stream, r, g, b, 0);
}
