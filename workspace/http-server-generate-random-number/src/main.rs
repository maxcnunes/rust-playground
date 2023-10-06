// To run this project:
//    cargo run --bin http-server-generate-random-number
//
// To test this server, access it at:
//    http://localhost:3000/?start=50&end=100
//
// Created based on https://github.com/programatik29/axum-tutorial/blob/master/tutorial/04-generate-random-number.md

use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
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

#[derive(Deserialize)]
struct RangeParams {
    start: usize,
    end: usize,
}

async fn handler(Query(range): Query<RangeParams>) -> Html<String> {
    let random_num = thread_rng().gen_range(range.start..range.end);

    Html(format!("<h1>Random number: {}</h1>", random_num))
}
