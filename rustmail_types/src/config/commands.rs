use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CommandConfig {
    pub prefix: String,
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            prefix: "!".to_string(),
        }
    }
}
