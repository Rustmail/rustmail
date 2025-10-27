use crate::config::Config;
use crate::errors::ModmailError;
use crate::i18n::get_translated_message;
use crate::types::logs::TicketLog;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ChannelId, CommandInteraction, Message};
use serenity::builder::CreateActionRow;
use serenity::client::Context;

pub fn extract_user_id(msg: &Message, config: &Config) -> String {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_name = "logs";

    if content.starts_with(&format!("{}{}", prefix, command_name)) {
        let start = prefix.len() + command_name.len();
        content[start..].trim().to_string()
    } else {
        String::new()
    }
}

pub async fn render_logs_page(
    config: &Config,
    logs: &[TicketLog],
    page: usize,
    per_page: usize,
) -> String {
    let total_pages = (logs.len() + per_page - 1) / per_page;
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

    for (_, log) in logs[start..end].iter().enumerate() {
        use std::fmt::Write;
        let _ = writeln!(
            desc,
            "**#{}** | [`🎫 {}`]({}) | 🔒 {} {}",
            log.id,
            log.ticket_id,
            format!(
                "http://{}:3002/panel/tickets/{}",
                &config.bot.ip.clone().unwrap(),
                log.ticket_id
            ),
            log.created_at,
            "\n".to_string(),
        );
    }

    if desc.is_empty() {
        desc = "_Aucun log trouvé pour cet utilisateur._".into();
    }

    format!(
        "{}\n_Page {}/{} ( 🧾 {} )_",
        desc,
        page + 1,
        total_pages.max(1),
        logs.len()
    )
}

pub async fn get_response(
    ctx: Context,
    config: Config,
    content: &str,
    components: Vec<CreateActionRow>,
    channel_id: ChannelId,
    command: Option<CommandInteraction>,
) -> Result<Message, ModmailError> {
    if !command.is_none() {
        let command = command.unwrap();

        let response = MessageBuilder::system_message(&ctx.clone(), &config.clone())
            .content(content)
            .components(components)
            .to_channel(channel_id)
            .build_interaction_message_followup()
            .await;

        let tkt = command.create_followup(&ctx.http, response).await;

        Ok(tkt?)
    } else {
        MessageBuilder::system_message(&ctx, &config)
            .content(content)
            .components(components)
            .to_channel(channel_id)
            .send(true)
            .await
    }
}
