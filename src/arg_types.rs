use std::fmt;
use std::fmt::Formatter;
use std::intrinsics::write_bytes;
use crate::app::create_app;
use std::process::exit;

pub enum Protocol {
    Tcp,
    Udp
}

pub enum StartAs {
    Server,
    Client
}

pub fn get_args() -> (Protocol, StartAs) {
    let args = create_app();
    let protocol_type = match args.value_of("protocol_type") {
        Some(protocol) => {
            match protocol {
                "tcp" => Protocol::Tcp,
                "udp" => Protocol::Udp,
                _ => {
                    eprintln!("specify tcp or udp. it's illegal argument.");
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
                    eprintln!("specify server or client. it's illegal argument.");
                    exit(1);
                }
            }
        }
        None => {
            panic!();
        }
    };
    (protocol_type, start_as)
}