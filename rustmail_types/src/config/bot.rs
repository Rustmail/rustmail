use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BotConfig {
    pub token: String,
    pub mode: ServerMode,
    pub status: String,
    pub welcome_message: String,
    pub close_message: String,
    pub typing_proxy_from_user: bool,
    pub typing_proxy_from_staff: bool,
    pub enable_logs: bool,
    pub enable_features: bool,
    pub enable_panel: bool,
    pub client_id: u64,
    pub client_secret: String,
    pub redirect_url: String,
    #[serde(
        default = "default_timezone",
        deserialize_with = "deserialize_timezone",
        serialize_with = "serialize_timezone"
    )]
    pub timezone: Tz,
    #[serde(default)]
    pub logs_channel_id: Option<u64>,
    #[serde(default)]
    pub features_channel_id: Option<u64>,
    #[serde(default)]
    pub ip: Option<String>,
    #[serde(default)]
    pub panel_super_admin_users: Vec<u64>,
    #[serde(default)]
    pub panel_super_admin_roles: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerMode {
    Single {
        guild_id: u64,
    },
    Dual {
        community_guild_id: u64,
        staff_guild_id: u64,
    },
}

impl BotConfig {
    pub fn get_community_guild_id(&self) -> u64 {
        match &self.mode {
            ServerMode::Single { guild_id } => *guild_id,
            ServerMode::Dual {
                community_guild_id, ..
            } => *community_guild_id,
        }
    }

    pub fn get_staff_guild_id(&self) -> u64 {
        match &self.mode {
            ServerMode::Single { guild_id } => *guild_id,
            ServerMode::Dual { staff_guild_id, .. } => *staff_guild_id,
        }
    }

    pub fn is_dual_mode(&self) -> bool {
        matches!(self.mode, ServerMode::Dual { .. })
    }

    pub fn validate_logs_config(&self) -> Result<(), String> {
        match (self.enable_logs, self.logs_channel_id) {
            (true, None) => {
                Err("'logs_channel_id' field is required if 'enable_logs' is true".to_string())
            }
            (false, Some(_)) => {
                Err("'logs_channel_id' must not be filled in if 'enable_logs' is false".to_string())
            }
            (true, Some(_)) => Ok(()),
            (false, None) => Ok(()),
        }
    }

    pub fn validate_features_config(&self) -> Result<(), String> {
        match (self.enable_features, self.features_channel_id) {
            (true, None) => Err(
                "'features_channel_id' field is required if 'enable_features' is true".to_string(),
            ),
            (false, Some(_)) => Err(
                "'features_channel_id' must not be filled in if 'enable_features' is false"
                    .to_string(),
            ),
            (true, Some(_)) => Ok(()),
            (false, None) => Ok(()),
        }
    }

    pub fn is_community_guild(&self, guild_id: u64) -> bool {
        match &self.mode {
            ServerMode::Single { guild_id: gid } => *gid == guild_id,
            ServerMode::Dual {
                community_guild_id, ..
            } => *community_guild_id == guild_id,
        }
    }
}

fn deserialize_timezone<'de, D>(deserializer: D) -> Result<Tz, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<Tz>().map_err(serde::de::Error::custom)
}

fn serialize_timezone<S>(tz: &Tz, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&tz.to_string())
}

fn default_timezone() -> Tz {
    chrono_tz::UTC
}
