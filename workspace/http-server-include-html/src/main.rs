// To run this project:
//    cargo run --bin http-server-include-html
//
// To test this server, access it at:
//    http://localhost:3000
//
// Created based on https://github.com/programatik29/axum-tutorial/blob/master/tutorial/05-include-html.md

use axum::{response::Html, routing::get, Router};
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

async fn handler() -> Html<&'static str> {
    // `std::include_str` macro can be used to include an utf-8 file as `&'static str` in compile
    // time. This method is relative to current `main.rs` file.
    Html(include_str!("../index.html"))
}
