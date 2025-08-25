use crate::config::Config;
use crate::db::messages::{MessageIds, get_thread_message_by_inbox_message_id};
use crate::db::operations::{
    get_message_ids_by_number, get_thread_by_channel_id, get_user_id_from_channel_id,
};
use crate::errors::MessageError::{DmAccessFailed, EditFailed};
use crate::errors::common::{incorrect_message_id, not_found, permission_denied, thread_not_found};
use crate::errors::{ModmailError, ModmailResult};
use crate::utils::conversion::hex_string_to_int::hex_string_to_int;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{ChannelId, Context, EditMessage, Message, MessageId, UserId};
use sqlx::SqlitePool;

pub async fn get_message_ids(
    message_number: i64,
    user_id: UserId,
    pool: &SqlitePool,
    _ctx: &Context,
    msg: &Message,
) -> ModmailResult<MessageIds> {
    let thread = match get_thread_by_channel_id(&msg.channel_id.to_string(), pool).await {
        Some(thread) => thread,
        None => return Err(thread_not_found()),
    };

    match get_message_ids_by_number(message_number, user_id, &thread.id, pool).await {
        Some(message_ids) => Ok(message_ids),
        None => Err(not_found("message not found")),
    }
}

pub async fn format_new_message<'a>(
    ctx: &'a Context,
    msg: &Message,
    content: &str,
    inbox_message_id: &str,
    message_number: u64,
    config: &'a Config,
    pool: &SqlitePool,
) -> ModmailResult<(MessageBuilder<'a>, MessageBuilder<'a>)> {
    let thread_message = match get_thread_message_by_inbox_message_id(inbox_message_id, pool).await
    {
        Ok(thread_message) => thread_message,
        Err(..) => return Err(permission_denied()),
    };

    let mut top_role_name: Option<String> = None;
    if let Some(guild_id) = msg.guild_id
        && let (Ok(member), Ok(roles_map)) = (
            guild_id.member(&ctx.http, msg.author.id).await,
            guild_id.roles(&ctx.http).await,
        )
    {
        top_role_name = member
            .roles
            .iter()
            .filter_map(|rid| roles_map.get(rid))
            .filter(|r| r.name != "@everyone")
            .max_by_key(|r| r.position)
            .map(|r| r.name.clone());
    }

    if thread_message.is_anonymous {
        let mut inbox_builder = MessageBuilder::anonymous_staff_message(ctx, config, msg.author.id)
            .content(content.to_string())
            .with_message_number(message_number);
        if let Some(role_name) = &top_role_name {
            inbox_builder = inbox_builder.with_role(role_name.clone());
        }

        let mut dm_builder = MessageBuilder::anonymous_staff_message(ctx, config, msg.author.id)
            .content(content.to_string());
        if let Some(role_name) = &top_role_name {
            dm_builder = dm_builder.with_role(role_name.clone());
        }

        Ok((inbox_builder, dm_builder))
    } else {
        let mut inbox_builder =
            MessageBuilder::staff_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(content.to_string())
                .with_message_number(message_number);
        if let Some(role_name) = &top_role_name {
            inbox_builder = inbox_builder.with_role(role_name.clone());
        }

        let mut dm_builder =
            MessageBuilder::staff_message(ctx, config, msg.author.id, msg.author.name.clone())
                .content(content.to_string());
        if let Some(role_name) = &top_role_name {
            dm_builder = dm_builder.with_role(role_name.clone());
        }

        Ok((inbox_builder, dm_builder))
    }
}

pub async fn edit_inbox_message(
    ctx: &Context,
    channel_id: ChannelId,
    inbox_msg_id: &str,
    edit_message: EditMessage,
) -> ModmailResult<()> {
    let message_id = match inbox_msg_id.parse::<u64>() {
        Ok(id) => MessageId::new(id),
        Err(e) => {
            return Err(incorrect_message_id(&format!(
                "Unable to parse inbox_msg_id (String) into message id (MessageId) : {}",
                e
            )));
        }
    };

    match channel_id
        .edit_message(&ctx.http, message_id, edit_message)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(ModmailError::Message(EditFailed(e.to_string()))),
    }
}

pub async fn edit_dm_message<'a>(
    ctx: &Context,
    channel_id: ChannelId,
    dm_msg_id: &str,
    edit_dm_builder: MessageBuilder<'a>,
    pool: &SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let message_id = match dm_msg_id.parse::<u64>() {
        Ok(id) => MessageId::new(id),
        Err(e) => {
            return Err(incorrect_message_id(&format!(
                "Unable to parse inbox_msg_id (String) into message id (MessageId) : {}",
                e
            )));
        }
    };

    let user_id = match get_user_id_from_channel_id(&channel_id.get().to_string(), pool).await {
        Some(user_id) => UserId::new(user_id as u64),
        None => return Err(not_found("user not found")),
    };

    let edit_dm_msg = edit_dm_builder
        .to_user(user_id)
        .color(hex_string_to_int(&config.thread.staff_message_color) as u32)
        .build_edit_message()
        .await;

    let dm_channel = match user_id.create_dm_channel(&ctx.http).await {
        Ok(channel) => channel,
        Err(e) => {
            return Err(ModmailError::Message(DmAccessFailed(format!(
                "Unable to access user DM (Maybe the user doesn't allow private messages from bots) : {}",
                e
            ))));
        }
    };

    let edit_result: ModmailResult<()> = match dm_channel
        .edit_message(&ctx.http, message_id, edit_dm_msg)
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => return Err(ModmailError::Message(EditFailed(e.to_string()))),
    };

    edit_result
}

pub async fn edit_messages<'a>(
    ctx: &Context,
    channel_id: ChannelId,
    dm_msg_id: String,
    inbox_msg_id: String,
    edited_message_builder: (MessageBuilder<'a>, MessageBuilder<'a>),
    pool: &SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let (inbox_msg_builder, dm_msg_builder) = edited_message_builder;

    let edit_inbox_msg = inbox_msg_builder.build_edit_message().await;

    match edit_inbox_message(ctx, channel_id, &inbox_msg_id, edit_inbox_msg).await {
        Ok(()) => (),
        Err(e) => return Err(e),
    };

    match edit_dm_message(ctx, channel_id, &dm_msg_id, dm_msg_builder, pool, config).await {
        Ok(()) => (),
        Err(e) => return Err(e),
    };

    Ok(())
}

pub async fn cleanup_command_message(ctx: &Context, msg: &Message) {
    if let Err(e) = msg.delete(&ctx.http).await {
        eprintln!("Failed to delete command message: {:?}", e);
    }
}
