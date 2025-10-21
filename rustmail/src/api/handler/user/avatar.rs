use crate::BotState;
use crate::api::utils::get_user_id_from_session::get_user_id_from_session;
use axum::extract::State;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

pub async fn handle_get_user_avatar(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let pool = {
        let state_lock = bot_state.lock().await;
        if let Some(pool) = &state_lock.db_pool {
            pool.clone()
        } else {
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    let session_id = match jar.get("session_id") {
        Some(cookie) => cookie.value().to_string(),
        None => {
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    let user_id_str: String = get_user_id_from_session(&session_id, &pool).await;
    let user_id: u64 = match user_id_str.parse() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error parsing user ID: {}", e);
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    let user_avatar = match sqlx::query("SELECT avatar_hash FROM sessions_panel WHERE user_id = ?")
        .bind(user_id_str.clone())
        .fetch_one(&pool)
        .await
    {
        Ok(record) => {
            let avatar_hash: String = record.get::<String, _>("avatar_hash");

            if !avatar_hash.is_empty() {
                let avatar_url = format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.png",
                    user_id_str, avatar_hash
                );
                UserAvatar {
                    avatar_url: Some(avatar_url),
                }
            } else {
                let avatar_url = format!(
                    "https://cdn.discordapp.com/embed/avatars/{}.png",
                    (user_id >> 22) % 6
                );
                UserAvatar {
                    avatar_url: Some(avatar_url),
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching user avatar: {}", e);
            UserAvatar { avatar_url: None }
        }
    };

    axum::response::Json(serde_json::json!(user_avatar))
}
