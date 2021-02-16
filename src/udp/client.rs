use std::net::UdpSocket;
use std::{io, str};

pub fn communicate(address: &str) -> Result<(), failure::Error> {
    // 0番ポートを指定することでOSがよしなに降ってくれる
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), address)?;

        // 大きさは適当。1024より大きかったらその分は破棄される
        let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!(
            "{}",
            str::from_utf8(&buffer)
                .expect("failed to convert to String")
        );
    }
}