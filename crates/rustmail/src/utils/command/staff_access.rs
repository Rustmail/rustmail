use crate::prelude::config::*;
use serenity::all::{ChannelId, Context, GuildId, UserId};

/// Whether `user_id` may use staff commands in `guild_id`.
///
/// Dual mode: everyone in the staff guild is staff, nobody in the community guild is.
/// Single mode: staff are the members who can see the inbox category (which is also who can see
/// the tickets), plus the configured super admins.
pub async fn is_staff_member(
    ctx: &Context,
    config: &Config,
    guild_id: GuildId,
    user_id: UserId,
) -> bool {
    if guild_id.get() != config.bot.get_staff_guild_id() {
        return false;
    }

    if config.bot.is_dual_mode() || config.bot.panel_super_admin_users.contains(&user_id.get()) {
        return true;
    }

    let member = match guild_id.member(ctx, user_id).await {
        Ok(m) => m,
        Err(_) => return false,
    };

    if member
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
