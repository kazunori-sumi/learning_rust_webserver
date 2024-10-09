use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("bind tcp listener!!!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("TcpListener is bound to the localhost:8080");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
