use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ErrorHandlingConfig {
    pub show_detailed_errors: bool,
    pub log_errors: bool,
    pub send_error_embeds: bool,
    pub auto_delete_error_messages: bool,
    pub error_message_ttl: Option<u64>,
    #[serde(default = "default_display_errors")]
    pub display_errors: bool,
}

fn default_display_errors() -> bool {
    true
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            show_detailed_errors: true,
            log_errors: true,
            send_error_embeds: true,
            auto_delete_error_messages: false,
            error_message_ttl: None,
            display_errors: true,
        }
    }
}
