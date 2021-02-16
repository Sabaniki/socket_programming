mod udp;
mod tcp;
mod util;

use std::env;
use util::{app, arg_types};

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
                    tcp::server::serve(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
                StartAs::Client => {
                    tcp::client::connect(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
            }
        }
        Protocol::Udp => {
            match start_as {
                StartAs::Server => {
                    udp::server::serve(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
                StartAs::Client => {
                    udp::client::communicate(address_str)
                        .unwrap_or_else(|error| error!("{}", error));
                }
            }
        }
    }
}
