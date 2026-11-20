pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{
    ChannelId, ChannelType, CommandOptionType, CommandType, CreateChannel, CreateCommand,
    CreateCommandOption, GuildId, UserId,
};
use std::collections::HashMap;

pub struct NewThreadCommand;

impl Command for NewThreadCommand {
    type Args = UserId;

    fn name(&self) -> &'static str {
        "new_thread"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["nt"]
    }

    fn community(&self) -> bool {
        true
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.new_thread", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("new_thread")
                .description(translate("slash_command.new_thread_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user_id",
                        translate("slash_command.new_thread_user_id_argument").await,
                    )
                    .required(true),
                ),
            CreateCommand::new("new_thread").kind(CommandType::User),
        ]
    }

    async fn community_definitions(&self, _config: &Config) -> Vec<CreateCommand> {
        vec![CreateCommand::new("new_thread").kind(CommandType::User)]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<UserId> {
        parse_new_thread_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<UserId> {
        parse_new_thread_slash(options)
    }

    async fn run(
        &self,
        user_id: UserId,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let ctx = invocation.ctx();

        let user = user_id
            .to_user(ctx)
            .await
            .map_err(|_| ModmailError::Discord(DiscordError::UserNotFound))?;
        if user.bot {
            return Err(ModmailError::Discord(DiscordError::UserIsABot));
        }
        if let Some(channel_id) = get_thread_channel_by_user_id(user_id, pool).await {
            return Err(ModmailError::Command(
                CommandError::UserHasAlreadyAThreadWithLink(user.name.clone(), channel_id),
            ));
        }

        let channel = GuildId::new(config.bot.get_staff_guild_id())
            .create_channel(
                &ctx.http,
                CreateChannel::new(format!("🔴・{}・0m", user.name))
                    .kind(ChannelType::Text)
                    .category(ChannelId::new(config.thread.inbox_category_id)),
            )
            .await
            .map_err(|_| ModmailError::Discord(DiscordError::ChannelCreationFailed))?;

        if let Err(e) =
            create_thread_for_user(&channel, user_id.get() as i64, &user.name, true, pool).await
        {
            let _ = channel.delete(&ctx.http).await;
            return Err(ModmailError::Database(DatabaseError::InsertFailed(
                e.to_string(),
            )));
        }

        post_ticket_recap(ctx, config, channel.id, user_id, &user.name).await;

        send_ticket_welcome(ctx, config, &channel, &user, false).await;

        let params = HashMap::from([
            ("user".to_string(), user.name),
            ("channel_id".to_string(), channel.to_string()),
            ("staff".to_string(), invocation.user().name.clone()),
        ]);
        invocation
            .reply_translated(config, "new_thread.success_without_dm", Some(&params))
            .await?;
        Ok(())
    }
}
