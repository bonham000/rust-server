use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {

    println!("Initializing...");

    let binding = TcpListener::bind("127.0.0.1:7878");

    match binding {
        Err(_) => println!("Could not bind to port, is it already in use?"),
        Ok(listener) => handle_connection(listener),
    }
}

fn handle_connection(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connected!");

        read_stream(stream);
    }
}

fn read_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}