use crate::prelude::types::*;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::{Duration, Utc};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    pub code: String,
    pub state: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct DiscordUser {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
}

pub async fn handle_callback(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
    Query(params): Query<AuthRequest>,
) -> (CookieJar, Redirect) {
    let state_lock = bot_state.lock().await;
    let Some(config) = &state_lock.config else {
        return (jar, Redirect::to("/error?message=Missing+configuration"));
    };

    let Some(db_pool) = &state_lock.db_pool else {
        return (jar, Redirect::to("/error?message=Database+not+initialized"));
    };

    let client = Client::new();

    let token_response = match client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", config.bot.client_id.to_string().as_str()),
            ("client_secret", config.bot.client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", params.code.as_str()),
            ("redirect_uri", config.bot.redirect_url.as_str()),
        ])
        .send()
        .await
    {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("⚠️ Token exchange failed with status: {}", resp.status());
                return (jar, Redirect::to("/error?message=Token+exchange+failed"));
            }
            resp
        }
        Err(e) => {
            eprintln!(
                "⚠️ Failed to exchange code for token (maybe client_secret or client_id: {}",
                e
            );
            return (
                jar,
                Redirect::to("/error?message=Failed+to+exchange+code+for+token"),
            );
        }
    };

    let token_data: serde_json::Value = token_response.json().await.unwrap();

    let access_token = token_data["access_token"].as_str().unwrap_or("");
    let refresh_token = token_data["refresh_token"].as_str().unwrap_or("");
    let expires_in = token_data["expires_in"].as_i64().unwrap_or(3600);

    let user_response = match client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("⚠️ Failed to fetch user info: {}", e);
            return (
                jar,
                Redirect::to("/error?message=Failed+to+fetch+user+info"),
            );
        }
    };

    let user: DiscordUser = match user_response.json().await {
        Ok(user) => user,
        Err(e) => {
            eprintln!("⚠️ Failed to parse user info: {}", e);
            return (
                jar,
                Redirect::to("/error?message=Failed+to+parse+user+info"),
            );
        }
    };
    let user_id = user.id.clone();

    let session_id = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + Duration::seconds(expires_in);
    let timestamp = expires_at.timestamp();

    if let Err(e) = sqlx::query!(r#"DELETE FROM sessions_panel WHERE user_id = ?"#, user_id)
        .execute(db_pool)
        .await
    {
        eprintln!("⚠️ Failed to delete old session: {}", e);
        return (jar, Redirect::to("/error?message=Database+delete+failed"));
    }

    if let Err(e) = sqlx::query!(
        r#"
        INSERT INTO sessions_panel (session_id, user_id, access_token, refresh_token, expires_at, avatar_hash)
        VALUES (?, ?, ?, ?, ?, ?)
        ON CONFLICT(session_id) DO UPDATE SET
            access_token = excluded.access_token,
            refresh_token = excluded.refresh_token,
            expires_at = excluded.expires_at,
            avatar_hash = excluded.avatar_hash
        "#,
        session_id,
        user_id,
        access_token,
        refresh_token,
        timestamp,
        user.avatar
    )
    .execute(db_pool)
    .await
    {
        eprintln!("⚠️ Failed to store session in database: {}", e);
        return (jar, Redirect::to("/error?message=Database+write+failed"));
    }

    let cookie_session = Cookie::build(("session_id", session_id))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let jar = jar.add(cookie_session);

    let target = params.state.unwrap_or_else(|| "/panel".to_string());
    (jar, Redirect::to(&target))
}
