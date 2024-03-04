use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router};
use axum::routing::get;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/",
        get(handler_hello)
    );

    // --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

// --- Handler Hello
async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler hello", "HANDLER");

    let api: Vec<i32> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await.unwrap().json().await.unwrap();
    Html(format!("{:?}", api))
}
