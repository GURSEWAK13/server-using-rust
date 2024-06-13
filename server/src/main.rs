use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
    let listener=TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming(){
        let _stream=stream.unwrap();
        handle_connection(_stream);
    }
}

fn handle_connection(mut stream:TcpStream ){
    let mut buffer=[0;1024];
    stream.read(&mut buffer).unwrap();
    // println!(
    //     "Request: {}",String::from_utf8_lossy(&buffer[..])
    // )
    let get =b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get){
        let content=fs::read_to_string("index.html").unwrap();
        // let response ="HTTPS/1.1 200 OK\r\n\r\n";
        let response=format!(
            "HTTPS/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            content.len(),
            content
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }else {
        let status_line="HTTPS/1.1 404 NOT FOUND";
        let contents=fs::read_to_string("404.html").unwrap();
        // let response ="HTTPS/1.1 200 OK\r\n\r\n";
        let response=format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents,
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    
}