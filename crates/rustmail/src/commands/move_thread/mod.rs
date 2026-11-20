pub mod args;
pub mod matching;

pub use args::*;
pub use matching::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{
    ChannelId, ChannelType, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    EditChannel, GuildId,
};
use serenity::http::HttpError;
use std::collections::HashMap;

pub struct MoveCommand;

impl Command for MoveCommand {
    type Args = MoveArgs;

    fn name(&self) -> &'static str {
        "move"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["mv"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.move", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("move")
                .description(translate("slash_command.move_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Channel,
                        "category",
                        translate("slash_command.move_command_name_argument").await,
                    )
                    .channel_types(vec![ChannelType::Category])
                    .required(true),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<MoveArgs> {
        parse_move_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<MoveArgs> {
        parse_move_slash(options)
    }

    async fn run(
        &self,
        args: MoveArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();
        current_ticket(invocation, pool).await?;

        let categories = staff_categories(ctx, config).await;
        if categories.is_empty() {
            return Err(ModmailError::Discord(DiscordError::FailedToFetchCategories));
        }
        let (category_id, category_name) = match args {
            MoveArgs::Category(id) => categories.into_iter().find(|(category, _)| *category == id),
            MoveArgs::Named(name) => find_best_match_category(&name, &categories),
        }
        .ok_or(ModmailError::Thread(ThreadError::CategoryNotFound))?;

        let permissions = get_category_permissions_overwrites(ctx, category_id).await?;
        invocation
            .channel_id()
            .edit(
                &ctx.http,
                EditChannel::new()
                    .category(category_id)
                    .permissions(permissions),
            )
            .await
            .map_err(move_error)?;

        let params = HashMap::from([
            ("category".to_string(), category_name),
            ("staff".to_string(), invocation.user_id().to_string()),
        ]);
        invocation
            .reply_translated(config, "move_thread.success", Some(&params))
            .await?;

        if let Some(message) = invocation.message() {
            let _ = message.delete(&ctx.http).await;
        }
        Ok(())
    }
}

async fn staff_categories(ctx: &Context, config: &Config) -> Vec<(ChannelId, String)> {
    let staff_guild = GuildId::new(config.bot.get_staff_guild_id());
    match staff_guild.channels(&ctx.http).await {
        Ok(channels) => channels
            .into_iter()
            .filter(|(_, channel)| channel.kind == ChannelType::Category)
            .map(|(id, channel)| (id, channel.name))
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn move_error(err: serenity::Error) -> ModmailError {
    if let serenity::Error::Http(HttpError::UnsuccessfulRequest(ref response)) = err
        && matches!(response.error.code, 30013 | 30030)
    {
        return ModmailError::Discord(DiscordError::CategoryFull);
    }
    ModmailError::from(err)
}
