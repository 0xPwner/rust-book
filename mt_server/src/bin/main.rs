use mt_server::ThreadPool;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer).to_string();
    println!("{}", request);
    let get_method = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get_method) {
        ("HTTP/1.1 200 OK", "src/html/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/html/404.html")
    };
    let content = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{} \r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        content.len(),
        content
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
