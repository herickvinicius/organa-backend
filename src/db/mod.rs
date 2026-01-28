use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool(database_url: &str) -> PgPool {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create database pool");
    
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run database migrations");
    
    pool
}