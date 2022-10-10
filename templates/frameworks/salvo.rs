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
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
