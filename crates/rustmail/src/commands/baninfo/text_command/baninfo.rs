use crate::db::repr::BannedUser;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{Colour, Context, CreateEmbed, GuildId, Message, RoleId};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn baninfo(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let query = extract_reply_content(&msg.content, &config.command.prefix, &["baninfo", "bi"])
        .unwrap_or_default();

    if query.is_empty() {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }

    let guild_id = config.bot.get_community_guild_id().to_string();
    let matches = resolve_banned_users(&guild_id, &query, pool).await?;

    send_baninfo_response(&ctx, config, msg.channel_id, &query, &matches).await;
    Ok(())
}

pub async fn resolve_banned_users(
    guild_id: &str,
    query: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<Vec<BannedUser>> {
    let trimmed = query.trim().trim_start_matches('@');
    if let Ok(parsed) = trimmed.parse::<u64>() {
        let user_id_str = parsed.to_string();
        if let Some(user) = get_banned_user_by_id(guild_id, &user_id_str, pool).await? {
            return Ok(vec![user]);
        }
    }
    find_banned_users_by_username(guild_id, trimmed, pool).await
}

async fn resolve_role_labels(ctx: &Context, config: &Config, role_ids: &[String]) -> Vec<String> {
    let community_guild = GuildId::new(config.bot.get_community_guild_id());

    let cached: Option<HashMap<RoleId, String>> = ctx.cache.guild(community_guild).map(|g| {
        g.roles
            .iter()
            .map(|(id, role)| (*id, role.name.clone()))
            .collect()
    });

    let map = match cached {
        Some(m) => m,
        None => match community_guild.roles(&ctx.http).await {
            Ok(roles) => roles
                .into_iter()
                .map(|(id, role)| (id, role.name))
                .collect(),
            Err(_) => HashMap::new(),
        },
    };

    role_ids
        .iter()
        .map(|raw| {
            let parsed = raw.parse::<u64>().ok().map(RoleId::new);
            match parsed.and_then(|id| map.get(&id)) {
                Some(name) => format!("**@{}**", name),
                None => format!("`@{}`", raw),
            }
        })
        .collect()
}

pub async fn send_baninfo_response(
    ctx: &Context,
    config: &Config,
    channel: serenity::all::ChannelId,
    query: &str,
    matches: &[BannedUser],
) {
    if matches.is_empty() {
        let mut params = HashMap::new();
        params.insert("query".to_string(), query.to_string());
        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content("baninfo.not_found", Some(&params), None, None)
            .await
            .to_channel(channel)
            .send(true)
            .await;
        return;
    }

    if matches.len() > 1 {
        let header =
            get_translated_message(config, "baninfo.multiple_matches", None, None, None, None)
                .await;
        let mut body = String::new();
        body.push_str(&header);
        body.push('\n');
        for user in matches {
            body.push_str(&format!(
                "• **{}** — `{}` (<@{}>)\n",
                user.username, user.user_id, user.user_id
            ));
        }
        let _ = MessageBuilder::system_message(ctx, config)
            .content(body)
            .to_channel(channel)
            .send(true)
            .await;
        return;
    }

    let user = &matches[0];
    let embed = build_baninfo_embed(ctx, config, user).await;
    let _ = MessageBuilder::system_message(ctx, config)
        .build_embed_only(embed)
        .to_channel(channel)
        .send(true)
        .await;
}

pub async fn build_baninfo_embed(ctx: &Context, config: &Config, user: &BannedUser) -> CreateEmbed {
    let dash = "—".to_string();

    let title = get_translated_message(config, "baninfo.title", None, None, None, None).await;
    let l_username =
        get_translated_message(config, "baninfo.label.username", None, None, None, None).await;
    let l_nickname =
        get_translated_message(config, "baninfo.label.nickname", None, None, None, None).await;
    let l_user_id =
        get_translated_message(config, "baninfo.label.user_id", None, None, None, None).await;
    let l_joined =
        get_translated_message(config, "baninfo.label.joined_at", None, None, None, None).await;
    let l_banned_at =
        get_translated_message(config, "baninfo.label.banned_at", None, None, None, None).await;
    let l_banned_by =
        get_translated_message(config, "baninfo.label.banned_by", None, None, None, None).await;
    let l_reason =
        get_translated_message(config, "baninfo.label.reason", None, None, None, None).await;
    let l_roles =
        get_translated_message(config, "baninfo.label.roles", None, None, None, None).await;
    let roles_unknown =
        get_translated_message(config, "baninfo.roles_unknown", None, None, None, None).await;

    let display_name = user
        .global_name
        .clone()
        .unwrap_or_else(|| user.username.clone());

    let nickname = user.nickname.clone().unwrap_or_else(|| dash.clone());
    let banned_by = user
        .banned_by
        .as_ref()
        .map(|id| format!("<@{}> (`{}`)", id, id))
        .unwrap_or_else(|| dash.clone());
    let reason = user.ban_reason.clone().unwrap_or_else(|| dash.clone());

    let joined = user
        .joined_at
        .map(|ts| format!("<t:{}:F>", ts))
        .unwrap_or_else(|| dash.clone());
    let banned_at = format!("<t:{}:F>", user.banned_at);

    let roles_text = if user.roles_unknown {
        roles_unknown
    } else if user.roles.is_empty() {
        dash.clone()
    } else {
        resolve_role_labels(ctx, config, &user.roles)
            .await
            .join(" ")
    };

    let mut embed = CreateEmbed::new()
        .title(title)
        .color(Colour::new(0xED4245))
        .field(
            l_username,
            format!("{} (`{}`)", display_name, user.username),
            true,
        )
        .field(l_nickname, nickname, true)
        .field(
            l_user_id,
            format!("<@{}> (`{}`)", user.user_id, user.user_id),
            true,
        )
        .field(l_joined, joined, true)
        .field(l_banned_at, banned_at, true)
        .field(l_banned_by, banned_by, true)
        .field(l_reason, reason, false)
        .field(l_roles, roles_text, false);

    if let Some(avatar) = &user.avatar_url {
        embed = embed.thumbnail(avatar);
    }

    embed
}
