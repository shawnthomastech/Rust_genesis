use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        thisisafunction(10.0);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        //blah
    }
}

// TODAY I ADDED A FILE TO THE CODEBASE TO ACTUALLY TRY TO GET A VISUAL ON THE DOCUMENT

fn thisisafunction(value: f32) {
    let taxes_cost = 0.03;
    let result = value * taxes_cost;
    println!("{}",result);
}

//just to seal things with an update 

// Today I need to make sure that the stylesheet I am using is actually working on the front-end of the app


