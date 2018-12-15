use std::io::prelude::*;
use std::fs;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {

    let port = "127.0.0.1:7878";

    println!("Initializing... running on port: {}", port);

    let binding = TcpListener::bind(port);

    match binding {
        Err(_) => println!("Could not bind to port, is it already in use?"),
        Ok(listener) => handle_connection(listener),
    }
}

fn handle_connection(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connected!");

        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get_index_request = b"GET / HTTP/1.1\r\n";

    let (status_line, file_path) = if buffer.starts_with(get_index_request) {
        ("HTTP/1.1 200 OK\r\n\r\n", "welcome.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(format!("client/{}", file_path)).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}