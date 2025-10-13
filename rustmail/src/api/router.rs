use crate::BotState;
use crate::api::routes::bot::create_bot_router;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_api_router(bot_state: Arc<Mutex<BotState>>) -> Router {
    let bot_router = create_bot_router();

    let app = Router::new()
        .nest("/rustmail", bot_router)
        .with_state(bot_state.clone());

    app
}
