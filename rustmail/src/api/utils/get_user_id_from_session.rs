use sqlx::{Row, SqlitePool, query};

pub async fn get_user_id_from_session(session_id: &str, db_pool: &SqlitePool) -> String {
    let result = query("SELECT user_id FROM sessions_panel WHERE session_id = ?")
        .bind(session_id)
        .fetch_one(db_pool)
        .await;

    match result {
        Ok(row) => {
            let user_id: String = row.get::<String, _>("user_id");
            user_id
        }
        Err(_) => "".to_string(),
    }
}
