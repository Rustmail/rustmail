use crate::prelude::config::*;
use crate::prelude::features::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use serenity::all::{ButtonStyle, CreateActionRow};

pub async fn render_logs_page(
    config: &Config,
    logs: &[TicketLog],
    page: usize,
    per_page: usize,
) -> String {
    let total_pages = logs.len().div_ceil(per_page);
    let start = page * per_page;
    let end = usize::min(start + per_page, logs.len());
    let no_logs = get_translated_message(
        config,
        "slash_commands.no_logs_found",
        None,
        None,
        None,
        None,
    )
    .await;

    let mut desc = String::new();

    if start >= logs.len() {
        return no_logs;
    }

    for log in logs[start..end].iter() {
        use std::fmt::Write;
        let _ = writeln!(
            desc,
            "**#{}** | [`🎫 {}`]({}) | 🔒 {}",
            log.id,
            log.ticket_id,
            panel_ticket_url(config, &log.ticket_id),
            log.created_at,
        );
    }

    format!(
        "{}\n_Page {}/{} ( 🧾 {} )_",
        desc,
        page + 1,
        total_pages.max(1),
        logs.len()
    )
}

pub async fn logs_page_buttons(
    config: &Config,
    session_id: &str,
    page: usize,
    total_logs: usize,
) -> Vec<CreateActionRow> {
    let prev = get_translated_message(config, "logs_command.prev", None, None, None, None).await;
    let next = get_translated_message(config, "logs_command.next", None, None, None, None).await;

    make_buttons(&[
        (
            &prev,
            &format!("command:logs_prev:{session_id}"),
            ButtonStyle::Primary,
            page == 0,
        ),
        (
            &next,
            &format!("command:logs_next:{session_id}"),
            ButtonStyle::Primary,
            !has_next_logs_page(page, total_logs),
        ),
    ])
}

pub fn has_next_logs_page(page: usize, total_logs: usize) -> bool {
    (page + 1) * LOGS_PAGE_SIZE < total_logs
}
