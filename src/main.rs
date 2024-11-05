#![allow(unused_imports)]
use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                loop {
                    let mut buf = [0; 256];
                    let read_count = stream.read(&mut buf).unwrap();
                    println!("Bytes leídos: {}", read_count);
                    if read_count == 0 {
                        break;
                    }
                    let _ = stream.write(b"+PONG\r\n");    
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
