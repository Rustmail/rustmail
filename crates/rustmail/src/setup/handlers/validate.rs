use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Deserialize)]
pub struct ValidateGuildRequest {
    pub token: String,
    pub guild_id: String,
}

#[derive(Deserialize)]
pub struct ValidateChannelRequest {
    pub token: String,
    pub guild_id: String,
    pub channel_id: String,
}

#[derive(Serialize)]
pub struct BotInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Serialize)]
pub struct GuildInfo {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Serialize)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub kind: u8,
}

#[derive(Serialize)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub bot: Option<BotInfo>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ValidateGuildResponse {
    pub valid: bool,
    pub guild: Option<GuildInfo>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ValidateChannelResponse {
    pub valid: bool,
    pub channel: Option<ChannelInfo>,
    pub error: Option<String>,
}

pub async fn handle_validate_token(Json(payload): Json<ValidateTokenRequest>) -> impl IntoResponse {
    let client = Client::new();

    let resp = match client
        .get("https://discord.com/api/v10/users/@me")
        .header("Authorization", format!("Bot {}", payload.token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateTokenResponse {
                    valid: false,
                    bot: None,
                    error: Some("Network error reaching Discord API".to_string()),
                }),
            );
        }
    };

    if !resp.status().is_success() {
        return (
            StatusCode::OK,
            Json(ValidateTokenResponse {
                valid: false,
                bot: None,
                error: Some("Invalid token".to_string()),
            }),
        );
    }

    let data: serde_json::Value = match resp.json().await {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateTokenResponse {
                    valid: false,
                    bot: None,
                    error: Some("Failed to parse Discord response".to_string()),
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(ValidateTokenResponse {
            valid: true,
            bot: Some(BotInfo {
                id: data["id"].as_str().unwrap_or("").to_string(),
                username: data["username"].as_str().unwrap_or("").to_string(),
                avatar: data["avatar"].as_str().map(|s| s.to_string()),
            }),
            error: None,
        }),
    )
}

pub async fn handle_validate_guild(Json(payload): Json<ValidateGuildRequest>) -> impl IntoResponse {
    let client = Client::new();

    let resp = match client
        .get(format!(
            "https://discord.com/api/v10/guilds/{}",
            payload.guild_id
        ))
        .header("Authorization", format!("Bot {}", payload.token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateGuildResponse {
                    valid: false,
                    guild: None,
                    error: Some("Network error reaching Discord API".to_string()),
                }),
            );
        }
    };

    if !resp.status().is_success() {
        return (
            StatusCode::OK,
            Json(ValidateGuildResponse {
                valid: false,
                guild: None,
                error: Some("Guild not found or bot is not a member".to_string()),
            }),
        );
    }

    let data: serde_json::Value = match resp.json().await {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateGuildResponse {
                    valid: false,
                    guild: None,
                    error: Some("Failed to parse Discord response".to_string()),
                }),
            );
        }
    };

    (
        StatusCode::OK,
        Json(ValidateGuildResponse {
            valid: true,
            guild: Some(GuildInfo {
                id: data["id"].as_str().unwrap_or("").to_string(),
                name: data["name"].as_str().unwrap_or("").to_string(),
                icon: data["icon"].as_str().map(|s| s.to_string()),
            }),
            error: None,
        }),
    )
}

pub async fn handle_validate_channel(
    Json(payload): Json<ValidateChannelRequest>,
) -> impl IntoResponse {
    let client = Client::new();

    let resp = match client
        .get(format!(
            "https://discord.com/api/v10/channels/{}",
            payload.channel_id
        ))
        .header("Authorization", format!("Bot {}", payload.token))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateChannelResponse {
                    valid: false,
                    channel: None,
                    error: Some("Network error reaching Discord API".to_string()),
                }),
            );
        }
    };

    if !resp.status().is_success() {
        return (
            StatusCode::OK,
            Json(ValidateChannelResponse {
                valid: false,
                channel: None,
                error: Some("Channel not found or bot has no access".to_string()),
            }),
        );
    }

    let data: serde_json::Value = match resp.json().await {
        Ok(d) => d,
        Err(_) => {
            return (
                StatusCode::OK,
                Json(ValidateChannelResponse {
                    valid: false,
                    channel: None,
                    error: Some("Failed to parse Discord response".to_string()),
                }),
            );
        }
    };

    let guild_id = data["guild_id"].as_str().unwrap_or("");
    if guild_id != payload.guild_id {
        return (
            StatusCode::OK,
            Json(ValidateChannelResponse {
                valid: false,
                channel: None,
                error: Some("Channel does not belong to the specified guild".to_string()),
            }),
        );
    }

    (
        StatusCode::OK,
        Json(ValidateChannelResponse {
            valid: true,
            channel: Some(ChannelInfo {
                id: data["id"].as_str().unwrap_or("").to_string(),
                name: data["name"].as_str().unwrap_or("").to_string(),
                kind: data["type"].as_u64().unwrap_or(0) as u8,
            }),
            error: None,
        }),
    )
}
