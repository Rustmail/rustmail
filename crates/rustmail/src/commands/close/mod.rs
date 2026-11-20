pub mod args;

pub use args::*;

use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::all::{
    Channel, ChannelId, CommandOptionType, Context, CreateCommand, CreateCommandOption, GuildId,
    PermissionOverwriteType, RoleId, UserId,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::Duration;

pub struct CloseCommand;

impl Command for CloseCommand {
    type Args = CloseArgs;

    fn name(&self) -> &'static str {
        "close"
    }

    fn aliases(&self) -> &'static [&'static str] {
        &["c"]
    }

    async fn doc(&self, config: &Config) -> String {
        get_translated_message(config, "help.close", None, None, None, None).await
    }

    async fn slash_definitions(&self, config: &Config) -> Vec<CreateCommand> {
        let translate =
            async |key| get_translated_message(config, key, None, None, None, None).await;

        vec![
            CreateCommand::new("close")
                .description(translate("slash_command.close_command_description").await)
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "time_before_close",
                        translate("slash_command.close_time_before_close_argument").await,
                    )
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "silent",
                        translate("slash_command.close_silent_argument").await,
                    )
                    .required(false),
                )
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::Boolean,
                        "cancel",
                        translate("slash_command.close_cancel_argument").await,
                    )
                    .required(false),
                ),
        ]
    }

    fn parse_text(&self, args: &str) -> ModmailResult<CloseArgs> {
        parse_close_text(args)
    }

    fn parse_slash(&self, options: &SlashOptions) -> ModmailResult<CloseArgs> {
        parse_close_slash(options)
    }

    async fn run(
        &self,
        args: CloseArgs,
        invocation: &Invocation,
        config: &Config,
    ) -> ModmailResult<()> {
        let pool = config
            .db_pool
            .as_ref()
            .ok_or_else(database_connection_failed)?;
        let thread = fetch_thread(pool, &invocation.channel_id().to_string()).await?;

        match args.action {
            CloseAction::Cancel => {
                cancel_scheduled_closure(&thread, invocation, config, pool).await
            }
            CloseAction::After(delay) => {
                schedule_closure(&thread, delay, args.silent, invocation, config, pool).await
            }
            CloseAction::Now => close_now(&thread, args.silent, invocation, config, pool).await,
        }
    }
}

async fn cancel_scheduled_closure(
    thread: &Thread,
    invocation: &Invocation,
    config: &Config,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    let existed = delete_scheduled_closure(&thread.id, pool)
        .await
        .unwrap_or(false);
    if !existed {
        return Err(ModmailError::Command(
            CommandError::NoSchedulableClosureToCancel,
        ));
    }

    let _ = invocation
        .reply_translated(config, "close.closure_canceled", None)
        .await;
    Ok(())
}

async fn schedule_closure(
    thread: &Thread,
    delay: Duration,
    silent: bool,
    invocation: &Invocation,
    config: &Config,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    if let Some(remaining) = remaining_scheduled_seconds(&thread.id, pool).await {
        let mut params = HashMap::new();
        params.insert("old_time".to_string(), format_duration(remaining as u64));
        let _ = invocation
            .reply_translated(config, "close.replacing_existing_closure", Some(&params))
            .await;
    }

    let mut params = HashMap::new();
    params.insert("time".to_string(), format_duration(delay.as_secs()));
    let close_key = if silent || is_thread_silent(&thread.id, pool).await {
        "close.silent_closing"
    } else {
        "close.closing"
    };
    let _ = invocation
        .reply_translated(config, close_key, Some(&params))
        .await;

    let snapshot = ChannelSnapshot::take(invocation.ctx(), invocation.channel_id()).await;
    let close_at = Utc::now().timestamp() + delay.as_secs() as i64;

    if let Err(e) = upsert_scheduled_closure(
        &thread.id,
        close_at,
        silent,
        &invocation.user_id().to_string(),
        &snapshot.category_id,
        &snapshot.category_name,
        &snapshot.required_permissions.to_string(),
        pool,
    )
    .await
    {
        eprintln!("Failed to persist scheduled closure: {e:?}");
    }

    schedule_one(invocation.ctx(), config, thread.id.clone(), close_at);
    Ok(())
}

