use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::{delete, get, post};
use rustmail_types::api::panel_permissions::PanelPermission;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_apikeys_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let apikeys_router = Router::new()
        .route("/", post(create_api_key_handler))
        .route("/", get(list_api_keys_handler))
        .route("/{id}/revoke", post(revoke_api_key_handler))
        .route("/{id}", delete(delete_api_key_handler))
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ManageApiKeys)
            },
        ))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    apikeys_router
}
