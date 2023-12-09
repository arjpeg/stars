use stars::server::Server;
use stars::{render, render_json, route};

fn main() {
    let mut server: Server = "127.0.0.1:8080"
        .try_into()
        .expect("Failed to create server instance");

    server.register_not_found(route!(GET, "/404", |_| {
        render!("./templates/404.html", 404)
    }));

    server.register_route(route!(GET, "/", |_| {
        render!("./templates/index.html")
    }));

    server.register_route(route!(GET, "/about", |_| {
        use std::collections::HashMap;

        let mut data = HashMap::new();
        data.insert("name", "Rust");
        data.insert("version", "1.0.0");
        data.insert("author", "Rustaceans");

        render_json!(data)
    }));

    server.run();
}
