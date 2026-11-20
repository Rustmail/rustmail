use crate::prelude::config::*;
use serenity::all::{ChannelId, Context, GuildId, Permissions, UserId};

pub async fn is_staff_member(
    ctx: &Context,
    config: &Config,
    guild_id: GuildId,
    user_id: UserId,
) -> bool {
    guild_id.get() == config.bot.get_staff_guild_id() && is_staff_user(ctx, config, user_id).await
}

pub async fn is_staff_user(ctx: &Context, config: &Config, user_id: UserId) -> bool {
    if config.bot.panel_super_admin_users.contains(&user_id.get()) {
        return true;
    }

    let guild_id = GuildId::new(config.bot.get_staff_guild_id());

    let member = match guild_id.member(ctx, user_id).await {
        Ok(m) => m,
        Err(_) => return false,
    };

    if config.bot.is_dual_mode()
        || member
            .roles
            .iter()
            .any(|role_id| config.bot.panel_super_admin_roles.contains(&role_id.get()))
    {
        return true;
    }

    let inbox_category_id = ChannelId::new(config.thread.inbox_category_id);

    let cached = ctx.cache.guild(guild_id).and_then(|guild| {
        guild
            .channels
            .get(&inbox_category_id)
            .map(|category| guild.user_permissions_in(category, &member))
    });

    let permissions = match cached {
        Some(p) => p,
        None => {
            let category = match inbox_category_id.to_channel(ctx).await {
                Ok(channel) => match channel.guild() {
                    Some(c) => c,
                    None => return false,
                },
                Err(_) => return false,
            };
            match guild_id.to_partial_guild(ctx).await {
                Ok(guild) => guild.user_permissions_in(&category, &member),
                Err(_) => return false,
            }
        }
    };

    permissions.view_channel()
}

pub async fn is_bot_admin(
    ctx: &Context,
    config: &Config,
    guild_id: Option<GuildId>,
    user_id: UserId,
) -> bool {
    if config.bot.panel_super_admin_users.contains(&user_id.get()) {
        return true;
    }
    let Some(guild_id) = guild_id else {
        return false;
    };
    let Ok(member) = guild_id.member(ctx, user_id).await else {
        return false;
    };
    if member
        .roles
        .iter()
        .any(|role| config.bot.panel_super_admin_roles.contains(&role.get()))
    {
        return true;
    }
    let Ok(guild) = guild_id.to_partial_guild(&ctx.http).await else {
        return false;
    };
    guild.owner_id == user_id
        || member.roles.iter().any(|role_id| {
            guild
                .roles
                .get(role_id)
                .is_some_and(|role| role.permissions.contains(Permissions::ADMINISTRATOR))
        })
}
