use crate::prelude::api::*;
use crate::prelude::types::*;
use axum::Router;
use axum::routing::{delete, get, post};
use rustmail_types::api::panel_permissions::PanelPermission;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_admin_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let admin_router = Router::new()
        .route("/members", get(handle_list_members))
        .route("/roles", get(handle_list_roles))
        .route("/permissions", get(handle_list_permissions))
        .route("/permissions", post(handle_grant_permission))
        .route("/permissions/{id}", delete(handle_revoke_permission))
        .layer(axum::middleware::from_fn_with_state(
            bot_state.clone(),
            move |state, jar, req, next| {
                require_panel_permission(state, jar, req, next, PanelPermission::ManagePermissions)
            },
        ))
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    admin_router
}
