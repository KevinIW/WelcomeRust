use std::{
    
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

use hello::ThreadPool;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() { 
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    }


fn handle_connection(mut stream: TcpStream) { 
    let buf_reader = BufReader::new(&mut stream); 
    let http_request: Vec<_> = buf_reader
    .lines() .map(|result| result.unwrap()) 
    .take_while(|line| !line.is_empty())
    .collect();


    // Parse the HTTP request method and path
    let (request_method, path) = if let Some(first_line) = http_request.first() {
        let mut parts = first_line.split_whitespace();
        let method = parts.next();
        let path = parts.next().unwrap_or("/");
        (method, path)
    } else {
        (None, "/")
    };

    let status_line;
    let contents;

    // Check if the request method is GET and if the path is "/hello" or "/sleep"
    if request_method == Some("GET") && (path == "/hello" || path == "/sleep") {
        status_line = "HTTP/1.1 200 OK";
        // Add a 5-second delay if the path is "/sleep"
        if path == "/sleep" {
            thread::sleep(Duration::from_secs(5));
        }
        contents = fs::read_to_string("src/templates/hello.html").unwrap_or_else(|_| {
            String::from("<html><body><h1>404 Not Found</h1></body></html>")
        });
    } else {
        status_line = "HTTP/1.1 400 Bad Request";
        contents = fs::read_to_string("src/templates/bad.html").unwrap_or_else(|_| {
            String::from("<html><body><h1>400 Bad Request</h1></body></html>")
        });
    }
    let length = contents.len();
    let response =
    format!("{status_line}\r\nContent-Length:{length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
   }
