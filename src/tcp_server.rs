use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

// コマンドライン引数で指定されたソケットアドレスで接続を待ち受ける
pub fn serve(address: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(address)?; //変換できなかったら早期リターン
    loop {
        let (stream, _) = listener.accept()?;
        // スレッドを立ち上げて接続に対処する．
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        })
    }
}

// クライアントからの入力を待ち受け，受信したらそれを表示した後に同じものを返却する
fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("handling data from {}", stream.peer_addr()?);
    let mut buffer = [0u8; 1024];
    loop {
        let n_bytes = stream.read(&mut buffer)?;
        if n_bytes == 0 {
            debug!("Connection closed");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buffer[..n_bytes])?);
        stream.write_all(&buffer[..n_bytes])?;
    }
}