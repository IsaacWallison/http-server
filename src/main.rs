use server::Server;
use handler::HTTPHandler;
use std::{default, env};

mod server;
mod http;
mod handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(HTTPHandler::new(public_path));
}
