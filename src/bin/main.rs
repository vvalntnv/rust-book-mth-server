use multithread::ThreadPool; 
use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = 
        TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    if let Err(_) = stream.read(&mut buffer){
        println!("Oopsie")
    }

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";


    let contents = fs::read_to_string("index.html").unwrap();

    // Root request
    if buffer.starts_with(get) {
        let response = make_response(&contents, "200 OK");

        send_response(&mut stream, response.as_bytes())

    //Sleep for 5 sec
    } else if buffer.starts_with(sleep) {
        let response = make_response(&contents, "200 OK");

        thread::sleep(Duration::from_secs(10));
        
        send_response(&mut stream, response.as_bytes());
    // 404
    } else {
        let not_found_page = fs::read_to_string("404.html").unwrap();

        let response = make_response(&not_found_page, "404 NOT FOUND");

        send_response(&mut stream, response.as_bytes())
    }
}

fn make_response(contents: &str, status_str: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status_str,
        contents.len(),
        contents
    )
}

fn send_response(stream: &mut TcpStream, response: &[u8]) {
        stream.write_all(response).unwrap();
        stream.flush().unwrap();
} 
