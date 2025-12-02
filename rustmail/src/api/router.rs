use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_api_router(bot_state: Arc<Mutex<BotState>>) -> Router {
    let admin_router = create_admin_router(bot_state.clone());
    let apikeys_router = create_apikeys_router(bot_state.clone());
    let bot_router = create_bot_router(bot_state.clone());
    let auth_router = create_auth_router();
    let panel_router = create_panel_router(bot_state.clone());
    let user_router = create_user_router(bot_state.clone());
    let external_router = create_external_router(bot_state.clone());

    let app = Router::new()
        .nest("/api/admin", admin_router)
        .nest("/api/apikeys", apikeys_router)
        .nest("/api/bot", bot_router)
        .nest("/api/auth", auth_router)
        .nest("/api/panel", panel_router)
        .nest("/api/user", user_router)
        .nest("/api/externals", external_router)
        .with_state(bot_state.clone());

    app
}
