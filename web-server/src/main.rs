use std::{net::{TcpListener, TcpStream}, thread, time::Duration};
use std::fs;
use std::io::prelude::*;

use web_server::ThreadPool;

pub fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool  = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream)
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    let _ = stream.read(&mut buffer);

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));

        ("200 OK", "hello.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status_line, contents);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}