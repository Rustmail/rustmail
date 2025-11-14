use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct NotificationsConfig {
    pub show_success_on_edit: bool,
    pub show_partial_success_on_edit: bool,
    pub show_failure_on_edit: bool,
    pub show_success_on_reply: bool,
    pub show_success_on_delete: bool,
    #[serde(default = "default_show_success")]
    pub show_success: bool,
    #[serde(default = "default_show_error")]
    pub show_error: bool,
}

fn default_show_success() -> bool {
    true
}

fn default_show_error() -> bool {
    true
}

impl Default for NotificationsConfig {
    fn default() -> Self {
        Self {
            show_success_on_edit: true,
            show_partial_success_on_edit: true,
            show_failure_on_edit: true,
            show_success_on_reply: true,
            show_success_on_delete: true,
            show_success: true,
            show_error: true,
        }
    }
}
