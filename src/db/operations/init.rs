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
        "CREATE TABLE IF NOT EXISTS schema_migrations (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, applied_at DATETIME DEFAULT CURRENT_TIMESTAMP)"
    ).execute(pool).await?;

    let migration_dir = "migrations";
    let mut entries = fs::read_dir(migration_dir)
        .map(|rd| rd.filter_map(|e| e.ok()).collect::<Vec<_>>())
        .unwrap_or_default();
    entries.sort_by_key(|e| e.path());
    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            let already_applied = sqlx::query_scalar::<_, i64>("SELECT COUNT(1) FROM schema_migrations WHERE name = ?")
                .bind(&name)
                .fetch_one(pool)
                .await?;
            if already_applied == 0 {
                let sql = fs::read_to_string(&path).expect("Erreur lecture migration");
                for statement in sql.split(";") {
                    let trimmed = statement.trim();
                    if !trimmed.is_empty() {
                        sqlx::query(trimmed).execute(pool).await?;
                    }
                }
                sqlx::query("INSERT INTO schema_migrations (name) VALUES (?)")
                    .bind(&name)
                    .execute(pool)
                    .await?;
                println!("Migration appliquée : {}", name);
            }
        }
    }
    Ok(())
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
