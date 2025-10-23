use crate::commands::{BoxFuture, RegistrableCommand};
use crate::config::Config;
use crate::db::reminders::{get_reminder_by_id, update_reminder_status};
use crate::errors::{common, CommandError, DatabaseError, ModmailError, ModmailResult};
use crate::i18n::get_translated_message;
use crate::types::logs::PaginationStore;
use crate::utils::command::defer_response::defer_response;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub struct RemoveReminderCommand;

impl RegistrableCommand for RemoveReminderCommand {
    fn name(&self) -> &'static str {
        "remove_reminder"
    }

    fn register(&self, config: &Config) -> BoxFuture<Vec<CreateCommand>> {
        let config = config.clone();
        let name = self.name();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.remove_reminder_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let id_desc = get_translated_message(
                &config,
                "slash_command.remove_reminder_id_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new(name).description(cmd_desc).add_option(
                CreateCommandOption::new(CommandOptionType::Number, "id", id_desc).required(true),
            )]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        _shutdown: Arc<Receiver<bool>>,
        _pagination: PaginationStore,
    ) -> BoxFuture<ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = config
                .db_pool
                .as_ref()
                .ok_or_else(common::database_connection_failed)?;

            let _ = defer_response(&ctx, &command).await;

            let mut reminder_id: Option<i64> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "id" => {
                        if let CommandDataOptionValue::Number(val) = &option.value {
                            reminder_id.replace(*val as i64);
                        }
                    }
                    _ => {}
                }
            }

            let reminder_id = match reminder_id {
                Some(id) => id,
                None => {
                    return Err(ModmailError::Command(CommandError::InvalidArguments(
                        "reminder ID".to_string(),
                    )));
                }
            };

            let reminder = match get_reminder_by_id(reminder_id, pool).await {
                Ok(Some(r)) => r,
                Ok(None) => {
                    return Err(ModmailError::Database(DatabaseError::NotFound(
                        "".to_string(),
                    )));
                }
                Err(e) => {
                    return Err(ModmailError::Database(DatabaseError::QueryFailed(
                        e.to_string(),
                    )));
                }
            };

            match update_reminder_status(&reminder, true, pool).await {
                Ok(_) => {
                    let mut params = HashMap::new();
                    params.insert("id".to_string(), reminder_id.to_string());

                    let response = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "remove_reminder.confirmation",
                            Some(&params),
                            None,
                            None,
                        )
                        .await
                        .to_channel(command.channel_id)
                        .build_interaction_message_followup()
                        .await;

                    let _ = command.create_followup(&ctx.http, response).await;
                }
                Err(e) => {
                    return Err(ModmailError::Database(DatabaseError::QueryFailed(
                        e.to_string(),
                    )));
                }
            }

            Ok(())
        })
    }
}
