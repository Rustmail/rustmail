use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct RemoveReminderCommand;

impl RegistrableCommand for RemoveReminderCommand {
    fn name(&self) -> &'static str {
        "unremind"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
            get_translated_message(config, "help.remove_reminder", None, None, None, None).await
        }
        .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
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
                Ok(Some(r)) => {
                    if r.completed {
                        return Err(ModmailError::Command(
                            CommandError::ReminderAlreadyCompleted(reminder_id.to_string()),
                        ));
                    } else {
                        r
                    }
                }
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

                    let _ = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "remove_reminder.confirmation",
                            Some(&params),
                            None,
                            None,
                        )
                        .await
                        .to_channel(command.channel_id)
                        .send_interaction_followup(&command, true)
                        .await;
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
