use std::fmt::Debug;

/// Helper macro to create a new route.
#[macro_export]
macro_rules! route {
    ($method:ident, $path:literal, $handler:expr) => {{
        use $crate::route::{Method, MethodKind, Route};

        Route::new(
            Method {
                method: MethodKind::$method,
                path: $path.to_string(),
            },
            $handler,
        )
    }};
}

/// Represents an HTTP route.
/// Contains a path and a handler function.
pub struct Route {
    pub method: Method,
    pub handler: Box<dyn Fn() -> ()>,
}

impl Debug for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Route")
            .field("method", &self.method)
            .finish()
    }
}

impl Route {
    pub fn new(method: Method, handler: fn()) -> Route {
        Route {
            method,
            handler: Box::new(handler),
        }
    }
}

/// Represents an HTTP method. Contains the path and the method itself.
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub path: String,
    pub method: MethodKind,
}

/// Represents an HTTP method kind.
#[derive(Debug, Clone, PartialEq)]
pub enum MethodKind {
    GET,
}

impl TryFrom<(&str, &str)> for Method {
    type Error = ();

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let (verb, path) = value;

        let method = match verb {
            "GET" => MethodKind::GET,
            _ => return Err(()),
        };

        Ok(Method {
            path: path.to_string(),
            method,
        })
    }
}
