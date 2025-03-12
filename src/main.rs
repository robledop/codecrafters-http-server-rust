use std::fmt::format;
#[allow(unused_imports)]
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            println!("accepted new connection");

            let buffer_reader = BufReader::new(&mut stream);
            let mut buffer_lines = buffer_reader.lines();
            if let Some(Ok(first_line)) = buffer_lines.next() {
                let parts: Vec<&str> = first_line.split(' ').collect();
                let (method, path) = (parts[0], parts[1]);

                let response = match method {
                    "GET" => match path {
                        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                        p if p.starts_with("/echo/") => {
                            let message = &path[6..];
                            format!(
                                "HTTP/1.1 200 OK\r\n\
                                Content-Type: text/plain\r\n\
                                Content-Length: {}\r\n\r\n{}",
                                message.len(),
                                message
                            )
                        }
                        "/user-agent" => {
                            let user_agent_line = buffer_lines
                                .find(|x| x.as_ref().unwrap().starts_with("User-Agent: "))
                                .unwrap()
                                .expect("Invalid user-agent line");
                            let user_agent = &user_agent_line[12..];
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent)
                        }
                        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                    },
                    "POST" => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                    _ => "HTTP/1.1 405 Method Not Allowed\r\n\r\n".to_string(),
                };

                stream
                    .write_all(response.as_bytes())
                    .expect("Failed to write to server");
            }
        } else {
            eprintln!("error: {}", stream.unwrap_err());
        }
    }
}
