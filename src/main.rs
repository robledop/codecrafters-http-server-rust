use std::io::{BufRead, Read, Write};
#[allow(unused_imports)]
use std::net::TcpListener;
use std::sync::Mutex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                // let mut read_buffer = String::new();
                // stream.read_to_string(&mut read_buffer).expect("read failed");
                // 
                // println!("RECEIVED: {}", read_buffer);
                // 
                // let lines: Vec<&str> = read_buffer.split("\r\n").collect();
                // 
                // let first_line = lines[0].split(" ").collect::<Vec<&str>>();
                // let (method, path) = (first_line[0], first_line[1]);
                // 
                // println!("method: {}, path: {}", method, path);

                stream
                    .write("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                    .expect("Failed to write to server");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
