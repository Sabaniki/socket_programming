mod app;
mod arg_types;

use app::create_app;
use arg_types::{Protocol, StartAs};
use std::process::exit;
use std::io::SeekFrom::Start;

fn main() {
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
            eprintln!("required argument is not specified.")
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
            eprintln!("required argument is not specified.")
        }
    };
}
