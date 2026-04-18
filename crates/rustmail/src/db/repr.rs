#[derive(Debug, Clone)]
pub struct Thread {
    pub id: String,
    pub user_id: i64,
    pub user_name: String,
    pub channel_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub key_hash: String,
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub expires_at: Option<i64>,
    pub last_used_at: Option<i64>,
    pub is_active: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TicketCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub emoji: Option<String>,
    pub discord_category_id: String,
    pub position: i64,
    pub enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TicketCategorySettings {
    pub enabled: bool,
    pub selection_timeout_s: i64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PendingCategorySelection {
    pub user_id: i64,
    pub prompt_msg_id: String,
    pub dm_channel_id: String,
    pub started_at: i64,
    pub expires_at: i64,
    pub queued_msg_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TrackedMember {
    pub guild_id: String,
    pub user_id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<i64>,
    pub first_seen_at: i64,
    pub last_seen_at: i64,
}

#[derive(Debug, Clone)]
pub struct BannedUser {
    pub guild_id: String,
    pub user_id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub roles: Vec<String>,
    pub joined_at: Option<i64>,
    pub banned_at: i64,
    pub banned_by: Option<String>,
    pub ban_reason: Option<String>,
    pub roles_unknown: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    CreateTicket,
    ReadTickets,
    UpdateTicket,
    DeleteTicket,
    ReadConfig,
    UpdateConfig,
    ManageBot,
}
