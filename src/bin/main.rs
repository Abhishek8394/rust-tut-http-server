use http_server::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let pool = ThreadPool::new(4).unwrap();
    println!("Listening..");
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // println!("Request\n{}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK \r\n\r\n", "index.html")
    } else if buffer.starts_with(sleep) {
        println!("sleeping for 5.");
        thread::sleep(Duration::from_secs(5));
        println!("I'm up.");
        ("HTTP/1.1 200 OK \r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 400 Not found \r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
