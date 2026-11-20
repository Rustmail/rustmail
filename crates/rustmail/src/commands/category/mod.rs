pub mod args;

pub use args::*;

use crate::db::operations::ticket_categories::CATEGORY_BUTTON_HARD_LIMIT;
use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use sqlx::SqlitePool;
use std::collections::HashMap;

pub struct CategoryCommand;

impl Command for CategoryCommand {
    type Args = CategoryArgs;

    fn name(&self) -> &'static str {
        "category"
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.category", None, None, None, None).await
    }

    async fn slash_definitions(&self, _config: &Config) -> Vec<CreateCommand> {
        let option =
            |kind, name: &str, description: &str| CreateCommandOption::new(kind, name, description);
        let category_name =
            || option(CommandOptionType::String, "name", "Category name").required(true);
        let subcommand = |name: &str, description: &str| {
            option(CommandOptionType::SubCommand, name, description)
        };

        vec![
            CreateCommand::new("category")
                .description("Manage ticket categories")
                .add_option(
                    subcommand("create", "Create a new category")
                        .add_sub_option(
                            option(
                                CommandOptionType::String,
                                "discord_category_id",
                                "Discord category channel ID",
                            )
                            .required(true),
                        )
                        .add_sub_option(category_name())
                        .add_sub_option(option(
                            CommandOptionType::String,
                            "description",
                            "Category description",
                        ))
                        .add_sub_option(option(CommandOptionType::String, "emoji", "Button emoji")),
                )
                .add_option(subcommand("list", "List all categories"))
                .add_option(
                    subcommand("delete", "Delete a category").add_sub_option(category_name()),
                )
                .add_option(
                    subcommand("rename", "Rename a category")
                        .add_sub_option(
                            option(CommandOptionType::String, "old_name", "Current name")
                                .required(true),
                        )
                        .add_sub_option(
                            option(CommandOptionType::String, "new_name", "New name")
                                .required(true),
                        ),
                )
                .add_option(
                    subcommand("move", "Move a category position")
                        .add_sub_option(category_name())
                        .add_sub_option(
                            option(CommandOptionType::Integer, "position", "New position")
                                .required(true),
                        ),
                )
                .add_option(
                    subcommand("enable", "Enable a category").add_sub_option(category_name()),
                )
                .add_option(
                    subcommand("disable", "Disable a category").add_sub_option(category_name()),
                )
                .add_option(
                    subcommand("timeout", "Set selection timeout in seconds").add_sub_option(
                        option(CommandOptionType::Integer, "seconds", "Seconds").required(true),
                    ),
                )
                .add_option(subcommand("on", "Enable category selection feature"))
                .add_option(subcommand("off", "Disable category selection feature"))
                .add_option(
                    option(
                        CommandOptionType::SubCommandGroup,
                        "roles",
                        "Manage roles linked to a category",
                    )
                    .add_sub_option(
                        subcommand("add", "Link a role to a category")
                            .add_sub_option(category_name())
                            .add_sub_option(
                                option(CommandOptionType::Role, "role", "Role to link")
                                    .required(true),
                            ),
                    )
                    .add_sub_option(
                        subcommand("remove", "Unlink a role from a category")
                            .add_sub_option(category_name())
                            .add_sub_option(
                                option(CommandOptionType::Role, "role", "Role to unlink")
                                    .required(true),
                            ),
                    )
                    .add_sub_option(
                        subcommand("list", "List roles linked to a category")
                            .add_sub_option(category_name()),
                    )
                    .add_sub_option(
                        subcommand("clear", "Unlink all roles from a category")
                            .add_sub_option(category_name()),
                    ),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<CategoryArgs> {
        parse_category_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<CategoryArgs> {
        parse_category_slash(options)
    }

    async fn run(
        &self,
        args: CategoryArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let name_param = |name: &str| HashMap::from([("name".to_string(), name.to_string())]);

        let (key, params) = match args {
            CategoryArgs::Create {
                discord_category_id,
                name,
                description,
                emoji,
            } => {
                ensure_can_enable_one_more(pool).await?;
                if get_category_by_name(&name, pool).await?.is_some() {
                    return Err(ModmailError::Command(
                        CommandError::TicketCategoryAlreadyExists,
                    ));
                }
                let created = create_category(
                    &name,
                    description.as_deref(),
                    emoji.as_deref(),
                    &discord_category_id.to_string(),
                    pool,
                )
                .await?;
                ("category.created", name_param(&created.name))
            }
            CategoryArgs::List => return self.list(invocation, config, pool).await,
            CategoryArgs::Rename { old_name, new_name } => {
                let category = find_category(&old_name, pool).await?;
                if let Some(existing) = get_category_by_name(&new_name, pool).await?
                    && existing.id != category.id
                {
                    return Err(ModmailError::Command(
                        CommandError::TicketCategoryAlreadyExists,
                    ));
                }
                update_category(
                    &category.id,
                    Some(&new_name),
                    None,
                    None,
                    None,
                    None,
                    None,
                    pool,
                )
                .await?;
                ("category.renamed", name_param(&new_name))
            }
            CategoryArgs::Move { name, position } => {
                let category = find_category(&name, pool).await?;
                update_category(
                    &category.id,
                    None,
                    None,
                    None,
                    None,
                    Some(position),
                    None,
                    pool,
                )
                .await?;
                let mut params = name_param(&category.name);
                params.insert("position".to_string(), position.to_string());
                ("category.moved", params)
            }
            CategoryArgs::Delete { name } => {
                let category = find_category(&name, pool).await?;
                delete_category(&category.id, pool).await?;
                ("category.deleted", name_param(&category.name))
            }
            CategoryArgs::SetEnabled { name, enabled } => {
                let category = find_category(&name, pool).await?;
                if enabled && !category.enabled {
                    ensure_can_enable_one_more(pool).await?;
                }
                update_category(
                    &category.id,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(enabled),
                    pool,
                )
                .await?;
                let key = if enabled {
                    "category.enabled_one"
                } else {
                    "category.disabled_one"
                };
                (key, name_param(&category.name))
            }
            CategoryArgs::Timeout { seconds } => {
                let settings = get_category_settings(pool).await?;
                update_category_settings(settings.enabled, seconds, pool).await?;
                (
                    "category.timeout_updated",
                    HashMap::from([("seconds".to_string(), seconds.to_string())]),
                )
            }
            CategoryArgs::Selection { enabled } => {
                let settings = get_category_settings(pool).await?;
                update_category_settings(enabled, settings.selection_timeout_s, pool).await?;
                let key = if enabled {
                    "category.feature_enabled"
                } else {
                    "category.feature_disabled"
                };
                (key, HashMap::new())
            }
            CategoryArgs::Roles(roles) => change_roles(roles, pool).await?,
        };

        invocation
            .reply_translated(config, key, Some(&params))
            .await?;
        Ok(())
    }
}

impl CategoryCommand {
    async fn list(
        &self,
        invocation: &Invocation,
        config: &Config,
        pool: &SqlitePool,
    ) -> ModmailResult<()> {
        let categories = list_all_categories(pool).await?;
        if categories.is_empty() {
            invocation
                .reply_translated(config, "category.list_empty", None)
                .await?;
            return Ok(());
        }

        let translate = async |key| {
            get_translated_message(
                config,
                key,
                None,
                Some(invocation.user_id()),
                invocation.guild_id().map(|g| g.get()),
                None,
            )
            .await
        };
        let enabled = translate("category.state_enabled").await;
        let disabled = translate("category.state_disabled").await;

        let mut body = format!("**{}**\n\n", translate("category.list_header").await);
        for category in &categories {
            body.push_str(&format!(
                "`{}` {} **{}** — {}\n",
                category.position,
                category.emoji.clone().unwrap_or_default(),
                category.name,
                if category.enabled {
                    &enabled
                } else {
                    &disabled
                }
            ));
        }
        invocation
            .reply(MessageBuilder::system_message(invocation.ctx(), config).content(body))
            .await?;
        Ok(())
    }
}

async fn change_roles(
    roles: RolesArgs,
    pool: &SqlitePool,
) -> ModmailResult<(&'static str, HashMap<String, String>)> {
    let (RolesArgs::Add { name, .. }
    | RolesArgs::Remove { name, .. }
    | RolesArgs::List { name }
    | RolesArgs::Clear { name }) = &roles;
    let category = find_category(name, pool).await?;
    let mut params = HashMap::from([("name".to_string(), category.name.clone())]);

    let key = match roles {
        RolesArgs::Add { role, .. } => {
            params.insert("role".to_string(), format!("<@&{role}>"));
            if add_category_role(&category.id, &role.to_string(), pool).await? {
                "category.role_added"
            } else {
                "category.role_already_linked"
            }
        }
        RolesArgs::Remove { role, .. } => {
            params.insert("role".to_string(), format!("<@&{role}>"));
            if remove_category_role(&category.id, &role.to_string(), pool).await? {
                "category.role_removed"
            } else {
                "category.role_not_linked"
            }
        }
        RolesArgs::List { .. } => {
            let role_ids = list_category_role_ids(&category.id, pool).await?;
            if role_ids.is_empty() {
                "category.roles_list_empty"
            } else {
                let mentions: Vec<String> = role_ids.iter().map(|id| format!("<@&{id}>")).collect();
                params.insert("roles".to_string(), mentions.join(", "));
                "category.roles_list"
            }
        }
        RolesArgs::Clear { .. } => {
            let removed = clear_category_roles(&category.id, pool).await?;
            params.insert("count".to_string(), removed.to_string());
            "category.roles_cleared"
        }
    };
    Ok((key, params))
}

async fn find_category(name: &str, pool: &SqlitePool) -> ModmailResult<TicketCategory> {
    get_category_by_name(name, pool)
        .await?
        .ok_or(ModmailError::Command(CommandError::TicketCategoryNotFound))
}

async fn ensure_can_enable_one_more(pool: &SqlitePool) -> ModmailResult<()> {
    if count_enabled_categories(pool).await? as usize >= CATEGORY_BUTTON_HARD_LIMIT {
        return Err(ModmailError::Command(
            CommandError::TooManyEnabledTicketCategories {
                max: CATEGORY_BUTTON_HARD_LIMIT,
            },
        ));
    }
    Ok(())
}
