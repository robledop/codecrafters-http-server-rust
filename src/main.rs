#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("accepted new connection");

            let buffer_reader = BufReader::new(&mut stream);
            if let Some(Ok(first_line)) = buffer_reader.lines().next() {
                let parts: Vec<&str> = first_line.split(' ').collect();
                let (method, path) = (parts[0], parts[1]);

                let response = match method {
                    "GET" => match path {
                        "/" => "HTTP/1.1 200 OK\r\n\r\n",
                        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                    },
                    "POST" => "HTTP/1.1 404 Not Found\r\n\r\n",
                    _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\n",
                };

                stream.write_all(response.as_bytes()).expect("Failed to write to server");
            }
        } else {
            eprintln!("error: {}", stream.unwrap_err());
        }
    }
}
