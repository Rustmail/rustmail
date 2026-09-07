use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UpdateConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default = "default_check_interval_hours")]
    pub check_interval_hours: u64,
    #[serde(default)]
    pub notify_channel_id: Option<u64>,
}

impl Default for UpdateConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            check_interval_hours: default_check_interval_hours(),
            notify_channel_id: None,
        }
    }
}

fn default_enabled() -> bool {
    true
}

fn default_check_interval_hours() -> u64 {
    6
}
