// To run this project:
//    cargo run --bin http-server-hello-world
//
// To test this server, access it at:
//    http://localhost:3000
//
// Created based on https://github.com/programatik29/axum-tutorial/blob/master/tutorial/03-hello-world.md

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = 3000;

    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Starting server at port {}", port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, world!"
}
