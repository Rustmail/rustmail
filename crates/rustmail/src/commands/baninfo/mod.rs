pub mod args;
pub mod embed;

pub use args::*;
pub use embed::*;

use crate::db::repr::BannedUser;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use sqlx::SqlitePool;
use std::collections::HashMap;

pub struct BaninfoCommand;

impl Command for BaninfoCommand {
    type Args = BaninfoArgs;

    fn name(&self) -> &'static str {
        "baninfo"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["bi"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.baninfo", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("baninfo")
                .description(translate("slash_command.baninfo_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "query",
                        translate("slash_command.baninfo_query_option").await,
                    )
                    .required(true),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<BaninfoArgs> {
        parse_baninfo_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<BaninfoArgs> {
        parse_baninfo_slash(options)
    }

    async fn run(
        &self,
        args: BaninfoArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;

        let guild_id = config.bot.get_community_guild_id().to_string();
        let matches = resolve_banned_users(&guild_id, &args.query, pool).await?;
        let builder = MessageBuilder::system_message(invocation.ctx(), config);

        match matches.as_slice() {
            [] => {
                let params = HashMap::from([("query".to_string(), args.query)]);
                invocation
                    .reply_translated(config, "baninfo.not_found", Some(&params))
                    .await?;
            }
            [user] => {
                let embed = build_baninfo_embed(invocation.ctx(), config, user).await;
                invocation.reply(builder.build_embed_only(embed)).await?;
            }
            users => {
                let header = get_translated_message(
                    config,
                    "baninfo.multiple_matches",
                    None,
                    None,
                    None,
                    None,
                )
                .await;
                let lines: String = users
                    .iter()
                    .map(|user| {
                        format!(
                            "• **{}** — `{}` (<@{}>)\n",
                            user.username, user.user_id, user.user_id
                        )
                    })
                    .collect();
                invocation
                    .reply(builder.content(format!("{header}\n{lines}")))
                    .await?;
            }
        }
        Ok(())
    }
}

async fn resolve_banned_users(
    guild_id: &str,
    query: &str,
    pool: &SqlitePool,
) -> ModmailResult<Vec<BannedUser>> {
    let normalized = normalize_banned_user_query(query);
    if let Ok(user_id) = normalized.parse::<u64>()
        && let Some(user) = get_banned_user_by_id(guild_id, &user_id.to_string(), pool).await?
    {
        return Ok(vec![user]);
    }
    find_banned_users_by_username(guild_id, normalized, pool).await
}
