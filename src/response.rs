use std::{collections::HashMap, fmt::Display};

/// Helper macro to render an html file.
#[macro_export]
macro_rules! render {
    ($path:literal) => {{
        let content = include_str!($path);

        $crate::response::Response {
            status_code: $crate::response::StatusCode::OK,
            headers: {
                let mut headers = HashMap::new();

                headers.insert("Content-Type".to_string(), "text/html".to_string());

                headers
            },
            body: $crate::response::ResponseBody::Text(content.to_string()),
        }
    }};
}

/// Represents an HTTP response from the server.
/// Contains the status code, headers, and body.
pub struct Response {
    /// The status code of the response.
    pub status_code: StatusCode,
    /// Any headers that should be sent with the response.
    pub headers: HashMap<String, String>,
    /// The body of the response.
    pub body: ResponseBody,
}

impl Response {
    /// Creates a new response.
    pub fn new(
        status_code: StatusCode,
        headers: HashMap<String, String>,
        body: ResponseBody,
    ) -> Self {
        Self {
            status_code,
            headers,
            body,
        }
    }
}

/// Represents an HTTP status code.
#[derive(Debug, Clone, PartialEq)]
pub enum StatusCode {
    OK,
    NotFound,
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404",
        };

        write!(f, "{}", code)
    }
}

/// Represents the body of an HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseBody {
    Text(String),
}

impl From<String> for ResponseBody {
    fn from(body: String) -> Self {
        Self::Text(body)
    }
}

impl From<ResponseBody> for String {
    fn from(body: ResponseBody) -> Self {
        match body {
            ResponseBody::Text(text) => text,
        }
    }
}
