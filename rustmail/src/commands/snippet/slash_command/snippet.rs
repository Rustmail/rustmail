use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use regex::Regex;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SnippetCommand;

#[async_trait::async_trait]
impl RegistrableCommand for SnippetCommand {
    fn name(&self) -> &'static str {
        "snippet"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move {
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
        .boxed()
    }

    fn register(&self, config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        let name = self.name();
        let config = config.clone();

        Box::pin(async move {
            let cmd_desc = get_translated_message(
                &config,
                "slash_command.snippet_command_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let create_desc = get_translated_message(
                &config,
                "slash_command.snippet_create_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let list_desc = get_translated_message(
                &config,
                "slash_command.snippet_list_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let show_desc = get_translated_message(
                &config,
                "slash_command.snippet_show_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let edit_desc = get_translated_message(
                &config,
                "slash_command.snippet_edit_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let delete_desc = get_translated_message(
                &config,
                "slash_command.snippet_delete_description",
                None,
                None,
                None,
                None,
            )
            .await;
            let key_desc = get_translated_message(
                &config,
                "slash_command.snippet_key_argument",
                None,
                None,
                None,
                None,
            )
            .await;
            let content_desc = get_translated_message(
                &config,
                "slash_command.snippet_content_argument",
                None,
                None,
                None,
                None,
            )
            .await;

            vec![
                CreateCommand::new(name)
                    .description(cmd_desc)
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "create",
                            create_desc.clone(),
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "key",
                                key_desc.clone(),
                            )
                            .required(true),
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "content",
                                content_desc.clone(),
                            )
                            .required(true),
                        ),
                    )
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "list",
                        list_desc,
                    ))
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::SubCommand, "show", show_desc)
                            .add_sub_option(
                                CreateCommandOption::new(
                                    CommandOptionType::String,
                                    "key",
                                    key_desc.clone(),
                                )
                                .required(true),
                            ),
                    )
                    .add_option(
                        CreateCommandOption::new(CommandOptionType::SubCommand, "edit", edit_desc)
                            .add_sub_option(
                                CreateCommandOption::new(
                                    CommandOptionType::String,
                                    "key",
                                    key_desc.clone(),
                                )
                                .required(true),
                            )
                            .add_sub_option(
                                CreateCommandOption::new(
                                    CommandOptionType::String,
                                    "content",
                                    content_desc,
                                )
                                .required(true),
                            ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "delete",
                            delete_desc,
                        )
                        .add_sub_option(
                            CreateCommandOption::new(CommandOptionType::String, "key", key_desc)
                                .required(true),
                        ),
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
        _handler: Arc<crate::prelude::handlers::InteractionHandler>,
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

            let subcommand = &command.data.options[0];
            let subcommand_name = &subcommand.name;

            match subcommand_name.as_str() {
                "create" => {
                    handle_create(&ctx, &command, &command.data.options, pool, &config).await
                }
                "list" => handle_list(&ctx, &command, pool, &config).await,
                "show" => handle_show(&ctx, &command, &command.data.options, pool, &config).await,
                "edit" => handle_edit(&ctx, &command, &command.data.options, pool, &config).await,
                "delete" => {
                    handle_delete(&ctx, &command, &command.data.options, pool, &config).await
                }
                _ => {
                    let response = MessageBuilder::system_message(&ctx, &config)
                        .translated_content(
                            "snippet.unknown_subcommand",
                            None,
                            Some(command.user.id),
                            command.guild_id.map(|g| g.get()),
                        )
                        .await
                        .to_channel(command.channel_id)
                        .build_interaction_message_followup()
                        .await;

                    command.create_followup(&ctx.http, response).await?;

                    Ok(())
                }
            }
        })
    }
}

async fn handle_create(
    ctx: &Context,
    command: &CommandInteraction,
    options: &Vec<CommandDataOption>,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut key = String::new();
    let mut content = String::new();

    if let Some(subcommand) = options.first() {
        if let CommandDataOptionValue::SubCommand(sub_options) = &subcommand.value {
            for option in sub_options {
                match option.name.as_str() {
                    "key" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            key = val.to_string();
                        }
                    }
                    "content" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            content = val.to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    let key_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !key_regex.is_match(&key) {
        return Err(ModmailError::Command(CommandError::InvalidSnippetKeyFormat));
    }

    if content.len() > 4000 {
        return Err(ModmailError::Command(CommandError::SnippetContentTooLong));
    }

    let created_by = command.user.id.to_string();
    match create_snippet(&key, &content, &created_by, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetAlreadyExists(
                key.to_string(),
            )));
        }
    }

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.clone());

    let response = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.created",
            Some(&params),
            Some(command.user.id),
            command.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(command.channel_id)
        .build_interaction_message_followup()
        .await;

    command.create_followup(&ctx.http, response).await?;

    Ok(())
}

