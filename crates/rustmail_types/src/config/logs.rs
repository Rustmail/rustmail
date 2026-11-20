use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct LogsConfig {
    pub show_log_on_edit: bool,
    pub show_log_on_delete: bool,
    #[serde(default)]
    pub hide_ticket_rename_logs: bool,
}

impl Default for LogsConfig {
    fn default() -> Self {
        Self {
            show_log_on_edit: true,
            show_log_on_delete: true,
            hide_ticket_rename_logs: false,
        }
    }
}
