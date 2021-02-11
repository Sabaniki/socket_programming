mod app;
mod arg_types;
mod tcp_server;
mod tcp_client;

use std::env;

#[macro_use]
extern crate log;

use arg_types::{Protocol, StartAs, get_args};
use log::error;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let (protocol_type, start_as, unchecked_address) = get_args();

    match protocol_type {
        Protocol::Tcp => {
            match start_as {
                StartAs::Server => {
                    tcp_server::serve(unchecked_address.as_str())
                        .unwrap_or_else(|error| error!("{}", error));
                }
                StartAs::Client => {
                    tcp_client::connect(unchecked_address.as_str())
                        .unwrap_or_else(|error| error!("{}", error));
                }
            }
        }
        Protocol::Udp => {
            match start_as {
                StartAs::Server => {
                    // TODO: UDPサーバの呼び出し
                }
                StartAs::Client => {
                    // TODO: UDPクラアンとの呼び出し
                }
            }
        }
    }
}
