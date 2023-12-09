use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    render,
    request::Request,
    response::{Response, ResponseBody, StatusCode},
    route::{Method, Route},
};

/// A struct that contains the server tcp listener and the routes.
pub struct Server {
    listener: TcpListener,
    routes: Vec<Route>,
}

impl Server {
    /// Creates a new server instance.
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            routes: Vec::new(),
        }
    }

    /// Registers a new route.
    pub fn register_route(&mut self, route: Route) {
        self.routes.push(route);
    }

    /// Starts the server.
    pub fn run(&mut self) {
        println!("Server listening...");

        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");

            self.handle_connection(stream);
        }
    }

    /// Handles a connection.
    fn handle_connection(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let full_request = buf_reader
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();

        // let content = "deez nuts lmao 12353";
        // let length = content.len();
        // let status_line = "HTTP/1.1 200 OK";

        // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

        // println!("{}", response);

        // stream.write_all(response.as_bytes()).unwrap();

        // dbg!(&full_request);

        let request = Request::new(full_request);
        let route = self
            .routes
            .iter()
            .find(|route| route.method == request.method);

        let response: Response = match route {
            Some(route) => {
                (route.handler)(request);
                todo!();
            }
            None => {
                println!("No route found!");
                render!("../404.html")
            }
        };

        let content: String = response.body.into();
        let length = content.len();

        // let response = format!(
        //     "HTTP/1.1 {}\r\nContent-Length: {length}\r\n\r\n{content}",
        //     response.status_code,
        // );
        let mut response_str = String::new();

        response_str.push_str(&format!("HTTP/1.1 {}\r\n", response.status_code));
        response_str.push_str(&format!("Content-Length: {}\r\n", length));

        for (name, value) in response.headers {
            response_str.push_str(&format!("{}: {}\r\n", name, value));
        }

        response_str.push_str("\r\n");
        response_str.push_str(&content);

        stream.write_all(response_str.as_bytes()).unwrap();

        // let content = "deez nuts lmao";
        // let length = content.len();
        // let status_line = "HTTP/1.1 200 OK";

        // let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

        // println!("{}", response);

        // stream.write_all(response.as_bytes()).unwrap();

        // &self
        //     .routes
        //     .iter()
        //     .find(|route| Method::from((verb, path)).unwrap() == route.method);

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
    }
}

impl From<TcpListener> for Server {
    fn from(listener: TcpListener) -> Self {
        Self::new(listener)
    }
}

impl TryFrom<&str> for Server {
    type Error = std::io::Error;

    fn try_from(addr: &str) -> Result<Self, Self::Error> {
        let listener = TcpListener::bind(addr)?;

        Ok(Self::new(listener))
    }
}
