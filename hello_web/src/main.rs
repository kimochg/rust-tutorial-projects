use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs;
use std::thread;
use std::time::Duration;
use hello_web::ThreadPool;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8877").unwrap();
    // defined in lib.rs
    let pool = ThreadPool::new(4);

    println!("listening at {}", listener.local_addr().unwrap());

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down server.")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
