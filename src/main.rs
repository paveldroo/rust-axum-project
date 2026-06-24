#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{
    Router,
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
};

use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

mod error;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(routes_hello()).merge(routes_static());

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    println!("--> LISTENING on 127.0.0.1:8080\n");
    axum::serve(listener, app).await.unwrap()
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{:name}", get(handler_hello2))
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
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

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2", "HANDLER");

    Html(format!("Hello2 <strong>{name:?}</strong>"))
}
