use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogsConfig {
    pub show_log_on_edit: bool,
    pub show_log_on_delete: bool,
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self {
            show_log_on_edit: true,
            show_log_on_delete: true,
        }
    }
}
