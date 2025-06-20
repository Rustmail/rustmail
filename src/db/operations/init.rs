use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;

pub async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "./db/db.sqlite";

    std::fs::create_dir_all("./db")?;

    if !Path::new(db_path).exists() {
        std::fs::File::create(db_path)?;
        println!("Database file created at: {}", db_path);
    }

    let db_url = format!("sqlite://{}", db_path);
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    println!("Database connection pool established");
    Ok(pool)
}

pub async fn _run_migrations(_pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("Database migrations completed (using Prisma schema)");
    Ok(())
}

pub async fn _health_check(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").execute(pool).await?;

    println!("Database health check passed");
    Ok(())
}

pub async fn _close_database(pool: SqlitePool) {
    pool.close().await;
    println!("Database connection pool closed");
}
