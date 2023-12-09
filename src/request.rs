use std::collections::HashMap;

use crate::route::Method;

/// Contains general information about an
/// HTTP request, such as the method, path,
/// query string, headers, etc.

pub struct Request {
    pub method: Method,
    pub headers: HashMap<String, String>,
}

impl Request {
    pub fn new(request_lines: Vec<String>) -> Self {
        // request_lines.iter().for_each(|line| {
        //     println!("{}", line);
        // });

        let request_line = request_lines
            .first()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();

        let verb = *request_line.get(0).unwrap();
        let path = *request_line.get(1).unwrap();

        let method = Method::try_from((verb, path)).unwrap();

        let headers = request_lines
            .iter()
            .skip(1)
            .filter(|line| line.contains(":"))
            .map(|line| {
                let mut line = line.split(": ");

                let key = line.next().unwrap().to_string();
                let value = line.next().unwrap().to_string();

                (key, value)
            })
            .collect::<HashMap<_, _>>();

        println!("{:?}", method);

        Self { method, headers }
    }
}
