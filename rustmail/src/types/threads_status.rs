#[derive(Debug, Clone)]
pub struct TicketState {
    pub channel_id: i64,
    pub owner_id: String,
    pub taken_by: Option<String>,
    pub last_message_by: TicketAuthor,
    pub last_message_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TicketAuthor {
    Staff,
    User,
}

impl TicketAuthor {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "staff" => TicketAuthor::Staff,
            _ => TicketAuthor::User,
        }
    }
}
