use std::env;
use std::net::SocketAddr;
use askama::Template;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use libsql::Builder;
use rss::Channel;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "items.html")]
struct ItemsTemplate<'a> {
    items: &'a Vec<String>,
}

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hn",
        get(handler_get_hn),
    ).nest_service(
        "/",
        ServeDir::new("templates"),
    );

    db_connect().await;

    // conn.execute("INSERT INTO users (name) VALUES (\"Iku\");", ()).await.unwrap();
    // conn.execute("INSERT INTO users (name) VALUES (\"Iku2\");", ()).await.unwrap();


    // --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
        .await
        .unwrap();
}

async fn db_connect() {

    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (ID INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);", ()).await.unwrap();
}

async fn handler_get_hn() -> impl IntoResponse {
    println!("->> {:<12} - handler get hn", "HANDLER");
    let items: Vec<i32> = reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json").await.unwrap().json().await.unwrap();
    let rss = reqwest::get("https://hnrss.org/frontpage").await.unwrap().bytes().await.unwrap();
    let channel = Channel::read_from(&rss[..]);
    let first_30: Vec<String> = channel.unwrap().items[0..10].iter().map(|item| item.clone().title.unwrap()).collect();

    // match channel.unwrap().items.to_vec().first() {
    //     Some(x) => println!("{}", x.clone().title.unwrap()),
    //     None => todo!(),
    // }
    let output = ItemsTemplate { items: &first_30 };
    output.render().unwrap()
}
