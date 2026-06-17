use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq)]
pub struct WizardData {
    pub token: String,
    pub server_mode: String,
    pub single_guild_id: String,
    pub community_guild_id: String,
    pub staff_guild_id: String,
    pub inbox_category_id: String,
    pub command_prefix: String,
    pub user_message_color: String,
    pub staff_message_color: String,
    pub system_message_color: String,
    pub embedded_message: bool,
    pub block_quote: bool,
    pub time_to_close_thread: u64,
    pub create_ticket_by_create_channel: bool,
    pub close_on_leave: bool,
    pub auto_archive_duration: u16,
    pub panel_url: String,
    pub api_port: u16,
    pub client_id: String,
    pub client_secret: String,
    pub locale: String,
    pub timezone: String,
    pub status: String,
    pub direct_message: String,
}

impl Default for WizardData {
    fn default() -> Self {
        Self {
            token: String::new(),
            server_mode: "single".to_string(),
            single_guild_id: String::new(),
            community_guild_id: String::new(),
            staff_guild_id: String::new(),
            inbox_category_id: String::new(),
            command_prefix: "!".to_string(),
            user_message_color: "5865F2".to_string(),
            staff_message_color: "ED4245".to_string(),
            system_message_color: "FEE75C".to_string(),
            embedded_message: true,
            block_quote: true,
            time_to_close_thread: 0,
            create_ticket_by_create_channel: false,
            close_on_leave: true,
            auto_archive_duration: 1440,
            panel_url: String::new(),
            api_port: 8080,
            client_id: String::new(),
            client_secret: String::new(),
            locale: "en".to_string(),
            timezone: "Europe/Paris".to_string(),
            status: "Need help? DM me!".to_string(),
            direct_message:
                "Thank you for contacting support! A staff member will be with you shortly."
                    .to_string(),
        }
    }
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

#[derive(Serialize)]
pub struct ValidateChannelRequest {
    pub token: String,
    pub guild_id: String,
    pub channel_id: String,
}

#[derive(Deserialize, Clone)]
pub struct ChannelInfo {
    pub id: String,
    pub name: String,
    pub kind: u8, // 4 is GUILD_CATEGORY
}

#[derive(Deserialize, Clone)]
pub struct ValidateChannelResponse {
    pub valid: bool,
    pub channel: Option<ChannelInfo>,
    pub error: Option<String>,
}