async fn close_now(
    thread: &Thread,
    silent: bool,
    invocation: &Invocation,
    config: &Config,
    pool: &SqlitePool,
) -> ModmailResult<()> {
    if let Some(remaining) = remaining_scheduled_seconds(&thread.id, pool).await {
        return Err(ModmailError::Command(
            CommandError::ClosureAlreadyScheduled(remaining),
        ));
    }

    let ctx = invocation.ctx();
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());
    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();
    let silent = silent || is_thread_silent(&thread.id, pool).await;

    let snapshot = ChannelSnapshot::take(ctx, invocation.channel_id()).await;

    if user_still_member && !silent {
        let _ = MessageBuilder::system_message(ctx, config)
            .content(&config.bot.close_message)
            .to_user(user_id)
            .send(true)
            .await;
    } else if !user_still_member {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());
        let _ = invocation
            .reply_translated(config, "user.left_server_close", Some(&params))
            .await;
    }

    close_thread(
        &thread.id,
        &invocation.user_id().to_string(),
        &snapshot.category_id,
        &snapshot.category_name,
        snapshot.required_permissions,
        pool,
    )
    .await?;
    let _ = delete_scheduled_closure(&thread.id, pool).await;

    log_ticket_closed(thread, invocation, config).await;

    invocation.channel_id().delete(&ctx.http).await?;
    Ok(())
}

async fn remaining_scheduled_seconds(thread_id: &str, pool: &SqlitePool) -> Option<i64> {
    let existing = get_scheduled_closure(thread_id, pool).await.ok()??;
    let remaining = existing.close_at - Utc::now().timestamp();
    (remaining > 0).then_some(remaining)
}

async fn log_ticket_closed(thread: &Thread, invocation: &Invocation, config: &Config) {
    let Some(logs_channel_id) = config.bot.logs_channel_id else {
        return;
    };
    if !config.bot.enable_rustmail_logs {
        return;
    }

    let base_url = config
        .bot
        .redirect_url
        .trim_end_matches("/api/auth/callback")
        .trim_end_matches('/');

    let mut params = HashMap::new();
    params.insert("staff".to_string(), invocation.user_id().to_string());
    params.insert("username".to_string(), thread.user_name.clone());
    params.insert("user_id".to_string(), thread.user_id.to_string());
    params.insert(
        "panel_url".to_string(),
        format!("{}/panel/tickets/{}", base_url, thread.id),
    );

    let _ = MessageBuilder::system_message(invocation.ctx(), config)
        .translated_content(
            "logs.ticket_closed",
            Some(&params),
            Some(invocation.user_id()),
            invocation.guild_id().map(|g| g.get()),
        )
        .await
        .to_channel(ChannelId::new(logs_channel_id))
        .send(true)
        .await;
}

struct ChannelSnapshot {
    category_id: String,
    category_name: String,
    required_permissions: u64,
}

impl ChannelSnapshot {
    async fn take(ctx: &Context, channel_id: ChannelId) -> Self {
        let Ok(Channel::Guild(channel)) = channel_id.to_channel(&ctx.http).await else {
            return Self {
                category_id: String::new(),
                category_name: String::new(),
                required_permissions: 0,
            };
        };

        let guild_id = channel.guild_id;
        let category_name = match channel.parent_id {
            Some(parent_id) => parent_id.name(&ctx.http).await.unwrap_or_default(),
            None => String::new(),
        };

        let everyone_role_id = RoleId::new(guild_id.get());
        let mut required_permissions = guild_id
            .to_partial_guild(&ctx.http)
            .await
            .ok()
            .and_then(|g| g.roles.get(&everyone_role_id).map(|r| r.permissions.bits()))
            .unwrap_or(0);
        for overwrite in &channel.permission_overwrites {
            if let PermissionOverwriteType::Role(_) = overwrite.kind {
                required_permissions =
                    (required_permissions & !overwrite.deny.bits()) | overwrite.allow.bits();
            }
        }

        Self {
            category_id: channel
                .parent_id
                .map(|id| id.to_string())
                .unwrap_or_default(),
            category_name,
            required_permissions,
        }
    }
}

pub fn format_duration(seconds: u64) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 3600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86400 {
        format!("{}h{}m", seconds / 3600, (seconds % 3600) / 60)
    } else {
        format!("{}d{}h", seconds / 86400, (seconds % 86400) / 3600)
    }
}