async fn handle_list(
    ctx: &Context,
    command: &CommandInteraction,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let snippets = get_all_snippets(pool).await?;

    if snippets.is_empty() {
        let response = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.no_snippets_found",
                None,
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(command.channel_id)
            .build_interaction_message_followup()
            .await;

        command.create_followup(&ctx.http, response).await?;
        return Ok(());
    }

    let title = get_translated_message(
        config,
        "snippet.list_title",
        None,
        Some(command.user.id),
        command.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let mut content = format!("**{}**\n\n", title);
    for (index, snippet) in snippets.iter().enumerate() {
        content.push_str(&format!("`{}` - {}\n\n", index + 1, snippet.key));
    }

    let response = MessageBuilder::system_message(ctx, config)
        .content(content)
        .to_channel(command.channel_id)
        .build_interaction_message_followup()
        .await;

    command.create_followup(&ctx.http, response).await?;

    Ok(())
}

async fn handle_show(
    ctx: &Context,
    command: &CommandInteraction,
    options: &Vec<CommandDataOption>,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut key = String::new();

    if let Some(subcommand) = options.first() {
        if let CommandDataOptionValue::SubCommand(sub_options) = &subcommand.value {
            for option in sub_options {
                if option.name == "key" {
                    if let CommandDataOptionValue::String(val) = &option.value {
                        key = val.to_string();
                    }
                }
            }
        }
    }

    match get_snippet_by_key(&key, pool).await? {
        Some(snippet) => {
            let title = get_translated_message(
                config,
                "snippet.show_title",
                Some(&HashMap::from([("key".to_string(), snippet.key.clone())])),
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let created_by_label = get_translated_message(
                config,
                "snippet.created_by",
                None,
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let created_at_label = get_translated_message(
                config,
                "snippet.created_at",
                None,
                Some(command.user.id),
                command.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let content = format!(
                "**{}**\n\n{}\n\n*{}: <@{}> | {}: {}*",
                title,
                snippet.content,
                created_by_label,
                snippet.created_by,
                created_at_label,
                snippet.created_at
            );

            let response = MessageBuilder::system_message(ctx, config)
                .content(content)
                .to_channel(command.channel_id)
                .build_interaction_message_followup()
                .await;

            command.create_followup(&ctx.http, response).await?;
        }
        None => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    }

    Ok(())
}

async fn handle_edit(
    ctx: &Context,
    command: &CommandInteraction,
    options: &Vec<CommandDataOption>,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut key = String::new();
    let mut content = String::new();

    if let Some(subcommand) = options.first() {
        if let CommandDataOptionValue::SubCommand(sub_options) = &subcommand.value {
            for option in sub_options {
                match option.name.as_str() {
                    "key" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            key = val.to_string();
                        }
                    }
                    "content" => {
                        if let CommandDataOptionValue::String(val) = &option.value {
                            content = val.to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if content.len() > 4000 {
        return Err(ModmailError::Command(CommandError::SnippetContentTooLong));
    }

    match update_snippet(&key, &content, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    };

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.clone());

    let response = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.updated",
            Some(&params),
            Some(command.user.id),
            command.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(command.channel_id)
        .build_interaction_message_followup()
        .await;

    command.create_followup(&ctx.http, response).await?;

    Ok(())
}

async fn handle_delete(
    ctx: &Context,
    command: &CommandInteraction,
    options: &Vec<CommandDataOption>,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut key = String::new();

    if let Some(subcommand) = options.first() {
        if let CommandDataOptionValue::SubCommand(sub_options) = &subcommand.value {
            for option in sub_options {
                if option.name == "key" {
                    if let CommandDataOptionValue::String(val) = &option.value {
                        key = val.to_string();
                    }
                }
            }
        }
    }

    match delete_snippet(&key, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    };

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.clone());

    let response = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.deleted",
            Some(&params),
            Some(command.user.id),
            command.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(command.channel_id)
        .build_interaction_message_followup()
        .await;

    command.create_followup(&ctx.http, response).await?;

    Ok(())
}
