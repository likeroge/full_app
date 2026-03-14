use sqlx::{Pool, Sqlite, SqlitePool};

pub type AppDatabase = Pool<Sqlite>;

pub async fn create_pool() -> Result<AppDatabase, sqlx::Error> {
    let database_url = "sqlite:file:data/database.db";
    SqlitePool::connect(database_url).await
}
