use std::io::prelude::*;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::{env, fs, thread};

enum Response {
    BadRequest,
    NotFound,
    Ok,
}

fn main() {
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + &port).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let mut status = Response::BadRequest;
    let dir = "files/";

    let s = std::str::from_utf8(&buffer).unwrap();
    let chunks: Vec<_> = s.split_ascii_whitespace().collect();

    let mut filename = "";
    if chunks.len() > 2 && chunks[0] == "GET" {
        status = Response::NotFound;

        let _method = chunks[0];
        let mut path = chunks[1];
        if path == "/" {
            path = "index.html"
        }

        filename = Path::new(path).file_name().unwrap().to_str().unwrap();
        if Path::new((dir.to_owned() + filename).as_str()).exists() {
            status = Response::Ok
        }
    }

    let (status_line, filename) = match status {
        Response::BadRequest => ("400 Bad Request", "400.html"),
        Response::NotFound => ("404 Not Found", "404.html"),
        Response::Ok => ("200 OK", filename),
    };

    let contents = fs::read_to_string(dir.to_owned() + filename).unwrap();
    let response = format!("HTTP/1.1 {}\r\n\r\n{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Both).unwrap();
}
