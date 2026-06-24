#![allow(unused)]

use std::net::SocketAddr;

use axum::{
    Router,
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
};

use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/hello", get(handler_hello));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> LISTENING on 127.0.0.1:8080\n");
    axum::serve(listener, app).await.unwrap()
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name:?}</strong>"))
}
