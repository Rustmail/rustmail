use crate::config::Config;
use crate::errors::ModmailResult;
use serenity::all::{
    ChannelId, Context, Message, PermissionOverwrite, PermissionOverwriteType, UserId,
};
use serenity::model::Permissions;

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

pub async fn extract_user_id(msg: &Message, config: &Config) -> String {
    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_names = ["add_staff", "as"];

    if command_names
        .iter()
        .any(|&name| content.starts_with(&format!("{}{}", prefix, name)))
    {
        let start = prefix.len() + command_names[0].len();
        content[start..].trim().to_string()
    } else {
        String::new()
    }
}
