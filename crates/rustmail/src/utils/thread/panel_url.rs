use crate::prelude::config::*;

pub fn panel_ticket_url(config: &Config, ticket_id: &str) -> String {
    let base_url = config
        .bot
        .redirect_url
        .trim_end_matches("/api/auth/callback")
        .trim_end_matches('/');
    format!("{base_url}/panel/tickets/{ticket_id}")
}
