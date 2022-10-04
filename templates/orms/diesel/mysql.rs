use diesel::{mysql::MysqlConnection, r2d2::ConnectionManager};

pub async fn pool() -> r2d2::Pool<ConnectionManager<MysqlConnection>> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
