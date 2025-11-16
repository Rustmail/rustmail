use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ThreadConfig {
    pub inbox_category_id: u64,
    pub embedded_message: bool,
    pub user_message_color: String,
    pub staff_message_color: String,
    pub system_message_color: String,
    pub block_quote: bool,
    pub time_to_close_thread: u64,
    pub create_ticket_by_create_channel: bool,
    #[serde(default = "default_close_on_leave")]
    pub close_on_leave: bool,
    #[serde(default = "default_auto_archive_duration")]
    pub auto_archive_duration: u16,
}

fn default_close_on_leave() -> bool {
    false
}

fn default_auto_archive_duration() -> u16 {
    10080
}

impl Default for ThreadConfig {
    fn default() -> Self {
        Self {
            inbox_category_id: 0,
            embedded_message: false,
            user_message_color: "5865f2".to_string(),
            staff_message_color: "57f287".to_string(),
            system_message_color: "faa81a".to_string(),
            block_quote: false,
            time_to_close_thread: 0,
            create_ticket_by_create_channel: false,
            close_on_leave: false,
            auto_archive_duration: 10080,
        }
    }
}
