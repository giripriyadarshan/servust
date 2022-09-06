use sea_orm::DatabaseConnection;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub connection: DatabaseConnection,
}

async fn connection_pool() -> AppState {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = sea_orm::Database::connect(&db_url).await.unwrap();
    AppState { connection }
}

pub fn pool() -> AppState {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(connection_pool())
}
