use crate::commands::help::common::{display_command_help, display_commands_list};
use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use serenity::all::{Context, Message};
use std::sync::Arc;

fn extract_request_command_name(command: &str) -> &str {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    if parts.len() > 1 { parts[1] } else { "" }
}

pub async fn help(
    ctx: Context,
    msg: Message,
    config: &Config,
    handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let command_name = extract_request_command_name(&msg.content);

    if command_name.is_empty() {
        display_commands_list(&ctx, config, handler.registry.clone(), Some(&msg), None).await?;
    } else {
        display_command_help(
            &ctx,
            config,
            handler.registry.clone(),
            Some(&msg),
            None,
            command_name,
        )
        .await?;
    }

    Ok(())
}
