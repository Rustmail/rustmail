use crate::BotState;
use axum::extract::State;
use axum::response::Redirect;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_panel(State(bot_state): State<Arc<Mutex<BotState>>>) -> Redirect {
    Redirect::to("/panel")
}
