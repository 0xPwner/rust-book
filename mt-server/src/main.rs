use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(s) => handle_connection(s),
            Err(e) => println!("{}", e.to_string())
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer).to_string();
    println!("{}", request);
    let get_method = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get_method) {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };
        let content = fs::read_to_string(filename).unwrap();
        let response = format!(
            "{} \r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            content.len(),
            content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
}
