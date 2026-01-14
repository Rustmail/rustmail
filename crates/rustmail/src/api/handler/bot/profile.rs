use crate::prelude::types::*;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use serenity::all::EditProfile;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub avatar: Option<String>,
    #[serde(default)]
    pub banner: Option<String>,
}

pub async fn handle_get_profile(
    State(bot_state): State<Arc<Mutex<BotState>>>,
) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    let ctx_guard = state_lock.bot_context.read().await;
    let ctx = match ctx_guard.as_ref() {
        Some(c) => c,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Bot is not running"})),
            );
        }
    };

    let current_user = ctx.cache.current_user().clone();

    let avatar_url = current_user.avatar_url();
    let banner_url = current_user.banner_url();

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "username": current_user.name,
            "avatar_url": avatar_url,
            "banner_url": banner_url,
            "discriminator": current_user.discriminator.map(|d| d.to_string()).unwrap_or_default(),
            "id": current_user.id.to_string()
        })),
    )
}

pub async fn handle_update_profile(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    Json(payload): Json<UpdateProfileRequest>,
) -> impl IntoResponse {
    let state_lock = bot_state.lock().await;

    let http = match &state_lock.bot_http {
        Some(h) => h.clone(),
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "Bot HTTP client not available"})),
            );
        }
    };

    let mut edit_profile = EditProfile::new();
    let mut changes_made = false;

    if let Some(ref username) = payload.username {
        if username.len() < 2 || username.len() > 32 {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "Username must be between 2 and 32 characters"})),
            );
        }
        edit_profile = edit_profile.username(username);
        changes_made = true;
    }

    if let Some(ref avatar_base64) = payload.avatar {
        match parse_base64_image(avatar_base64) {
            Ok(create_attachment) => {
                edit_profile = edit_profile.avatar(&create_attachment);
                changes_made = true;
            }
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error": format!("Invalid avatar: {}", e)})),
                );
            }
        }
    }

    if let Some(ref banner_base64) = payload.banner {
        match parse_base64_image(banner_base64) {
            Ok(create_attachment) => {
                edit_profile = edit_profile.banner(&create_attachment);
                changes_made = true;
            }
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"error": format!("Invalid banner: {}", e)})),
                );
            }
        }
    }

    if !changes_made {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "No changes specified"})),
        );
    }

    match http.edit_profile(&edit_profile).await {
        Ok(user) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "success": true,
                "username": user.name,
                "avatar_url": user.avatar_url(),
                "banner_url": user.banner_url()
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Failed to update profile: {}", e)})),
        ),
    }
}

fn parse_base64_image(data: &str) -> Result<serenity::all::CreateAttachment, String> {
    let (content_type, base64_data) = if data.starts_with("data:") {
        let parts: Vec<&str> = data.splitn(2, ',').collect();
        if parts.len() != 2 {
            return Err("Invalid data URI format".to_string());
        }

        let metadata = parts[0];
        let base64_content = parts[1];

        let ct = if metadata.contains("image/png") {
            "image/png"
        } else if metadata.contains("image/jpeg") || metadata.contains("image/jpg") {
            "image/jpeg"
        } else if metadata.contains("image/gif") {
            "image/gif"
        } else if metadata.contains("image/webp") {
            "image/webp"
        } else {
            return Err("Unsupported image format. Use PNG, JPEG, GIF, or WebP".to_string());
        };

        (ct, base64_content)
    } else {
        ("image/png", data)
    };

    let image_bytes = BASE64
        .decode(base64_data)
        .map_err(|e| format!("Failed to decode base64: {}", e))?;

    let filename = match content_type {
        "image/png" => "image.png",
        "image/jpeg" => "image.jpg",
        "image/gif" => "image.gif",
        "image/webp" => "image.webp",
        _ => "image.png",
    };

    Ok(serenity::all::CreateAttachment::bytes(
        image_bytes,
        filename,
    ))
}
