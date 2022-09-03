use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

pub fn pool() -> AppState {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AppState {
        connection: DatabaseConnection::new(&db_url),
    }
}
