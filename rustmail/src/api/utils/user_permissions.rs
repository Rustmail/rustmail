use crate::prelude::api::*;
use serenity::all::{
    ChannelId, GuildId, Http, PermissionOverwriteType, Permissions, RoleId, UserId,
};
use std::sync::Arc;

pub async fn is_admin_or_owner(user_id: &str, guild_id: u64, bot_http: Arc<Http>) -> bool {
    let cache = get_admin_cache();
    if let Some(is_admin) = cache.get(user_id).await {
        return is_admin;
    }

    let user_id_num = match user_id.parse::<u64>() {
        Ok(id) => id,
        Err(_) => return false,
    };
    let guild_id_obj = GuildId::new(guild_id);
    let user_id_obj = UserId::new(user_id_num);

    let guild = match guild_id_obj.to_partial_guild(bot_http.clone()).await {
        Ok(g) => g,
        Err(_) => return false,
    };

    if guild.owner_id == user_id_obj {
        cache.insert(user_id.to_string(), true).await;
        return true;
    }

    let member = match guild_id_obj.member(bot_http.clone(), user_id_obj).await {
        Ok(m) => m,
        Err(_) => {
            cache.insert(user_id.to_string(), false).await;
            return false;
        }
    };

    let is_admin = member.roles.iter().any(|role_id| {
        guild
            .roles
            .get(role_id)
            .map(|role| role.permissions.contains(Permissions::ADMINISTRATOR))
            .unwrap_or(false)
    });

    cache.insert(user_id.to_string(), is_admin).await;
    is_admin
}

pub async fn get_user_permissions_in_category(
    user_id: &str,
    guild_id: u64,
    category_id: &str,
    bot_http: Arc<Http>,
) -> Option<u64> {
    let cache = get_permissions_cache();
    let cache_key = (user_id.to_string(), category_id.to_string());
    if let Some(perms) = cache.get(&cache_key).await {
        return Some(perms);
    }

    let user_id_num = user_id.parse::<u64>().ok()?;
    let category_id_num = category_id.parse::<u64>().ok()?;
    let guild_id_obj = GuildId::new(guild_id);
    let user_id_obj = UserId::new(user_id_num);
    let channel_id = ChannelId::new(category_id_num);

    let member = match guild_id_obj.member(bot_http.clone(), user_id_obj).await {
        Ok(m) => m,
        Err(_) => return None,
    };

    let guild_roles = match guild_id_obj.roles(bot_http.clone()).await {
        Ok(roles) => roles,
        Err(_) => return None,
    };

    let category = match channel_id.to_channel(bot_http.clone()).await {
        Ok(channel) => match channel.guild() {
            Some(guild_channel) => guild_channel,
            None => return None,
        },
        Err(_) => return None,
    };

    let everyone_role_id = RoleId::new(guild_id_obj.get());
    let mut permissions = guild_roles
        .get(&everyone_role_id)
        .map(|r| r.permissions.bits())
        .unwrap_or(0u64);

    for overwrite in &category.permission_overwrites {
        if let PermissionOverwriteType::Role(role_id) = overwrite.kind {
            if role_id == everyone_role_id {
                let deny = overwrite.deny.bits();
                let allow = overwrite.allow.bits();
                permissions = (permissions & !deny) | allow;
                break;
            }
        }
    }

    let mut combined_allow = 0u64;
    let mut combined_deny = 0u64;

    for role_id in &member.roles {
        for overwrite in &category.permission_overwrites {
            if let PermissionOverwriteType::Role(overwrite_role_id) = overwrite.kind {
                if overwrite_role_id == *role_id {
                    combined_allow |= overwrite.allow.bits();
                    combined_deny |= overwrite.deny.bits();
                }
            }
        }
    }

    permissions = (permissions & !combined_deny) | combined_allow;

    for overwrite in &category.permission_overwrites {
        if let PermissionOverwriteType::Member(member_id) = overwrite.kind {
            if member_id == user_id_obj {
                let deny = overwrite.deny.bits();
                let allow = overwrite.allow.bits();
                permissions = (permissions & !deny) | allow;
                break;
            }
        }
    }

    cache.insert(cache_key, permissions).await;
    Some(permissions)
}

pub fn can_view_channel(user_permissions: u64) -> bool {
    const VIEW_CHANNEL: u64 = 1 << 10;
    (user_permissions & VIEW_CHANNEL) == VIEW_CHANNEL
}
