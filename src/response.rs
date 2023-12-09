use std::{collections::HashMap, fmt::Display};

/// Helper macro to render an html file.
#[macro_export]
macro_rules! render {
    ($path:literal) => {{
        render!($path, $crate::response::StatusCode::OK)
    }};

    ($path:literal, $status_code:expr) => {{
        let content = include_str!($path);
        use std::collections::HashMap;

        let status_code: $crate::response::StatusCode = $status_code.into();

        $crate::response::Response {
            status_code,
            headers: {
                let mut headers = HashMap::new();

                headers.insert("Content-Type".to_string(), "text/html".to_string());

                headers
            },
            body: $crate::response::ResponseBody::Text(content.to_string()),
        }
    }};
}

/// Helper macro to render a json response.
#[macro_export]
macro_rules! render_json {
    ($json:expr) => {{
        // Use the json crate to serialize the json
        let json = serde_json::to_string(&$json).unwrap();

        $crate::response::Response {
            status_code: $crate::response::StatusCode::OK,
            headers: {
                let mut headers = HashMap::new();

                headers.insert("Content-Type".to_string(), "application/json".to_string());

                headers
            },
            body: $crate::response::ResponseBody::Json(json),
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

impl From<usize> for StatusCode {
    fn from(code: usize) -> Self {
        match code {
            200 => Self::OK,
            404 => Self::NotFound,
            _ => Self::NotFound,
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = match self {
            StatusCode::OK => "200 OK",
            StatusCode::NotFound => "404 NOT FOUND",
        };

        write!(f, "{}", code)
    }
}

/// Represents the body of an HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub enum ResponseBody {
    Text(String),
    Json(String),
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
            ResponseBody::Json(json) => json,
        }
    }
}
