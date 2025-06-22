use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::path::Path;
use std::fs;

pub async fn init_database() -> Result<SqlitePool, sqlx::Error> {
    let db_path = "./db/db.sqlite";

    fs::create_dir_all("./db")?;

    if !Path::new(db_path).exists() {
        fs::File::create(db_path)?;
        println!("Database file created at: {}", db_path);
    }

    let db_url = format!("sqlite://{}", db_path);
    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    apply_migrations(&pool).await?;

    println!("Database connection pool established");
    Ok(pool)
}

async fn apply_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS threads (
            id TEXT PRIMARY KEY,
            user_id INTEGER NOT NULL,
            user_name TEXT NOT NULL,
            channel_id TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            next_message_number INTEGER DEFAULT 1,
            status INTEGER DEFAULT 1,
            user_left BOOLEAN DEFAULT FALSE
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS thread_messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            thread_id TEXT NOT NULL,
            user_id INTEGER NOT NULL,
            user_name TEXT NOT NULL,
            is_anonymous BOOLEAN DEFAULT FALSE,
            dm_message_id TEXT,
            inbox_message_id TEXT,
            message_number INTEGER,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            content TEXT NOT NULL,
            thread_status INTEGER DEFAULT 1,
            FOREIGN KEY (thread_id) REFERENCES threads(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS blocked_users (
            user_id TEXT PRIMARY KEY,
            user_name TEXT NOT NULL,
            blocked_by TEXT NOT NULL,
            blocked_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            expires_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS staff_alerts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            staff_user_id INTEGER NOT NULL,
            thread_user_id INTEGER NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            used BOOLEAN DEFAULT FALSE
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS system_metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO system_metadata (key, value) 
        VALUES ('last_recovery_timestamp', datetime('now'))
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_system_metadata(key: &str, pool: &SqlitePool) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query_scalar("SELECT value FROM system_metadata WHERE key = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;
    
    Ok(result)
}

pub async fn set_system_metadata(key: &str, value: &str, pool: &SqlitePool) -> Result<(), sqlx::Error> {
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
    set_system_metadata("last_recovery_timestamp", &chrono::Utc::now().to_rfc3339(), pool).await
}
