pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use crate::types::snippet::Snippet;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use std::collections::HashMap;

pub struct SnippetCommand;

impl Command for SnippetCommand {
    type Args = SnippetArgs;

    fn name(&self) -> &'static str {
        "snippet"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["s"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(
            config,
            "slash_command.snippet_command_help",
            None,
            None,
            None,
            None,
        )
        .await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;
        let key_option = CreateCommandOption::new(
            CommandOptionType::String,
            "key",
            translate("slash_command.snippet_key_argument").await,
        )
        .required(true);
        let content_option = CreateCommandOption::new(
            CommandOptionType::String,
            "content",
            translate("slash_command.snippet_content_argument").await,
        )
        .required(true);
        let subcommand = async |name, key| {
            CreateCommandOption::new(CommandOptionType::SubCommand, name, translate(key).await)
        };

        vec![
            CreateCommand::new("snippet")
                .description(translate("slash_command.snippet_command_description").await)
                .add_option(
                    subcommand("create", "slash_command.snippet_create_description")
                        .await
                        .add_sub_option(key_option.clone())
                        .add_sub_option(content_option.clone()),
                )
                .add_option(subcommand("list", "slash_command.snippet_list_description").await)
                .add_option(
                    subcommand("show", "slash_command.snippet_show_description")
                        .await
                        .add_sub_option(key_option.clone()),
                )
                .add_option(
                    subcommand("edit", "slash_command.snippet_edit_description")
                        .await
                        .add_sub_option(key_option.clone())
                        .add_sub_option(content_option),
                )
                .add_option(
                    subcommand("delete", "slash_command.snippet_delete_description")
                        .await
                        .add_sub_option(key_option.clone()),
                )
                .add_option(
                    subcommand("use", "slash_command.snippet_use_description")
                        .await
                        .add_sub_option(key_option),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<SnippetArgs> {
        parse_snippet_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<SnippetArgs> {
        parse_snippet_slash(options)
    }

    async fn run(
        &self,
        args: SnippetArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let key_param = |key: &str| HashMap::from([("key".to_string(), key.to_string())]);

        match args {
            SnippetArgs::Create { key, content } => {
                create_snippet(&key, &content, &invocation.user_id().to_string(), pool)
                    .await
                    .map_err(|_| {
                        ModmailError::Command(CommandError::SnippetAlreadyExists(key.clone()))
                    })?;
                invocation
                    .reply_translated(config, "snippet.created", Some(&key_param(&key)))
                    .await?;
            }
            SnippetArgs::List => {
                let snippets = get_all_snippets(pool).await?;
                if snippets.is_empty() {
                    invocation
                        .reply_translated(config, "snippet.list_empty", None)
                        .await?;
                    return Ok(());
                }
                let title = self
                    .translate(config, invocation, "snippet.list_title", None)
                    .await;
                let mut list = format!("**{title}**\n\n");
                for (index, snippet) in snippets.iter().enumerate() {
                    list.push_str(&format!("`{}` - {}\n\n", index + 1, snippet.key));
                }
                self.reply_content(invocation, config, list).await?;
            }
            SnippetArgs::Show { key } => {
                let snippet = find_snippet(&key, pool).await?;
                let title = self
                    .translate(
                        config,
                        invocation,
                        "snippet.show_title",
                        Some(&key_param(&snippet.key)),
                    )
                    .await;
                let created_by = self
                    .translate(config, invocation, "snippet.created_by", None)
                    .await;
                let created_at = self
                    .translate(config, invocation, "snippet.created_at", None)
                    .await;
                let content = format!(
                    "**{title}**\n\n{}\n\n*{created_by}: <@{}> | {created_at}: {}*",
                    snippet.content, snippet.created_by, snippet.created_at
                );
                self.reply_content(invocation, config, content).await?;
            }
            SnippetArgs::Edit { key, content } => {
                update_snippet(&key, &content, pool).await.map_err(|_| {
                    ModmailError::Command(CommandError::SnippetNotFound(key.clone()))
                })?;
                invocation
                    .reply_translated(config, "snippet.updated", Some(&key_param(&key)))
                    .await?;
            }
            SnippetArgs::Delete { key } => {
                delete_snippet(&key, pool).await.map_err(|_| {
                    ModmailError::Command(CommandError::SnippetNotFound(key.clone()))
                })?;
                invocation
                    .reply_translated(config, "snippet.deleted", Some(&key_param(&key)))
                    .await?;
            }
            SnippetArgs::Use { key } => {
                let in_ticket =
                    get_thread_by_channel_id(&invocation.channel_id().to_string(), pool)
                        .await
                        .is_some();
                if in_ticket {
                    let reply = ReplyArgs {
                        snippet: Some(key),
                        ..ReplyArgs::default()
                    };
                    return send_staff_reply(reply, invocation, config).await;
                }
                let snippet = find_snippet(&key, pool).await?;
                self.reply_content(invocation, config, snippet.content)
                    .await?;
            }
        }
        Ok(())
    }
}

impl SnippetCommand {
    async fn translate(
        &self,
        config: &Config,
        invocation: &Invocation,
        key: &str,
        params: Option<&HashMap<String, String>>,
    ) -> String {
        get_translated_message(
            config,
            key,
            params,
            Some(invocation.user_id()),
            invocation.guild_id().map(|g| g.get()),
            None,
        )
        .await
    }

    async fn reply_content(
        &self,
        invocation: &Invocation,
        config: &Config,
        content: String,
    ) -> ModmailResult<()> {
        invocation
            .reply(MessageBuilder::system_message(invocation.ctx(), config).content(content))
            .await?;
        Ok(())
    }
}

async fn find_snippet(key: &str, pool: &sqlx::SqlitePool) -> ModmailResult<Snippet> {
    get_snippet_by_key(key, pool)
        .await?
        .ok_or_else(|| ModmailError::Command(CommandError::SnippetNotFound(key.to_string())))
}
