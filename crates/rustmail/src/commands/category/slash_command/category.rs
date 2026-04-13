use crate::db::operations::ticket_categories::CATEGORY_BUTTON_HARD_LIMIT;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::FutureExt;
use serenity::all::{
    CommandDataOption, CommandDataOptionValue, CommandInteraction, CommandOptionType, Context,
    CreateCommand, CreateCommandOption, ResolvedOption,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct CategoryCommand;

#[async_trait::async_trait]
impl RegistrableCommand for CategoryCommand {
    fn name(&self) -> &'static str {
        "category"
    }

    fn doc<'a>(&self, config: &'a Config) -> BoxFuture<'a, String> {
        async move { get_translated_message(config, "help.category", None, None, None, None).await }
            .boxed()
    }

    fn register(&self, _config: &Config) -> BoxFuture<'_, Vec<CreateCommand>> {
        Box::pin(async move {
            vec![
                CreateCommand::new("category")
                    .description("Manage ticket categories")
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "create",
                            "Create a new category",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "discord_category_id",
                                "Discord category channel ID",
                            )
                            .required(true),
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "name",
                                "Category name",
                            )
                            .required(true),
                        )
                        .add_sub_option(CreateCommandOption::new(
                            CommandOptionType::String,
                            "description",
                            "Category description",
                        ))
                        .add_sub_option(CreateCommandOption::new(
                            CommandOptionType::String,
                            "emoji",
                            "Button emoji",
                        )),
                    )
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "list",
                        "List all categories",
                    ))
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "delete",
                            "Delete a category",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "name",
                                "Category name",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "rename",
                            "Rename a category",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "old_name",
                                "Current name",
                            )
                            .required(true),
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "new_name",
                                "New name",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "move",
                            "Move a category position",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "name",
                                "Category name",
                            )
                            .required(true),
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::Integer,
                                "position",
                                "New position",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "enable",
                            "Enable a category",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "name",
                                "Category name",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "disable",
                            "Disable a category",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::String,
                                "name",
                                "Category name",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(
                        CreateCommandOption::new(
                            CommandOptionType::SubCommand,
                            "timeout",
                            "Set selection timeout in seconds",
                        )
                        .add_sub_option(
                            CreateCommandOption::new(
                                CommandOptionType::Integer,
                                "seconds",
                                "Seconds",
                            )
                            .required(true),
                        ),
                    )
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "on",
                        "Enable category selection feature",
                    ))
                    .add_option(CreateCommandOption::new(
                        CommandOptionType::SubCommand,
                        "off",
                        "Disable category selection feature",
                    )),
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

            let subcommand = match command.data.options.first() {
                Some(s) => s,
                None => return Ok(()),
            };

            match subcommand.name.as_str() {
                "create" => sub_create(&ctx, &command, &command.data.options, pool, &config).await,
                "list" => sub_list(&ctx, &command, pool, &config).await,
                "delete" => sub_delete(&ctx, &command, &command.data.options, pool, &config).await,
                "rename" => sub_rename(&ctx, &command, &command.data.options, pool, &config).await,
                "move" => sub_move(&ctx, &command, &command.data.options, pool, &config).await,
                "enable" => {
                    sub_enable_one(&ctx, &command, &command.data.options, pool, &config, true).await
                }
                "disable" => {
                    sub_enable_one(&ctx, &command, &command.data.options, pool, &config, false)
                        .await
                }
                "timeout" => {
                    sub_timeout(&ctx, &command, &command.data.options, pool, &config).await
                }
                "on" => sub_feature(&ctx, &command, pool, &config, true).await,
                "off" => sub_feature(&ctx, &command, pool, &config, false).await,
                _ => reply(&ctx, &command, &config, "category.unknown_subcommand", None).await,
            }
        })
    }
}

fn get_string(opts: &[CommandDataOption], key: &str) -> Option<String> {
    let sub = opts.first()?;
    if let CommandDataOptionValue::SubCommand(sub_opts) = &sub.value {
        for o in sub_opts {
            if o.name == key {
                if let CommandDataOptionValue::String(s) = &o.value {
                    return Some(s.clone());
                }
            }
        }
    }
    None
}

fn get_int(opts: &[CommandDataOption], key: &str) -> Option<i64> {
    let sub = opts.first()?;
    if let CommandDataOptionValue::SubCommand(sub_opts) = &sub.value {
        for o in sub_opts {
            if o.name == key {
                if let CommandDataOptionValue::Integer(v) = &o.value {
                    return Some(*v);
                }
            }
        }
    }
    None
}

async fn reply(
    ctx: &Context,
    command: &CommandInteraction,
    config: &Config,
    key: &str,
    params: Option<HashMap<String, String>>,
) -> ModmailResult<()> {
    let mut p = params.unwrap_or_default();
    p.insert("prefix".to_string(), config.command.prefix.clone());
    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(
            key,
            Some(&p),
            Some(command.user.id),
            command.guild_id.map(|g| g.get()),
        )
        .await
        .to_channel(command.channel_id)
        .send_interaction_followup(command, true)
        .await;
    Ok(())
}

