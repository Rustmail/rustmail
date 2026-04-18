use crate::commands::baninfo::text_command::{resolve_banned_users, send_baninfo_response};
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

pub struct BaninfoCommand;

#[async_trait::async_trait]
impl RegistrableCommand for BaninfoCommand {
    fn name(&self) -> &'static str {
        "baninfo"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.baninfo", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.baninfo_command_description",
                None,
                None,
                None,
                None,
            )
            .await;

            let query_desc = get_translated_message(
                &config,
                "slash_command.baninfo_query_option",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(self.name())
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::String, "query", query_desc)
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
                .ok_or_else(database_connection_failed)?;

            defer_response(&ctx, &command).await?;

            let mut query: Option<String> = None;
            for option in &command.data.options {
                if option.name.as_str() == "query" {
                    if let CommandDataOptionValue::String(val) = &option.value {
                        let trimmed = val.trim();
                        if !trimmed.is_empty() {
                            query = Some(trimmed.to_string());
                        }
                    }
                }
            }

            let query =
                query.ok_or_else(|| ModmailError::Command(CommandError::MissingArguments))?;
            let guild_id = config.bot.get_community_guild_id().to_string();
            let matches = resolve_banned_users(&guild_id, &query, pool).await?;

            send_baninfo_response(&ctx, &config, command.channel_id, &query, &matches).await;
            Ok(())
        })
    }
}
