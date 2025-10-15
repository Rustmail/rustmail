use crate::BotState;
use axum::extract::State;
use axum::response::Redirect;
use std::sync::Arc;
use axum_extra::extract::CookieJar;
use sqlx::{query, Row};
use tokio::sync::Mutex;

pub async fn handle_login(
    jar: CookieJar,
    State(bot_state): State<Arc<Mutex<BotState>>>) -> Redirect {
    let state_lock = bot_state.lock().await;

    let session_cookie = jar.get("session_id");
    let user_id = jar.get("user_id");

    if let (Some(session_cookie), Some(user_id)) = (session_cookie, user_id) {
        let session_id = session_cookie.value();
        let user_id = user_id.value();

        if let Some(db_pool) = &state_lock.db_pool {
            if let Ok(row) = query("SELECT expires_at FROM sessions_panel WHERE session_id = ? AND user_id = ?")
                .bind(session_id)
                .bind(user_id)
                .fetch_one(db_pool)
                .await
            {
                let expires_at: i64 = row.get::<i64, _>("expires_at");
                let current_timestamp = chrono::Utc::now().timestamp();

                if expires_at > current_timestamp {
                    return Redirect::to("/panel");
                }
            }
        }
    }

    let Some(config) = &state_lock.config else {
        return Redirect::to("/error?message=Missing+configuration");
    };

    let client_id = config.bot.client_id;

    let url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify%20guilds",
        client_id,
        urlencoding::encode("http://localhost:3002/api/auth/callback")
    );

    Redirect::to(&url)
}
