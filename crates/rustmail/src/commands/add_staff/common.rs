use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    ChannelId, Context, GuildId, Message, PermissionOverwrite, PermissionOverwriteType, RoleId,
    UserId,
};
use serenity::model::Permissions;

pub const MAX_ROLE_MEMBERS_PER_ADD: usize = 50;

pub enum AddTarget {
    User(UserId),
    Role(RoleId),
}

pub enum AddTargetParse {
    Explicit(AddTarget),
    AmbiguousId(u64),
}

pub struct AddRoleOutcome {
    pub added: Vec<UserId>,
    pub failed: Vec<UserId>,
}

pub async fn add_user_to_channel(
    ctx: &Context,
    channel_id: ChannelId,
    user_id: UserId,
) -> ModmailResult<()> {
    let allow = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;

    channel_id
        .create_permission(
            &ctx.http,
            PermissionOverwrite {
                allow,
                deny: Permissions::empty(),
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .await?;

    Ok(())
}

pub fn parse_add_target(raw: &str) -> Option<AddTargetParse> {
    let s = raw.trim();
    if let Some(inner) = s.strip_prefix("<@&").and_then(|s| s.strip_suffix('>')) {
        return inner
            .parse::<u64>()
            .ok()
            .map(|id| AddTargetParse::Explicit(AddTarget::Role(RoleId::new(id))));
    }
    if let Some(inner) = s.strip_prefix("<@").and_then(|s| s.strip_suffix('>')) {
        let inner = inner.strip_prefix('!').unwrap_or(inner);
        return inner
            .parse::<u64>()
            .ok()
            .map(|id| AddTargetParse::Explicit(AddTarget::User(UserId::new(id))));
    }
    s.parse::<u64>().ok().map(AddTargetParse::AmbiguousId)
}

pub async fn members_with_role(
    ctx: &Context,
    guild_id: GuildId,
    role_id: RoleId,
) -> ModmailResult<Vec<UserId>> {
    let members = guild_id.members(&ctx.http, None, None).await.map_err(|_| {
        ModmailError::Discord(DiscordError::ApiError(
            "Failed to fetch guild members".to_string(),
        ))
    })?;

    Ok(members
        .into_iter()
        .filter(|m| m.roles.contains(&role_id))
        .map(|m| m.user.id)
        .collect())
}

pub async fn add_role_members_to_channel(
    ctx: &Context,
    channel_id: ChannelId,
    members: Vec<UserId>,
) -> AddRoleOutcome {
    let mut added = Vec::new();
    let mut failed = Vec::new();
    for user_id in members {
        match add_user_to_channel(ctx, channel_id, user_id).await {
            Ok(_) => added.push(user_id),
            Err(_) => failed.push(user_id),
        }
    }
    AddRoleOutcome { added, failed }
}

pub async fn extract_staff_id(msg: &Message, config: &Config) -> String {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_names = ["addmod", "am"];

    if let Some(matched_name) = command_names
        .iter()
        .find(|&name| content.starts_with(&format!("{}{}", prefix, name)))
    {
        let start = prefix.len() + matched_name.len();
        content[start..].trim().to_string()
    } else {
        String::new()
    }
}
