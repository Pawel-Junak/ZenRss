use std::net::SocketAddr;
use axum::response::{Html, IntoResponse};
use axum::{Router};
use axum::routing::get;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hn",
        get(handler_get_hn)
    ).nest_service(
        "/",
        ServeDir::new("dist")
    );

    // --- Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn handler_get_hn() -> impl IntoResponse {
    println!("->> {:<12} - handler get hn", "HANDLER");
    let items: Vec<i32> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await.unwrap().json().await.unwrap();

    // let mut output = String::from("");
    // api.iter().for_each( |item|
    //     output = format!("{} <b>{}</b><br>", output, item)
    // );
    Html(output)
}
