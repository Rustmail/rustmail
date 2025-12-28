use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption, UserId,
};
use std::sync::Arc;

pub struct LogsCommand;

impl RegistrableCommand for LogsCommand {
    fn name(&self) -> &'static str {
        "logs"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.logs", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_commands.logs_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let id_desc = get_translated_message(
                &config,
                "slash_commands.logs_id_argument_description",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![CreateCommand::new("logs").description(cmd_desc).add_option(
                CreateCommandOption::new(CommandOptionType::User, "id", id_desc).required(false),
            )]
        })
    }

    fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction,
        _options: &[ResolvedOption<'_>],
        config: &Config,
        handler: Arc<InteractionHandler>,
    ) -> BoxFuture<'_, ModmailResult<()>> {
        let ctx = ctx.clone();
        let command = command.clone();
        let config = config.clone();

        Box::pin(async move {
            let pool = match config.db_pool.clone() {
                Some(pool) => pool.clone(),
                None => return Err(ModmailError::Database(DatabaseError::ConnectionFailed)),
            };

            defer_response(&ctx, &command).await?;

            let mut user_id: Option<UserId> = None;

            for option in &command.data.options {
                match option.name.as_str() {
                    "id" => {
                        if let CommandDataOptionValue::User(val) = &option.value {
                            user_id.replace(val.clone());
                        }
                    }
                    _ => {}
                }
            }

            if !user_id.is_some() {
                handle_logs_in_thread(
                    &ctx,
                    &command.clone().channel_id,
                    Some(command.clone()),
                    &config,
                    &pool,
                    handler.pagination.clone(),
                )
                .await
            } else {
                handle_logs_from_user_id(
                    &ctx,
                    &command.clone().channel_id,
                    Some(command),
                    &config,
                    &pool,
                    &user_id.unwrap().to_string(),
                    handler.pagination.clone(),
                )
                .await
            }
        })
    }
}
