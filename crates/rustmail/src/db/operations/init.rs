use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
};
use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "db/db.sqlite";

    fs::create_dir_all("db")?;

    if !Path::new(db_path).exists() {
        fs::File::create(db_path)?;
        println!("Database file created at: {}", db_path);
    }

    let connect_options = SqliteConnectOptions::from_str(&format!("sqlite://{}", db_path))?
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(30)
        .connect_with(connect_options)
        .await?;

    sqlx::migrate!("../../migrations").run(&pool).await?;

    println!("Database connection pool established");
    Ok(pool)
}

pub async fn get_system_metadata(
    key: &str,
    pool: &SqlitePool,
) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query_scalar("SELECT value FROM system_metadata WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;

    Ok(result)
}

pub async fn set_system_metadata(
    key: &str,
    value: &str,
    pool: &SqlitePool,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO system_metadata (key, value, updated_at) 
        VALUES (?, ?, datetime('now'))
        "#,
    )
    .bind(key)
    .bind(value)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_last_recovery_timestamp(pool: &SqlitePool) -> Result<Option<String>, sqlx::Error> {
    get_system_metadata("last_recovery_timestamp", pool).await
}

pub async fn update_last_recovery_timestamp(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    set_system_metadata(
        "last_recovery_timestamp",
        &chrono::Utc::now().to_rfc3339(),
        pool,
    )
    .await
}
