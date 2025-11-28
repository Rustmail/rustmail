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
