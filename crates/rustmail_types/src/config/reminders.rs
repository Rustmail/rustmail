use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ReminderConfig {
    pub embed_color: String,
}

impl Default for ReminderConfig {
    fn default() -> Self {
        Self {
            embed_color: "ffcc00".to_string(),
        }
    }
}
