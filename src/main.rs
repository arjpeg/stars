pub mod request;
pub mod route;
pub mod server;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use route::{Method, MethodKind};
use server::Server;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     println!("Connection established!");

    //     handle_connection(stream);
    // }
    let mut server: Server = "127.0.0.1:8080"
        .try_into()
        .expect("Failed to create server instance");

    server.register_route(route!(GET, "/", || {
        println!("Hello, world!");
    }));

    server.run();
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();
//     let mut request_line = request_line.split_whitespace();

//     let verb = request_line.nth(0).unwrap();
//     let path = request_line.nth(1).unwrap();

//     let (status_line, filename) = if verb == "GET" {
//         match path {
//             "/" => ("HTTP/1.1 200 OK", "index.html"),

//             _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
//         }
//     } else {
//         ("HTTP/1.1 404 NOT FOUND", "404.html")
//     };

//     let content = fs::read_to_string(filename).unwrap();
//     let length = content.len();

//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

//     stream.write(response.as_bytes()).unwrap();
// }
