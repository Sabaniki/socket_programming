mod app;
mod arg_types;
mod tcp_server;

use app::create_app;
use arg_types::{Protocol, StartAs, get_args};

fn main() {
    let (protocol_type, start_as, unchecked_address) = get_args();


    match protocol_type {
        Protocol::Tcp => {
            match start_as {
                StartAs::Server => {
                    // TODO: TCPサーバの呼び出し
                },
                StartAs::Client => {
                    // TODO: TCPクライアントの呼び出し
                }
            }
        },
        Protocol::Udp => {
            match start_as {
                StartAs::Server => {
                    // TODO: UDPサーバの呼び出し
                },
                StartAs::Client => {
                    // TODO: UDPクラアンとの呼び出し
                }
            }
        }
    }
}
