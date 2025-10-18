use crate::BotState;
use crate::api::routes::auth::create_auth_router;
use crate::api::routes::bot::create_bot_router;
use crate::api::routes::panel::create_panel_router;
use crate::api::routes::user::create_user_router;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_api_router(bot_state: Arc<Mutex<BotState>>) -> Router {
    let bot_router = create_bot_router(bot_state.clone());
    let auth_router = create_auth_router();
    let panel_router = create_panel_router(bot_state.clone());
    let user_router = create_user_router(bot_state.clone());

    let app = Router::new()
        .nest("/api/bot", bot_router)
        .nest("/api/auth", auth_router)
        .nest("/api/panel", panel_router)
        .nest("/api/user", user_router)
        .with_state(bot_state.clone());

    app
}
