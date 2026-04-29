use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use serenity::all::{Context, GuildId, Message, RoleId, UserId};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn add_staff(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if !thread_exists_by_channel(msg.channel_id, pool).await {
        return Err(ModmailError::Thread(ThreadError::NotAThreadChannel));
    }

    let raw = extract_staff_id(&msg, config).await;

    if raw.is_empty() {
        return Err(ModmailError::Command(CommandError::InvalidFormat));
    }

    let parsed =
        parse_add_target(&raw).ok_or_else(|| ModmailError::Command(CommandError::InvalidFormat))?;

    let guild_id = msg
        .guild_id
        .unwrap_or_else(|| GuildId::new(config.bot.get_staff_guild_id()));

    let target = match parsed {
        AddTargetParse::Explicit(t) => t,
        AddTargetParse::AmbiguousId(id) => {
            let guild = guild_id.to_partial_guild(&ctx.http).await.map_err(|_| {
                ModmailError::Discord(DiscordError::ApiError("Guild not found".to_string()))
            })?;
            if guild.roles.contains_key(&RoleId::new(id)) {
                AddTarget::Role(RoleId::new(id))
            } else {
                AddTarget::User(UserId::new(id))
            }
        }
    };

    match target {
        AddTarget::User(user_id) => add_single_user(&ctx, &msg, config, user_id).await,
        AddTarget::Role(role_id) => add_role(&ctx, &msg, config, role_id, guild_id).await,
    }
}

async fn add_single_user(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    user_id: UserId,
) -> ModmailResult<()> {
    add_user_to_channel(ctx, msg.channel_id, user_id).await?;

    let mut params = HashMap::new();
    params.insert("user".to_string(), format!("<@{}>", user_id));

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content("add_staff.add_success", Some(&params), None, None)
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;

    Ok(())
}

async fn add_role(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    role_id: RoleId,
    guild_id: GuildId,
) -> ModmailResult<()> {
    if role_id.get() == guild_id.get() {
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_everyone_forbidden", None, None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await;
        return Ok(());
    }

    let role_mention = format!("<@&{}>", role_id);
    let members = members_with_role(ctx, guild_id, role_id).await?;

    if members.is_empty() {
        let mut params = HashMap::new();
        params.insert("role".to_string(), role_mention);
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_no_members", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await;
        return Ok(());
    }

    if members.len() > MAX_ROLE_MEMBERS_PER_ADD {
        let mut params = HashMap::new();
        params.insert("role".to_string(), role_mention);
        params.insert("count".to_string(), members.len().to_string());
        params.insert("max".to_string(), MAX_ROLE_MEMBERS_PER_ADD.to_string());
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("add_staff.role_too_many", Some(&params), None, None)
            .await
            .to_channel(msg.channel_id)
            .send(true)
            .await;
        return Ok(());
    }

    let total = members.len();
    let outcome = add_role_members_to_channel(ctx, msg.channel_id, members).await;

    let key = if outcome.failed.is_empty() {
        "add_staff.role_add_success"
    } else {
        "add_staff.role_add_partial"
    };

    let mut params = HashMap::new();
    params.insert("role".to_string(), role_mention);
    params.insert("count".to_string(), outcome.added.len().to_string());
    params.insert("added".to_string(), outcome.added.len().to_string());
    params.insert("total".to_string(), total.to_string());
    params.insert("failed".to_string(), outcome.failed.len().to_string());

    let _ = MessageBuilder::system_message(ctx, config)
        .translated_content(key, Some(&params), None, None)
        .await
        .to_channel(msg.channel_id)
        .send(true)
        .await;

    Ok(())
}
