use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router};
use axum::routing::get;

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

    let body = reqwest::get("https://news.ycombinator.com")
        .await.unwrap().text().await.unwrap();
    Html(format!("{body}"))
}