async fn sub_create(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let discord_id = match get_string(options, "discord_category_id") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.create_usage", None).await,
    };
    let name = match get_string(options, "name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.create_usage", None).await,
    };
    let description = get_string(options, "description");
    let emoji = get_string(options, "emoji");

    if discord_id.parse::<u64>().is_err() {
        return reply(
            ctx,
            command,
            config,
            "category.invalid_discord_category",
            None,
        )
        .await;
    }

    let enabled = count_enabled_categories(pool).await?;
    if enabled as usize >= CATEGORY_BUTTON_HARD_LIMIT {
        let mut params = HashMap::new();
        params.insert("max".to_string(), CATEGORY_BUTTON_HARD_LIMIT.to_string());
        return reply(
            ctx,
            command,
            config,
            "category.too_many_enabled",
            Some(params),
        )
        .await;
    }

    if get_category_by_name(&name, pool).await?.is_some() {
        return reply(ctx, command, config, "category.already_exists", None).await;
    }

    let created = create_category(
        &name,
        description.as_deref(),
        emoji.as_deref(),
        &discord_id,
        pool,
    )
    .await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), created.name);
    reply(ctx, command, config, "category.created", Some(params)).await
}

async fn sub_list(
    ctx: &Context,
    command: &CommandInteraction,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let cats = list_all_categories(pool).await?;
    if cats.is_empty() {
        return reply(ctx, command, config, "category.list_empty", None).await;
    }
    let header = get_translated_message(
        config,
        "category.list_header",
        None,
        Some(command.user.id),
        command.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    let enabled_label = get_translated_message(
        config,
        "category.state_enabled",
        None,
        Some(command.user.id),
        command.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    let disabled_label = get_translated_message(
        config,
        "category.state_disabled",
        None,
        Some(command.user.id),
        command.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    let mut body = format!("**{}**\n\n", header);
    for cat in &cats {
        let state = if cat.enabled {
            enabled_label.clone()
        } else {
            disabled_label.clone()
        };
        let emoji = cat.emoji.clone().unwrap_or_default();
        body.push_str(&format!(
            "`{}` {} **{}** — {}\n",
            cat.position, emoji, cat.name, state
        ));
    }
    let _ = MessageBuilder::system_message(ctx, config)
        .content(body)
        .to_channel(command.channel_id)
        .send_interaction_followup(command, true)
        .await;
    Ok(())
}

async fn sub_delete(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let name = match get_string(options, "name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let cat = match get_category_by_name(&name, pool).await? {
        Some(c) => c,
        None => return reply(ctx, command, config, "category.not_found", None).await,
    };
    delete_category(&cat.id, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    reply(ctx, command, config, "category.deleted", Some(params)).await
}

async fn sub_rename(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let old = match get_string(options, "old_name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let new = match get_string(options, "new_name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let cat = match get_category_by_name(&old, pool).await? {
        Some(c) => c,
        None => return reply(ctx, command, config, "category.not_found", None).await,
    };
    update_category(&cat.id, Some(&new), None, None, None, None, None, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), new);
    reply(ctx, command, config, "category.renamed", Some(params)).await
}

async fn sub_move(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let name = match get_string(options, "name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let pos = match get_int(options, "position") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let cat = match get_category_by_name(&name, pool).await? {
        Some(c) => c,
        None => return reply(ctx, command, config, "category.not_found", None).await,
    };
    update_category(&cat.id, None, None, None, None, Some(pos), None, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    params.insert("position".to_string(), pos.to_string());
    reply(ctx, command, config, "category.moved", Some(params)).await
}

async fn sub_enable_one(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
    enable: bool,
) -> ModmailResult<()> {
    let name = match get_string(options, "name") {
        Some(v) => v,
        None => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let cat = match get_category_by_name(&name, pool).await? {
        Some(c) => c,
        None => return reply(ctx, command, config, "category.not_found", None).await,
    };
    if enable && !cat.enabled {
        let enabled_count = count_enabled_categories(pool).await?;
        if enabled_count as usize >= CATEGORY_BUTTON_HARD_LIMIT {
            let mut params = HashMap::new();
            params.insert("max".to_string(), CATEGORY_BUTTON_HARD_LIMIT.to_string());
            return reply(
                ctx,
                command,
                config,
                "category.too_many_enabled",
                Some(params),
            )
            .await;
        }
    }
    update_category(&cat.id, None, None, None, None, None, Some(enable), pool).await?;
    let key = if enable {
        "category.enabled_one"
    } else {
        "category.disabled_one"
    };
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    reply(ctx, command, config, key, Some(params)).await
}

async fn sub_timeout(
    ctx: &Context,
    command: &CommandInteraction,
    options: &[CommandDataOption],
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let secs = match get_int(options, "seconds") {
        Some(v) if v >= 30 => v,
        _ => return reply(ctx, command, config, "category.text_usage", None).await,
    };
    let settings = get_category_settings(pool).await?;
    update_category_settings(settings.enabled, secs, pool).await?;
    let mut params = HashMap::new();
    params.insert("seconds".to_string(), secs.to_string());
    reply(
        ctx,
        command,
        config,
        "category.timeout_updated",
        Some(params),
    )
    .await
}

async fn sub_feature(
    ctx: &Context,
    command: &CommandInteraction,
    pool: &sqlx::SqlitePool,
    config: &Config,
    enable: bool,
) -> ModmailResult<()> {
    let settings = get_category_settings(pool).await?;
    update_category_settings(enable, settings.selection_timeout_s, pool).await?;
    let key = if enable {
        "category.feature_enabled"
    } else {
        "category.feature_disabled"
    };
    reply(ctx, command, config, key, None).await
}
