use crate::api::handler::auth::callback::handle_callback;
use crate::api::handler::auth::login::handle_login;
use crate::BotState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_auth_router() -> Router<Arc<Mutex<BotState>>> {
    let auth_router = Router::new()
        .route("/login", get(handle_login))
        .route("/callback", get(handle_callback));

    auth_router
}
