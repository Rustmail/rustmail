use crate::BotState;
use axum::extract::State;
use axum::response::Redirect;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_login(State(bot_state): State<Arc<Mutex<BotState>>>) -> Redirect {
    println!("Yo");
    let state_lock = bot_state.lock().await;

    println!("Handling login request");

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
