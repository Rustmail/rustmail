use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOptionValue, CommandInteraction, CommandOptionType, Context, CreateCommand,
    CreateCommandOption, ResolvedOption,
};
use std::sync::Arc;

pub struct AlertCommand;

#[async_trait::async_trait]
impl RegistrableCommand for AlertCommand {
    fn name(&self) -> &'static str {
        "alert"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.alert", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.alert_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let cancel_desc = get_translated_message(
                &config,
                "slash_command.alert_cancel_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new("alert")
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::Boolean, "cancel", cancel_desc)
                            .required(false),
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
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let user_id = get_thread_user_id_from_command(&ctx, &command, &config, pool).await?;

            let is_cancel = match command.data.options.iter().find(|opt| opt.name == "cancel") {
                Some(opt) => match &opt.value {
                    CommandDataOptionValue::Boolean(cancel) => *cancel,
                    _ => {
                        return Err(ModmailError::Command(CommandError::InvalidArguments(
                            "user_id".to_string(),
                        )));
                    }
                },
                None => false,
            };

            if is_cancel {
                handle_cancel_alert_from_command(&ctx, &command, &config, user_id, pool).await
            } else {
                handle_set_alert_from_command(&ctx, &command, &config, user_id, pool).await
            }
        })
    }
}
