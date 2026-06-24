#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> LISTENING on 127.0.0.1:8080\n");
    axum::serve(listener, app).await.unwrap()
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello <strong>world!</strong>")
}
