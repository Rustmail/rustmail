use crate::prelude::config::*;
use crate::prelude::errors::*;
use serenity::all::{
    ChannelId, Context, Message, PermissionOverwrite, PermissionOverwriteType, Permissions, UserId,
};

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
