use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{CommandInteraction, Context, Message};
use std::sync::Arc;

pub async fn display_commands_list(
    ctx: &Context,
    config: &Config,
    registry: Arc<CommandRegistry>,
    msg: Option<&Message>,
    command: Option<&CommandInteraction>,
) -> ModmailResult<()> {
    let mut docs_message = String::new();

    let welcome_msg = get_translated_message(&config, "help.message", None, None, None, None).await;
    docs_message.push_str(&welcome_msg);

    for (name, _) in &registry.commands {
        docs_message.push_str(&format!("- **{}**\n", name))
    }

    if let Some(msg) = msg {
        let _ = MessageBuilder::system_message(&ctx, config)
            .content(docs_message)
            .to_channel(msg.channel_id)
            .send(true)
            .await;

        return Ok(());
    }

    if let Some(command) = command {
        let _ = MessageBuilder::system_message(&ctx, config)
            .content(docs_message)
            .to_channel(command.channel_id)
            .send_interaction_followup(&command, true)
            .await;

        return Ok(());
    }

    println!("No valid message or command interaction provided.");
    Ok(())
}

pub async fn display_command_help(
    ctx: &Context,
    config: &Config,
    registry: Arc<CommandRegistry>,
    msg: Option<&Message>,
    command: Option<&CommandInteraction>,
    command_name: &str,
) -> ModmailResult<()> {
    if let Some(cmd) = registry.commands.get(command_name) {
        let command_doc = cmd.doc(config).await;
        let mut docs_message = String::new();
        docs_message.push_str(&format!("**{}**\n\n", command_name));
        docs_message.push_str(&command_doc);

        if let Some(msg) = msg {
            let _ = MessageBuilder::system_message(&ctx, config)
                .content(docs_message)
                .to_channel(msg.channel_id)
                .send(true)
                .await;

            return Ok(());
        }

        if let Some(command) = command {
            let _ = MessageBuilder::system_message(&ctx, config)
                .content(docs_message)
                .to_channel(command.channel_id)
                .send_interaction_followup(&command, true)
                .await;

            return Ok(());
        }

        println!("No valid message or command interaction provided.");
        Ok(())
    } else {
        Err(ModmailError::Command(CommandError::UnknownCommand(
            format!("{}", command_name),
        )))
    }
}
