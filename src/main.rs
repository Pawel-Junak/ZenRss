use std::net::SocketAddr;
use askama::Template;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path="items.html")]
struct ItemsTemplate<'a> {
    items: &'a Vec<i32>
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hn",
        get(handler_get_hn)
    ).nest_service(
        "/",
        ServeDir::new("templates")
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
    let output = ItemsTemplate {items: &items};
    output.render().unwrap()
}
