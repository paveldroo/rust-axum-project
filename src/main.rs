#![allow(unused)]

use std::net::SocketAddr;

use axum::{Router, response::Html, routing::get};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/hello",
        get(|| async { Html("<strong>Hello world!</strong>") }),
    );

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> LISTENING on 127.0.0.1:8080\n");
    axum::serve(listener, app).await.unwrap()
}
