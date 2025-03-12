#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let buffer_reader = BufReader::new(&mut stream);
                let first_line = buffer_reader
                    .lines()
                    .next()
                    .unwrap()
                    .expect("failed to read first line");

                let first_line_split = first_line.split(" ").collect::<Vec<&str>>();
                let (method, path) = (first_line_split[0], first_line_split[1]);

                let response;

                match method {
                    "GET" => {
                        response = match path {
                            "/" => "HTTP/1.1 200 OK\r\n\r\n",
                            _ => "HTTP/1.1 404 Not Found\r\n\r\n",
                        };
                    }
                    "POST" => {
                        response = "HTTP/1.1 200 OK\r\n\r\n";
                    }
                    _ => {
                        response = "HTTP/1.1 404 MethodNotAllowed\r\n\r\n";
                    }
                }

                stream
                    .write(response.as_bytes())
                    .expect("Failed to write to server");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
