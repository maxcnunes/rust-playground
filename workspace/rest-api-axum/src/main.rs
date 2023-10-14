// A simple REST API using axum and sqlx.
//
// To run this project:
// 1. Bring Postgres DB up with `docker compose up`
// 2. Start the service with `cargo run`
//
// To test this server, access it at:
//    http://localhost:8000
//
// Created based on:
// https://carlosmv.hashnode.dev/creating-a-rest-api-with-axum-sqlx-rust
// https://github.com/tokio-rs/axum/tree/main/examples

use axum::{
    routing::{delete, get, post, put},
    Router,
};

use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::fs;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod errors;
mod models;
mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    assert_eq!(key, "DATABASE_URL");

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("tower_http=trace")
                .unwrap_or_else(|_| "rest_api_axum=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&database_url)
        .await
        .context("could not connect to database_url")?;

    let app = Router::new()
        .route("/hello", get(root))
        .route("/tasks", get(routes::tasks::get_tasks::handler))
        .route("/task", post(routes::tasks::create_task::handler))
        .route("/task/:id", get(routes::tasks::get_task::handler))
        .route("/task/:id", put(routes::tasks::update_task::handler))
        .route("/task/:id", delete(routes::tasks::delete_task::handler))
        .with_state(pool)
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
