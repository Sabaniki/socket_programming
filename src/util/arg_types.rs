use crate::app::create_app;
use std::process::exit;
use log::error;

pub enum Protocol {
    Tcp,
    Udp,
}

pub enum StartAs {
    Server,
    Client,
}

pub fn get_args() -> (Protocol, StartAs, String) {
    let args = create_app();
    let protocol_type = match args.value_of("protocol_type") {
        Some(protocol) => {
            match protocol {
                "tcp" => Protocol::Tcp,
                "udp" => Protocol::Udp,
                _ => {
                    error!("specify tcp or udp. it's illegal argument.");
                    exit(1);
                }
            }
        }
        None => {
            panic!();
        }
    };
    let start_as = match args.value_of("start_as") {
        Some(start_as) => {
            match start_as {
                "server" => StartAs::Server,
                "client" => StartAs::Client,
                _ => {
                    error!("specify server or client. it's illegal argument.");
                    exit(1);
                }
            }
        }
        None => {
            panic!();
        }
    };
    let unchecked_address = match args.value_of("address_and_port") {
        Some(addr) => addr.to_string(),
        None => panic!()
    };
    (protocol_type, start_as, unchecked_address)
}