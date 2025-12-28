pub mod tickets;
pub use tickets::*;

use crate::api::auth_middleware;
use crate::types::BotState;
use axum::Router;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn create_external_router(bot_state: Arc<Mutex<BotState>>) -> Router<Arc<Mutex<BotState>>> {
    let external_tickets_router = create_external_ticket_router(bot_state.clone());

    let bot_router = Router::new()
        .nest("/tickets", external_tickets_router)
        .layer(axum::middleware::from_fn_with_state(
            bot_state,
            auth_middleware,
        ));

    bot_router
}
