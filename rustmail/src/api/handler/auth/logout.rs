use crate::BotState;
use axum::extract::State;
use axum::response::Redirect;
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_logout(
    jar: CookieJar,
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> Redirect {
    let state_lock = bot_state.lock().await;

    let session_cookie = jar.get("session_id");
    let user_id = jar.get("user_id");

    if let (Some(session_cookie), Some(user_id)) = (session_cookie, user_id) {
        let session_id = session_cookie.value();
        let user_id = user_id.value();

        if let Some(db_pool) = &state_lock.db_pool {
            let _ = sqlx::query("DELETE FROM sessions_panel WHERE session_id = ? AND user_id = ?")
                .bind(session_id)
                .bind(user_id)
                .execute(db_pool)
                .await;
        }
    }

    Redirect::to("/")
}
