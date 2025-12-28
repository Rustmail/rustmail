pub mod panel_permissions;

use crate::config::*;
use serde::{Deserialize, Serialize};

pub use panel_permissions::*;

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
    pub staff_discord_id: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatisticsOverview {
    pub open_tickets: i64,
    pub total_closed: i64,
    pub closed_today: i64,
    pub closed_this_week: i64,
    pub closed_this_month: i64,
    pub avg_response_time_seconds: Option<i64>,
    pub avg_resolution_time_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyActivity {
    pub date: String,
    pub created: i64,
    pub closed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoryStats {
    pub name: String,
    pub count: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StaffMember {
    pub user_id: String,
    pub username: String,
    pub messages_count: i64,
    pub tickets_closed: i64,
    pub avg_response_time_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopPerformer {
    pub user_id: String,
    pub username: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopPerformers {
    pub fastest_responder: Option<TopPerformer>,
    pub most_messages: Option<TopPerformer>,
    pub most_tickets_closed: Option<TopPerformer>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Statistics {
    pub overview: StatisticsOverview,
    pub activity: Vec<DailyActivity>,
    pub categories: Vec<CategoryStats>,
    pub staff_leaderboard: Vec<StaffMember>,
    pub top_performers: TopPerformers,
}
