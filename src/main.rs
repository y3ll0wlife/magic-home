mod utils;
use core::panic;
use std::net::TcpStream;
use std::process::exit;

use dialoguer::{theme::ColorfulTheme, Input, Select};
use utils::control::{change_color, turn_off, turn_on};
use utils::discovery::discover;
use utils::light::{connect, status};
fn main() {
    for device in discover() {
        let stream = connect(&device.address, 5577);
        println!("Found device {}", device.address);
        _menu(&stream);
    }
}

fn _menu(stream: &TcpStream) {
    let states = &["Change Color", "Turn On", "Turn Off", "Status", "Exit"];
    status(&stream);

    loop {
        let turn_on_off = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&states[..])
            .interact()
            .unwrap();

        match turn_on_off {
            0 => {
                let red: u8 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Red")
                    .interact_text()
                    .unwrap();

                let green: u8 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Green")
                    .interact_text()
                    .unwrap();

                let blue: u8 = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Blue")
                    .interact_text()
                    .unwrap();

                change_color(&stream, red, green, blue, 0);
                println!("Updating to red {} green {} blue {}", red, green, blue);
            }

            1 => {
                println!("Turning the lights on");
                turn_on(&stream);
            }
            2 => {
                println!("Turning the lights off");
                turn_off(&stream);
            }

            3 => {
                status(&stream);
            }

            4 => exit(0),

            _ => panic!("invalid option (0, 1, 2, 3, 4)"),
        }
    }
}
