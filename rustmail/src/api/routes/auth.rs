use crate::api::handler::auth::callback::handle_callback;
use crate::api::handler::auth::login::handle_login;
use crate::api::handler::auth::logout::handle_logout;
use crate::BotState;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_auth_router() -> Router<Arc<Mutex<BotState>>> {
    let auth_router = Router::new()
        .route("/login", get(handle_login))
        .route("/callback", get(handle_callback))
        .route("/logout", get(handle_logout));

    auth_router
}
