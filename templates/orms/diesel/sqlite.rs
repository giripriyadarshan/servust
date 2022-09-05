use diesel::{r2d2::ConnectionManager, sqlite::SqliteConnection};

pub fn pool() -> r2d2::Pool<ConnectionManager<SqliteConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
