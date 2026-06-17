use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq)]
pub struct WizardData {
    pub token: String,
    pub server_mode: String,
    pub single_guild_id: String,
    pub community_guild_id: String,
    pub staff_guild_id: String,
}

#[derive(Serialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct BotInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub bot: Option<BotInfo>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ValidateGuildRequest {
    pub token: String,
    pub guild_id: String,
}

#[derive(Deserialize, Clone)]
pub struct GuildInfo {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ValidateGuildResponse {
    pub valid: bool,
    pub guild: Option<GuildInfo>,
    pub error: Option<String>,
}
