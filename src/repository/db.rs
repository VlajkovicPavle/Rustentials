use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
use std::result::Result;
include!("./queries/schema.rs");

const DATABASE_URL: &str = "sqlite://rustentials.db";

pub async fn create_database(db_url: &str) -> bool {
    Sqlite::create_database(db_url).await.unwrap();
    match create_schema(db_url).await {
        Ok(_) => {
            println!("Database successfuly created!");
            true
        }
        Err(_) => false,
    }
}
async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(db_url).await?;
    let db_schema: &str = &fetch_schema_query();
    let result = sqlx::query(db_schema).execute(&pool).await;
    pool.close().await;
    result
}

pub async fn fetch_db_instances() -> Result<SqlitePool, sqlx::Error> {
    if !Sqlite::database_exists(DATABASE_URL).await.unwrap_or(false)
        && !create_database(DATABASE_URL).await
    {
        panic!("Failed to create database");
    }
    let instances = SqlitePool::connect(DATABASE_URL).await.unwrap();
    Ok(instances)
}
