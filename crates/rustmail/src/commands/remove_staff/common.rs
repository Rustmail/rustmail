use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    ChannelId, Context, Message, PermissionOverwrite, PermissionOverwriteType, Permissions, UserId,
};

pub struct RemoveRoleOutcome {
    pub removed: Vec<UserId>,
    pub failed: Vec<UserId>,
}

pub async fn remove_user_from_channel(
    ctx: &Context,
    channel_id: ChannelId,
    user_id: UserId,
) -> ModmailResult<()> {
    let deny = Permissions::VIEW_CHANNEL | Permissions::SEND_MESSAGES;

    channel_id
        .create_permission(
            &ctx.http,
            PermissionOverwrite {
                allow: Permissions::empty(),
                deny,
                kind: PermissionOverwriteType::Member(user_id),
            },
        )
        .await?;

    Ok(())
}

pub async fn remove_role_members_from_channel(
    ctx: &Context,
    channel_id: ChannelId,
    members: Vec<UserId>,
) -> RemoveRoleOutcome {
    let mut removed = Vec::new();
    let mut failed = Vec::new();
    for user_id in members {
        match remove_user_from_channel(ctx, channel_id, user_id).await {
            Ok(_) => removed.push(user_id),
            Err(_) => failed.push(user_id),
        }
    }
    RemoveRoleOutcome { removed, failed }
}

pub async fn extract_remove_staff_id(msg: &Message, config: &Config) -> String {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_names = ["delmod", "dm"];

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
