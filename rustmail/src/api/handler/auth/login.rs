use crate::BotState;
use crate::api::utils::get_user_id_from_session::get_user_id_from_session;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use sqlx::{Row, query};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_login(
    jar: CookieJar,
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    let redirect_after = params
        .get("redirect")
        .cloned()
        .unwrap_or_else(|| "/panel".to_string());

    let db_pool = {
        let state_lock = bot_state.lock().await;

        if let Some(pool) = &state_lock.db_pool {
            pool.clone()
        } else {
            return Redirect::to("/error?message=Database+not+initialized");
        }
    };

    let bot_config = {
        let state_lock = bot_state.lock().await;

        if let Some(config) = &state_lock.config {
            config.clone()
        } else {
            return Redirect::to("/error?message=Bot+not+configured");
        }
    };

    let session_cookie = jar.get("session_id");

    if let Some(session_cookie) = session_cookie {
        let session_id = session_cookie.value();
        let user_id = get_user_id_from_session(session_id, &db_pool).await;

        if let Ok(row) =
            query("SELECT expires_at FROM sessions_panel WHERE session_id = ? AND user_id = ?")
                .bind(session_id)
                .bind(user_id)
                .fetch_one(&db_pool)
                .await
        {
            let expires_at: i64 = row.get::<i64, _>("expires_at");
            let current_timestamp = chrono::Utc::now().timestamp();

            if expires_at > current_timestamp {
                return Redirect::to("/panel");
            }
        }
    }

    let url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify%20guilds&state={}",
        bot_config.bot.client_id,
        urlencoding::encode(bot_config.bot.redirect_url.as_str()),
        urlencoding::encode(&redirect_after),
    );

    Redirect::to(&url)
}
