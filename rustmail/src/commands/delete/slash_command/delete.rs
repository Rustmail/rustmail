use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, CommandType, Context,
    CreateCommand, CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct DeleteCommand;

#[async_trait::async_trait]
impl RegistrableCommand for DeleteCommand {
    fn name(&self) -> &'static str {
        "delete"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.delete", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.delete_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let message_id_desc = get_translated_message(
                &config,
                "slash_command.delete_message_id_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("delete")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::Number,
                            "message_id",
                            message_id_desc,
                        )
                        .required(true),
                    ),
                CreateCommand::new("delete").kind(CommandType::Message),
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
                .ok_or_else(database_connection_failed)?;

            defer_response_ephemeral(&ctx, &command).await?;

            let mut message_number: i64 = -1;

            for option in &command.data.options {
                match option.name.as_str() {
                    "message_id" => {
                        if let CommandDataOptionValue::Number(val) = &option.value {
                            message_number = *val as i64;
                        }
                    }
                    _ => {}
                }
            }

            let (user_id, thread) = get_thread_info(&command.channel_id.to_string(), pool).await?;

            if message_number < 0 {
                if let Some(message_id) = &command.data.target_id {
                    let thread_message = match get_thread_message_by_message_id(
                        &message_id.to_message_id().to_string(),
                        pool,
                    )
                    .await
                    {
                        Ok(thread_message) => thread_message,
                        Err(e) => {
                            return Err(ModmailError::Message(MessageError::MessageNotFound(
                                e.to_string(),
                            )));
                        }
                    };
                    if let Some(msg_number) = thread_message.message_number {
                        message_number = msg_number;
                    } else {
                        return Err(ModmailError::Message(MessageError::MessageNotFound(
                            "".to_string(),
                        )));
                    }
                } else {
                    return Err(ModmailError::Message(MessageError::MessageNotFound(
                        "".to_string(),
                    )));
                }
            }

            let message_ids =
                get_message_ids_for_delete(user_id, &thread, message_number, pool).await?;

            delete_discord_messages(&ctx, &command.channel_id, user_id, &message_ids).await?;
            delete_database_message(&message_ids, pool).await?;
            update_message_numbers(&thread.channel_id, message_number, pool).await;

            let mut params = HashMap::new();
            params.insert("number".to_string(), message_number.to_string());

            let _ = MessageBuilder::system_message(&ctx, &config)
                .translated_content("delete.success", Some(&params), None, None)
                .await
                .to_channel(command.channel_id)
                .ephemeral(true)
                .send_interaction_followup(&command, true)
                .await;

            Ok(())
        })
    }
}
