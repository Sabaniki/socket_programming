mod app;
mod arg_types;
mod udp;
mod tcp;

use std::env;
use udp::{udp_client, udp_server};
use tcp::{tcp_client, tcp_server};

#[macro_use]
extern crate log;

use arg_types::{Protocol, StartAs, get_args};
use log::error;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let (protocol_type, start_as, unchecked_address) = get_args();
    let address_str = unchecked_address.as_str();

    match protocol_type {
        Protocol::Tcp => {
            match start_as {
                StartAs::Server => {
                    tcp_server::serve(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
                StartAs::Client => {
                    tcp_client::connect(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
            }
        }
        Protocol::Udp => {
            match start_as {
                StartAs::Server => {
                    udp_server::serve(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
                StartAs::Client => {
                    udp_client::communicate(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
            }
        }
    }
}
