use crate::db::repr::BannedUser;
use crate::prelude::config::*;
use crate::prelude::i18n::*;
use serenity::all::{Colour, Context, CreateEmbed, GuildId, RoleId};
use std::collections::HashMap;

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
