use crate::commands::delete::common::{
    delete_database_message, delete_discord_messages, get_message_ids, get_thread_info,
    update_message_numbers,
};
use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::errors::{common, MessageError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::utils::command::defer_response::defer_response_ephemeral;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;

pub struct DeleteCommand;

#[async_trait::async_trait]
impl RegistrableCommand for DeleteCommand {
    fn name(&self) -> &'static str {
        "delete"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
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
            ]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        options: &[ResolvedOption<'_>],
        config: &Config,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

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
                return Err(ModmailError::Message(MessageError::MessageNotFound(
                    "".to_string(),
                )));
            }

            let message_ids = get_message_ids(user_id, &thread, message_number, pool).await?;

            delete_discord_messages(&ctx, &command.channel_id, user_id, &message_ids).await?;
            delete_database_message(&message_ids, pool).await?;
            update_message_numbers(&thread.channel_id, message_number, pool).await;

            let mut params = HashMap::new();
            params.insert("number".to_string(), message_number.to_string());

            let response = MessageBuilder::system_message(&ctx, &config)
                .translated_content("delete.success", Some(&params), None, None)
                .await
                .to_channel(command.channel_id)
                .ephemeral(true)
                .build_interaction_message_followup()
                .await;

            let _ = command.create_followup(&ctx.http, response).await;

            Ok(())
        })
    }
}
