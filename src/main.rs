#[allow(unused_imports)]
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::string::ToString;

const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const CREATED: &str = "HTTP/1.1 201 Created\r\n\r\n";
const OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const METHOD_NOT_ALLOWED: &str = "HTTP/1.1 405 Method not allowed\r\n\r\n";

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let mut directory: String = "".to_string();
    let mut i = 0;
    for arg in args.iter() {
        if arg == "--directory" && args.len() > i + 1 {
            directory = args[i + 1].clone();
        }

        i += 1;
    }

    if directory != "" {
        println!("Using directory: {}", directory);
    }

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("accepted new connection");

            let dir = directory.clone();

            std::thread::spawn(|| handle_request(stream, dir));
        } else {
            eprintln!("error: {}", stream.unwrap_err());
        }
    }
}

fn handle_request(mut stream: TcpStream, directory: String) {
    let mut buffer_reader = BufReader::new(&mut stream);
    let mut request_line = String::new();
    buffer_reader.read_line(&mut request_line).unwrap();
    let parts: Vec<&str> = request_line.split(' ').collect();
    let (method, path) = (parts[0], parts[1]);

    let mut headers = Vec::new();

    loop {
        let mut line = String::new();
        buffer_reader.read_line(&mut line).unwrap();
        let line = line.trim_end();
        if line.is_empty() {
            break; // If the line is empty, it means this is the end of the headers
        }

        headers.push(line.to_string());
    }

    let response = match method {
        "GET" => match path {
            "/" => OK.to_string(),
            p if p.starts_with("/echo/") => {
                let message = &path["/echo/".len()..].trim().to_string();

                let accept_encoding_line = headers
                    .iter()
                    .find(|&x| x.to_lowercase().starts_with("accept-encoding: "));

                if let Some(accept_encoding_line) = accept_encoding_line {
                    let accept_encoding: Vec<&str> = accept_encoding_line
                        ["accept-encoding: ".len()..]
                        .trim()
                        .split(';')
                        .collect();

                    if accept_encoding.contains(&"gzip") {
                        format!(
                            "HTTP/1.1 200 OK\r\n\
                            Content-Type: text/plain\r\n\
                            Content-Encoding: gzip\r\n\
                            Content-Length: {}\r\n\r\n{}",
                            message.len(),
                            message
                        )
                    } else {
                        format!(
                            "HTTP/1.1 200 OK\r\n\
                            Content-Type: text/plain\r\n\
                            Content-Length: {}\r\n\r\n{}",
                            message.len(),
                            message
                        )
                    }
                } else {
                    format!(
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: text/plain\r\n\
                        Content-Length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    )
                }
            }
            "/user-agent" => {
                let user_agent_line = headers
                    .iter()
                    .find(|&x| x.to_lowercase().starts_with("user-agent: "))
                    .unwrap();
                let user_agent = &user_agent_line["user-agent: ".len()..].trim().to_string();

                format!(
                    "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: {}\
                    \r\n\r\n\
                    {}",
                    user_agent.len(),
                    user_agent
                )
            }
            p if p.starts_with("/files/") => {
                let file_name = &path["/files/".len()..].trim().to_string();
                let file_path = format!("{}{}", directory, file_name);

                let file_content = fs::read_to_string(&file_path);

                match file_content {
                    Ok(file) => format!(
                        "HTTP/1.1 200 OK\r\n\
                        Content-Type: application/octet-stream\r\n\
                        Content-Length: {}\r\n\r\n{}",
                        file.len(),
                        file
                    ),
                    Err(_) => NOT_FOUND.to_string(),
                }
            }
            _ => NOT_FOUND.to_string(),
        },
        "POST" => match path {
            p if p.starts_with("/files/") => {
                let file_name = &path["/files/".len()..].trim().to_string();
                let file_path = format!("{}{}", directory, file_name);

                let content_length: usize = headers
                    .iter()
                    .find(|&x| x.to_lowercase().starts_with("content-length: "))
                    .unwrap()["Content-Length: ".len()..]
                    .trim()
                    .parse()
                    .unwrap();

                let mut body_buffer = vec![0; content_length];
                buffer_reader.read_exact(&mut body_buffer).unwrap();

                let mut file = File::create(&file_path).unwrap();
                file.write_all(&body_buffer).unwrap();

                CREATED.to_string()
            }
            _ => NOT_FOUND.to_string(),
        },
        _ => METHOD_NOT_ALLOWED.to_string(),
    };

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write to server");
}
