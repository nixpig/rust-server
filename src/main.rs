use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path,
};

use hello::ThreadPool;

const PUBLIC_PATH: &str = "public/";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down server...");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request_line_parts: Vec<&str> = request_line.split(" ").collect();

    let request_path = public(request_line_parts[1]);

    let status_line;
    let filename;

    match request_line_parts[1] {
        "/" => {
            (status_line, filename) = ("HTTP/1.1 OK", public("index.html"));
        }
        any => {
            if !path::Path::new(&public(any)).exists() {
                (status_line, filename) = ("HTTP/1.1 404 NOT FOUND", public("404.html"));
            } else {
                (status_line, filename) = ("HTTP/1.1 OK", public(any));
            }
        }
    }

    println!("{status_line}, {filename}");

    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn public(filepath: &str) -> String {
    format!("{PUBLIC_PATH}/{filepath}")
}
