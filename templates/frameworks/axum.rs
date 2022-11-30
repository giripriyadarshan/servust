use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;

use crate::db::pool;
mod db;

async fn hello_world() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
    let pool = pool().await;

    let app = Router::new().route("/", get(hello_world)).with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
