use serenity::all::{ChannelId, MessageId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct TicketLog {
    pub id: i64,
    pub ticket_id: String,
    pub user_id: String,
    pub created_at: String,
}

#[derive(Clone)]
pub struct PaginationContext {
    pub user_id: String,
    pub logs: Vec<TicketLog>,
    pub current_page: usize,
    pub message_id: MessageId,
    pub channel_id: ChannelId,
}

pub type PaginationStore = Arc<Mutex<HashMap<String, PaginationContext>>>;
