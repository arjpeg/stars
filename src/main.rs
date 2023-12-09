pub mod request;
pub mod response;
pub mod route;
pub mod server;

use server::Server;

fn main() {
    let mut server: Server = "127.0.0.1:8080"
        .try_into()
        .expect("Failed to create server instance");

    server.register_route(route!(GET, "/", |req| {
        println!("{:?}", req.headers);
        render!("../index.html")
    }));

    server.run();
}
