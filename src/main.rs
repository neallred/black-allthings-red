extern crate tiny_http;
use tiny_http::{Server, Response};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_port = String::from("8080");
    let port = args.get(1).unwrap_or(&default_port);

    let page = include_bytes!("../index.html").to_vec();

    let server = Server::http(format!("0.0.0.0:{}", port)).unwrap();

    for request in server.incoming_requests() {
        let _ = request.respond(Response::from_data(page.clone()));
    }
}
