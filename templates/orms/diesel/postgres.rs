#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;

pub fn pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
