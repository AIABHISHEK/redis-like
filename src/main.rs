#![allow(unused_imports)]
use std::{io::{BufRead, BufReader, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                _stream.write_all("+PONG\r\n".as_bytes()).unwrap();
                let reader = BufReader::new(_stream.try_clone().unwrap());
                for line in reader.lines() {
                    _stream.write_all("+PONG\r\n".as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
