use crate::BotState;
use axum::extract::{Query, State};
use axum::response::Redirect;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(serde::Deserialize)]
pub struct AuthRequest {
    pub code: String,
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

    let client = Client::new();

    let client_id = config.bot.client_id.to_string();
    let client_secret = config.bot.client_secret.clone();
    let code = params.code.clone();

    let token_response = client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", client_id.as_str()),
            ("client_secret", client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", code.as_str()),
            ("redirect_uri", "http://localhost:3002/api/auth/callback"),
        ])
        .send()
        .await
        .unwrap();

    let token_data: serde_json::Value = token_response.json().await.unwrap();
    let access_token = token_data["access_token"].as_str().unwrap();

    let client_response = client
        .get("https://discord.com/api/users/@me")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap();

    let user: DiscordUser = client_response.json().await.unwrap();
    let user_id = user.id.clone();
    println!("Authenticated user: {:?}", user);

    let cookie_token = Cookie::build(("session_token", access_token.to_string()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .build();

    let cookie_user = Cookie::build(("user_id", user_id.clone()))
        .path("/")
        .http_only(false)
        .same_site(SameSite::Lax)
        .build();

    let jar = jar.add(cookie_token).add(cookie_user);

    (jar, Redirect::to("/panel"))
}
