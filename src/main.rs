#![allow(unused_imports)]
use bytes::Bytes;
use std::io::{BufRead, BufReader, Read, Write};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            loop {
                let bytes_read = socket.read(&mut buf).await.unwrap();
                if bytes_read == 0 {
                    break;
                }
                socket.write_all("+PONG\r\n".as_bytes()).await.unwrap();
            }
        });
    }
}
