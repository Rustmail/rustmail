use crate::config::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ConfigResponse {
    pub bot: BotConfig,
    pub command: CommandConfig,
    pub thread: ThreadConfig,
    pub language: LanguageConfig,
    pub error_handling: ErrorHandlingConfig,
    pub notifications: NotificationsConfig,
    pub reminders: ReminderConfig,
    pub logs: LogsConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateTicket {
    pub discord_id: String,
    pub api_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: i64,
    pub key: String,
    pub content: String,
    pub created_by: String,
    pub created_at: String,
    pub updated_at: String,
}
