mod utils;
use core::panic;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::net::TcpStream;
use std::process::exit;
use utils::control::{change_color, turn_off, turn_on};
use utils::discovery::discover;
use utils::light::{connect, status};

fn main() {
    let devices = discover(1);

    if devices.is_empty() {
        println!("No devices was found, please ensure you are on the correct network");
        exit(0)
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your device")
        .default(0)
        .items(&devices[..])
        .interact()
        .unwrap();

    let stream = connect(&devices.get(selection).unwrap().address, 5577);

    _menu(&stream);
}

fn _menu(stream: &TcpStream) {
    let states = &["Change Color", "Turn On", "Turn Off", "Status", "Exit"];

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
