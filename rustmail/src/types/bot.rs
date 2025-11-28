use crate::prelude::config::*;
use serenity::all::Http;
use std::sync::Arc;
use tokio::sync::watch::Sender;
use tokio::task::JoinHandle;

pub enum BotStatus {
    Stopped,
    Running {
        handle: JoinHandle<()>,
        shutdown: Sender<bool>,
    },
}

pub enum BotCommand {
    CheckUserIsMember {
        user_id: u64,
        resp: tokio::sync::oneshot::Sender<bool>,
    },
}

pub struct BotState {
    pub config: Option<Config>,
    pub status: BotStatus,
    pub db_pool: Option<sqlx::SqlitePool>,
    pub command_tx: tokio::sync::mpsc::Sender<BotCommand>,
    pub bot_http: Option<Arc<Http>>,
}
