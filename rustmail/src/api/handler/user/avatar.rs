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
        Some(pool) => pool,
        None => {
            return axum::response::Json(serde_json::json!(UserAvatar { avatar_url: None }));
        }
    };

    let user_id = params.id;

    let user_avatar = match sqlx::query!(
        "SELECT avatar_hash FROM sessions_panel WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool)
    .await
    {
        Ok(record) => {
            if let Some(avatar_hash) = record.avatar_hash {
                let avatar_url = format!(
                    "https://cdn.discordapp.com/avatars/{}/{}.png",
                    user_id, avatar_hash
                );
                UserAvatar {
                    avatar_url: Some(avatar_url),
                }
            } else {
                UserAvatar { avatar_url: None }
            }
        }
        Err(e) => {
            eprintln!("Error fetching user avatar: {}", e);
            UserAvatar { avatar_url: None }
        }
    };

    axum::response::Json(serde_json::json!(user_avatar))
}
