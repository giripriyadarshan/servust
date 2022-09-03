use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use crate::db::pool;

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool()))
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}