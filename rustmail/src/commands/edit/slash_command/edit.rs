use crate::commands::edit::message_ops::{edit_messages, format_new_message, get_message_ids};
use crate::commands::edit::validation::validate_edit_permissions;
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::{get_thread_message_by_inbox_message_id, update_message_content};
use crate::errors::common::message_not_found;
use crate::errors::{common, ModmailResult};
use crate::handlers::guild_interaction_handler::InteractionHandler;
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response;
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct EditCommand;

#[async_trait::async_trait]
impl RegistrableCommand for EditCommand {
    fn name(&self) -> &'static str {
        "edit"
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.edit_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let msg_id_desc = get_translated_message(
                &config,
                "slash_command.edit_message_id_argument",
                None,
                None,
                None,
                None,
            )
            .await;
            let edit_content_desc = get_translated_message(
                &config,
                "slash_command.edit_message_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("edit")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::Integer,
                            "message_id",
                            msg_id_desc,
                        )
                        .required(true),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::String,
                            "message",
                            edit_content_desc,
                        )
                        .required(true),
                    ),
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut msg_id: i64 = 0;
            let mut new_content: String = String::new();

            for option in command.data.options.iter() {
                match option.name.as_str() {
                    "message_id" => {
                        if let CommandDataOptionValue::Integer(value) = &option.value {
                            msg_id = *value;
                        }
                    }
                    "message" => {
                        if let CommandDataOptionValue::String(value) = &option.value {
                            new_content = value.clone();
                        }
                    }
                    _ => {}
                }
            }

            match validate_edit_permissions(msg_id, command.channel_id, command.user.id, pool).await
            {
                Ok(()) => (),
                Err(e) => return Err(e),
            };

            let ids = match get_message_ids(msg_id, command.user.id, pool, &ctx, command.channel_id)
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
                (None, Some(&command)),
                &new_content,
                &inbox_message_id,
                msg_id as u64,
                &config,
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
                command.channel_id,
                dm_msg_id.clone(),
                inbox_message_id.clone(),
                edited_messages_builder,
                pool,
                &config,
            )
            .await;

            match edit_result {
                Ok(()) => {
                    if config.notifications.show_success_on_edit {
                        let response = MessageBuilder::system_message(&ctx, &config)
                            .translated_content(
                                "success.message_edited",
                                None,
                                Some(command.user.id),
                                command.guild_id.map(|g| g.get()),
                            )
                            .await
                            .color(hex_string_to_int(&config.thread.system_message_color) as u32)
                            .to_channel(command.channel_id)
                            .build_interaction_message_followup()
                            .await;

                        let _ = command.create_followup(&ctx.http, response).await;
                    };

                    if config.logs.show_log_on_edit {
                        let message_link = format!(
                            "https://discord.com/channels/{}/{}/{}",
                            config.bot.get_staff_guild_id(),
                            command.channel_id.get(),
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
                        params.insert("after".to_string(), format!("`{}`", new_content.clone()));
                        params.insert("link".to_string(), message_link);

                        let response = MessageBuilder::system_message(&ctx, &config)
                            .translated_content(
                                "edit.modification_from_staff",
                                Some(&params),
                                Some(command.user.id),
                                Some(config.bot.get_staff_guild_id()),
                            )
                            .await
                            .to_channel(command.channel_id)
                            .build_interaction_message_followup()
                            .await;

                        let _ = command.create_followup(&ctx.http, response).await;
                    }

                    match update_message_content(&dm_msg_id, &new_content, pool).await {
                        Ok(()) => (),
                        Err(e) => return Err(e),
                    }

                    Ok(())
                }
                Err(e) => Err(e),
            }
        })
    }
}
