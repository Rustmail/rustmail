use crate::BotState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UserIdQuery {
    id: String,
}

pub async fn handle_get_user_avatar(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Query(params): Query<UserIdQuery>,
) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    let pool = match &state_lock.db_pool {
        Some(pool) => pool.clone(),
        None => {
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    drop(state_lock);

    let user_id_str = params.id;

    let user_id: u64 = match user_id_str.parse::<u64>() {
        Ok(id) => id,
        Err(e) => {
            eprintln!("Error parsing user ID: {}", e);
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    let user_avatar = match sqlx::query!(
        "SELECT avatar_hash FROM sessions_panel WHERE user_id = ?",
        user_id_str
    )
    .fetch_one(&pool)
    .await
    {
        Ok(record) => {
            if let Some(avatar_hash) = record.avatar_hash {
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
