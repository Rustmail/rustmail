use crate::api::router::create_api_router;
use crate::bot::init_bot_state;
use crate::config::Config;
use tokio::signal;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;

mod api;
mod bot;
mod commands;
mod config;
mod db;
mod errors;
mod features;
mod handlers;
mod i18n;
mod modules;
mod utils;

pub struct Database;

enum BotStatus {
    Stopped,
    Running {
        handle: JoinHandle<()>,
        shutdown: Sender<bool>,
    },
}

struct BotState {
    config: Option<Config>,
    status: BotStatus,
    db_pool: Option<sqlx::SqlitePool>,
}

#[tokio::main]
async fn main() {
    let bot_state = init_bot_state().await;

    let app = create_api_router(bot_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C handler");
    println!("Shutdown signal received");
}
