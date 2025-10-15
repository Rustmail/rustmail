use crate::{BotCommand, BotState};
use axum::extract::Request;
use axum::extract::State;
use axum::middleware::Next;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::CookieJar;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn check_user_with_bot(bot_state: Arc<Mutex<BotState>>, user_id: &str) -> bool {
    let user_id_num = match user_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return false,
    };

    let cmd_tx = {
        let state_lock = bot_state.lock().await;
        state_lock.command_tx.clone()
    };

    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();

    if cmd_tx
        .send(BotCommand::CheckUserIsMember {
            user_id: user_id_num,
            resp: resp_tx,
        })
        .await
        .is_err()
    {
        return false;
    }

    let test = resp_rx.await.unwrap();
    println!("User is member: {}", test);

    test
}

async fn check_user_with_api(token: &str) -> bool {
    false
}

async fn verify_token(token: &str, user_id: &str, bot_state: Arc<Mutex<BotState>>) -> bool {
    let state_lock = bot_state.lock().await;

    let bot_http = state_lock.bot_http.clone();
    drop(state_lock);

    if let Some(http) = &bot_http {
        return check_user_with_bot(bot_state, user_id).await;
    }

    check_user_with_api(token).await
}

pub async fn auth_middleware(
    State(bot_state): State<Arc<Mutex<BotState>>>,
    jar: CookieJar,
    req: Request,
    next: Next,
) -> Response {
    let token_cookie = jar.get("session_token");
    let user_cookie = jar.get("user_id");

    if token_cookie.is_none() || user_cookie.is_none() {
        return Redirect::to("/login").into_response();
    }

    let token = token_cookie.unwrap().value().to_string();
    let user_id = user_cookie.unwrap().value().to_string();

    let is_valid = verify_token(token.as_str(), user_id.as_str(), bot_state).await;

    if !is_valid {
        return Redirect::to("/login").into_response();
    }

    next.run(req).await
}
