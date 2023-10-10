use salvo::{prelude::*, affix};
use crate::db::pool;

mod db;

#[handler]
async fn hello_world() -> &'static str {
    "Hello world"
}
#[tokio::main]
async fn main() {
    let pool = pool().await;
    let router = Router::new().hoop(affix::inject(pool)).get(hello_world);
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
