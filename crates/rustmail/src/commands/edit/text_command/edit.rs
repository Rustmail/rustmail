use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn edit(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let raw_content: String = match extract_command_content(&msg, config) {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    let command_input: EditCommandInput = match parse_edit_command(&raw_content) {
        Ok(command_input) => command_input,
        Err(e) => return Err(e),
    };

    match validate_edit_permissions(
        command_input.message_number,
        msg.channel_id,
        msg.author.id,
        pool,
    )
    .await
    {
        Ok(()) => (),
        Err(e) => return Err(e),
    };

    let ids = match get_message_ids(
        command_input.message_number,
        msg.author.id,
        pool,
        &ctx,
        msg.channel_id,
    )
    .await
    {
        Ok(ids) => ids,
        Err(e) => return Err(e),
    };

    let dm_msg_id = match ids.dm_message_id {
        Some(msg_id) => msg_id,
        None => return Err(message_not_found("Inbox message ID not found")),
    };

    let inbox_message_id = match ids.inbox_message_id {
        Some(msg_id) => msg_id,
        None => return Err(message_not_found("DM message ID not found")),
    };

    let edited_messages_builder = match format_new_message(
        &ctx,
        (Some(&msg), None),
        &command_input.new_content,
        &inbox_message_id,
        command_input.message_number as u64,
        config,
        pool,
    )
    .await
    {
        Ok(edited_messages) => edited_messages,
        Err(e) => return Err(e),
    };

    let before_content: String =
        match get_thread_message_by_inbox_message_id(&inbox_message_id, pool).await {
            Ok(tm) => tm.content,
            Err(_) => String::new(),
        };

    let edit_result = edit_messages(
        &ctx,
        msg.channel_id,
        dm_msg_id.clone(),
        inbox_message_id.clone(),
        edited_messages_builder,
        pool,
        config,
    )
    .await;

    match edit_result {
        Ok(()) => {
            if config.notifications.show_success_on_edit {
                let _ = MessageBuilder::system_message(&ctx, config)
                    .translated_content(
                        "success.message_edited",
                        None,
                        Some(msg.author.id),
                        msg.guild_id.map(|g| g.get()),
                    )
                    .await
                    .color(hex_string_to_int(&config.thread.system_message_color) as u32)
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;
            };

            if config.logs.show_log_on_edit {
                let message_link = format!(
                    "https://discord.com/channels/{}/{}/{}",
                    config.bot.get_staff_guild_id(),
                    msg.channel_id.get(),
                    inbox_message_id
                );

                let mut params = HashMap::new();
                params.insert(
                    "before".to_string(),
                    if before_content.is_empty() {
                        "(inconnu)".to_string()
                    } else {
                        format!("`{}`", before_content.clone())
                    },
                );
                params.insert(
                    "after".to_string(),
                    format!("`{}`", command_input.new_content.clone()),
                );
                params.insert("link".to_string(), message_link);

                let _ = MessageBuilder::system_message(&ctx, config)
                    .translated_content(
                        "edit.modification_from_staff",
                        Some(&params),
                        Some(msg.author.id),
                        Some(config.bot.get_staff_guild_id()),
                    )
                    .await
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;
            }

            cleanup_command_message(&ctx, &msg).await;

            match update_message_content(&dm_msg_id, &command_input.new_content, pool).await {
                Ok(()) => (),
                Err(e) => return Err(e),
            }

            Ok(())
        }
        Err(e) => Err(e),
    }
}
