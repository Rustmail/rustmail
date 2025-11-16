use serenity::all::{Channel, Context, GuildChannel, PermissionOverwriteType, RoleId};

pub async fn get_category_id_from_guild_channel(ctx: &Context, channel: &GuildChannel) -> String {
    match channel.id.to_channel(&ctx.http).await {
        Ok(channel) => match channel.guild() {
            Some(guild_channel) => match guild_channel.parent_id {
                Some(category_id) => category_id.to_string(),
                None => String::new(),
            },
            None => String::new(),
        },
        _ => String::new(),
    }
}

pub async fn get_category_name_from_guild_channel(ctx: &Context, channel: &GuildChannel) -> String {
    match channel.id.to_channel(&ctx.http).await {
        Ok(channel) => match channel.guild() {
            Some(guild_channel) => match guild_channel.parent_id {
                Some(category_id) => match category_id.name(&ctx.http).await {
                    Ok(category_name) => category_name.clone(),
                    _ => String::new(),
                },
                None => String::new(),
            },
            None => String::new(),
        },
        _ => String::new(),
    }
}

pub async fn get_required_permissions_channel_from_guild_channel(
    ctx: &Context,
    channel: &GuildChannel,
) -> u64 {
    match channel.id.to_channel(&ctx.http).await {
        Ok(Channel::Guild(guild_channel)) => {
            let guild_id = guild_channel.guild_id;
            let guild = guild_id.to_partial_guild(&ctx.http).await.ok();

            let everyone_role_id = RoleId::new(guild_id.get());

            let mut perms = guild
                .and_then(|g| g.roles.get(&everyone_role_id).map(|r| r.permissions.bits()))
                .unwrap_or(0u64);

            for overwrite in &guild_channel.permission_overwrites {
                if let PermissionOverwriteType::Role(_) = overwrite.kind {
                    let allow = overwrite.allow.bits();
                    let deny = overwrite.deny.bits();
                    perms = (perms & !deny) | allow;
                }
            }

            perms
        }
        _ => 0u64,
    }
}
