use crate::config::Config;
use crate::db::thread_exists;
use crate::errors::CommandError::InvalidFormat;
use crate::errors::ThreadError::NotAThreadChannel;
use crate::errors::{ModmailError, ModmailResult, common};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{
    ChannelId, Context, Message, PermissionOverwrite, PermissionOverwriteType, UserId,
};
use serenity::model::Permissions;
use std::collections::HashMap;

async fn add_user_to_channel(
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

async fn extract_user_id(msg: &Message, config: &Config) -> String {
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

pub async fn add_staff(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let user_id_str = extract_user_id(msg, config).await;

    if user_id_str.is_empty() {
        return Err(ModmailError::Command(InvalidFormat));
    }

    let user_id = match user_id_str.parse::<u64>() {
        Ok(id) => UserId::new(id),
        Err(_) => return Err(ModmailError::Command(InvalidFormat)),
    };

    if thread_exists(msg.author.id, pool).await {
        match add_user_to_channel(ctx, msg.channel_id, user_id).await {
            Ok(_) => {
                let mut params = HashMap::new();
                params.insert("user".to_string(), format!("<@{}>", user_id));

                let _ = MessageBuilder::system_message(ctx, config)
                    .translated_content("add_staff.add_success", Some(&params), None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send()
                    .await;

                Ok(())
            }
            Err(..) => Err(ModmailError::Command(InvalidFormat)),
        }
    } else {
        Err(ModmailError::Thread(NotAThreadChannel))
    }
}
