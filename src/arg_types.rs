use std::fmt;
use std::fmt::Formatter;
use std::intrinsics::write_bytes;

pub enum Protocol {
    Tcp,
    Udp
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Protocol::Tcp => write!("tcp"),
            Protocol::Udp => write!("udp"),
        }
    }
}

pub enum StartAs {
    Server,
    Client
}

impl fmt::Display for StartAs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            StartAs::Server => write!("server"),
            StartAs::Client => write!("client"),
        }
    }
}

trait ArgType{}

impl ArgType for Protocol{}
impl ArgType for StartAs{}
