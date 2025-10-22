use crate::config::Config;
use crate::types::logs::TicketLog;
use serenity::all::{ChannelId, CommandInteraction, Message};
use serenity::builder::CreateActionRow;
use serenity::client::Context;
use crate::errors::ModmailError;
use crate::utils::message::message_builder::MessageBuilder;

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

pub fn render_logs_page(logs: &[TicketLog], page: usize, per_page: usize) -> String {
    let total_pages = (logs.len() + per_page - 1) / per_page;
    let start = page * per_page;
    let end = usize::min(start + per_page, logs.len());

    let mut desc = String::new();

    if start >= logs.len() {
        return "_Aucun log trouvé pour cet utilisateur._".into();
    }

    for (_, log) in logs[start..end].iter().enumerate() {
        use std::fmt::Write;
        let _ = writeln!(
            desc,
            "**#{}** | [`Ticket {}`]({}) | Fermé le {} {}",
            log.id,
            log.ticket_id,
            format!("http://localhost:3002/panel/tickets/{}", log.ticket_id),
            log.created_at,
            "\n".to_string(),
        );
    }

    if desc.is_empty() {
        desc = "_Aucun log trouvé pour cet utilisateur._".into();
    }

    format!(
        "{}\n_Page {}/{} ({} logs totaux)_",
        desc,
        page + 1,
        total_pages.max(1),
        logs.len()
    )
}

pub async fn get_response(ctx: Context, config: Config, content: &str, components: Vec<CreateActionRow>, channel_id: ChannelId, command: Option<CommandInteraction>) -> Result<Message, ModmailError> {

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
            .send(false)
            .await
    }
}
