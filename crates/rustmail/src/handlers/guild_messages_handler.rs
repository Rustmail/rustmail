use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::utils::*;
use serenity::all::{GuildId, MessageId, UserId};
use serenity::{
    all::{ChannelId, Context, EventHandler, Message, MessageUpdateEvent},
    async_trait,
};
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};
use std::{collections::HashMap, sync::Arc};

static SUPPRESSED_DELETES: LazyLock<Mutex<HashSet<u64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

#[derive(Clone)]
pub struct GuildMessagesHandler {
    pub config: Arc<Config>,
    pub registry: Arc<CommandRegistry>,
    pub maintenance_mode: Arc<AtomicBool>,
}

impl GuildMessagesHandler {
    pub fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        maintenance_mode: Arc<AtomicBool>,
    ) -> Self {
        Self {
            config: Arc::new(config.clone()),
            registry,
            maintenance_mode,
        }
    }
}

async fn manage_incoming_message(
    ctx: &Context,
    msg: &Message,
    config: &Config,
) -> ModmailResult<()> {
    if msg.author.bot {
        return Ok(());
    }

    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    if let Some(guild_id) = msg.guild_id {
        let community_guild_id = config.bot.get_community_guild_id();
        if guild_id.get() != community_guild_id {
            let error_key = if config.bot.is_dual_mode() {
                "server.wrong_guild_dual"
            } else {
                "server.wrong_guild_single"
            };

            let error_msg = get_translated_message(
                config,
                error_key,
                None,
                Some(msg.author.id),
                Some(guild_id.get()),
                None,
            )
            .await;

            let error = validation_failed(&error_msg);
            let _ = error_handler
                .reply_to_msg_with_error(ctx, msg, &error)
                .await;
            return Err(error);
        }
    }

    let user_key = msg.author.id.get();
    let user_mutex = get_thread_lock(config, user_key);
    let guard = user_mutex.lock().await;

    if thread_exists_by_user(msg.author.id, pool).await {
        if let Some(channel_id_str) = get_thread_channel_by_user_id(msg.author.id, pool).await {
            let channel_id_num = channel_id_str
                .parse::<u64>()
                .map_err(|_| validation_failed("Invalid channel ID format"))?;

            let channel_id = ChannelId::new(channel_id_num);

            const MAX_ATTACHMENT_SIZE: u32 = 8 * 1024 * 1024;
            for attachment in &msg.attachments {
                if attachment.size > MAX_ATTACHMENT_SIZE {
                    let _ = MessageBuilder::system_message(ctx, config)
                        .translated_content(
                            "discord.attachment_too_large",
                            None,
                            Some(msg.author.id),
                            None,
                        )
                        .await
                        .to_user(msg.author.id)
                        .send(true)
                        .await;

                    drop(guard);
                    return Ok(());
                }
            }

            if let Err(e) = send_to_thread(ctx, channel_id, msg, config, false).await {
                let error = validation_failed(&format!("Failed to forward message: {}", e));
                let _ = error_handler
                    .reply_to_msg_with_error(ctx, msg, &error)
                    .await;
                return Err(error);
            }

            if let Ok(thread) = fetch_thread(pool, &channel_id_str).await
                && let Ok(existed) = delete_scheduled_closure(&thread.id, pool).await
                && existed
            {
                let _ = MessageBuilder::system_message(ctx, config)
                    .translated_content(
                        "close.auto_canceled_on_message",
                        None,
                        Some(msg.author.id),
                        None,
                    )
                    .await
                    .to_channel(channel_id)
                    .send(true)
                    .await;
            }
        }
    } else {
        if maybe_start_category_selection(ctx, config, msg).await {
            drop(guard);
            return Ok(());
        }
        create_channel(ctx, msg, config).await;
    }

    drop(guard);
    Ok(())
}

#[async_trait]
impl EventHandler for GuildMessagesHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_none() {
            if self.maintenance_mode.load(Ordering::Relaxed) {
                let _ = MessageBuilder::system_message(&ctx, &self.config)
                    .translated_content(
                        "status.maintenance_mode_active_user",
                        None,
                        Some(msg.author.id),
                        None,
                    )
                    .await
                    .to_user(msg.author.id)
                    .send(true)
                    .await;
                return;
            }

            if let Err(error) = manage_incoming_message(&ctx, &msg, &self.config).await {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler
                        .reply_to_msg_with_error(&ctx, &msg, &error)
                        .await;
                } else {
                    eprintln!("DM handling error: {}", error);
                }
            }
            return;
        }

        if let Some((command_name, args)) =
            split_text_command(&msg.content, &self.config.command.prefix)
        {
            if let Some(guild_id) = msg.guild_id
                && !is_staff_member(&ctx, &self.config, guild_id, msg.author.id).await
            {
                return;
            }

            if self.maintenance_mode.load(Ordering::Relaxed)
                && !is_bot_admin(&ctx, &self.config, msg.guild_id, msg.author.id).await
            {
                let _ = MessageBuilder::system_message(&ctx, &self.config)
                    .translated_content("status.maintenance_mode_active", None, None, None)
                    .await
                    .to_channel(msg.channel_id)
                    .send(true)
                    .await;
                return;
            }

            if let Some(command) = self.registry.text(command_name) {
                let invocation = Invocation::text(ctx.clone(), msg.clone());
                if let Err(error) = command.run_text(args, &invocation, &self.config).await {
                    invocation.reply_error(&self.config, &error).await;
                }
            }
            return;
        }

        if let Some(guild_id) = msg.guild_id {
            let staff_guild_id = self.config.bot.get_staff_guild_id();
            if guild_id.get() == staff_guild_id
                && !msg.author.bot
                && let Some(pool) = &self.config.db_pool
            {
                let channel_id_str = msg.channel_id.to_string();
                if let Some(_thread) = get_thread_by_channel_id(&channel_id_str, pool).await
                    && let Err(e) =
                        insert_internal_message(&ctx, &msg, &_thread.id, pool, &self.config).await
                {
                    eprintln!("Failed to record internal message: {}", e);
                }
            }
        }
        return;
    }

    async fn message_delete(
        &self,
        ctx: Context,
        _channel_id: ChannelId,
        deleted_message_id: MessageId,
        _guild_id: Option<GuildId>,
    ) {
        {
            let mut suppressed = SUPPRESSED_DELETES.lock().unwrap();
            if suppressed.remove(&deleted_message_id.get()) {
                return;
            }
        }
        let pool = match &self.config.db_pool {
            Some(p) => p,
            None => return,
        };

        let message_entry =
            match get_thread_message_by_message_id(&deleted_message_id.to_string(), pool).await {
                Ok(m) => m,
                Err(_) => return,
            };

        let thread_opt =
            get_thread_by_user_id(UserId::new(message_entry.user_id as u64), pool).await;
        let thread = match thread_opt {
            Some(t) => t,
            None => return,
        };

        if self.config.logs.show_log_on_delete {
            let guild_id = self.config.bot.get_community_guild_id();
            let mut params = HashMap::new();

            params.insert(
                "content".to_string(),
                format!("`{}`", message_entry.content.clone()),
            );
            params.insert(
                "userid".to_string(),
                format!("<@{}>", message_entry.user_id),
            );

            let is_staff_message = message_entry.message_number.is_some();
            let key = if is_staff_message {
                "delete.removed_by_staff"
            } else {
                "delete.removed_by_user"
            };

            let _ = MessageBuilder::system_message(&ctx, &self.config)
                .translated_content(
                    key,
                    Some(&params),
                    Some(UserId::new(message_entry.user_id as u64)),
                    Some(guild_id),
                )
                .await
                .to_channel(ChannelId::new(
                    thread.channel_id.parse::<u64>().unwrap_or(0),
                ))
                .send(true)
                .await;
        }

        if let Some(num) = message_entry.message_number {
            let _ = update_message_numbers_after_deletion(&thread.channel_id, num, pool).await;
        }

        let _ = delete_message(&deleted_message_id.to_string(), pool).await;
    }

    async fn message_delete_bulk(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        multiple_deleted_messages_ids: Vec<MessageId>,
        guild_id: Option<GuildId>,
    ) {
        let futures: Vec<_> = multiple_deleted_messages_ids
            .into_iter()
            .map(|mid| self.message_delete(ctx.clone(), channel_id, mid, guild_id))
            .collect();

        futures::future::join_all(futures).await;
    }

    async fn message_update(
        &self,
        ctx: Context,
        old_if_available: Option<Message>,
        _new: Option<Message>,
        event: MessageUpdateEvent,
    ) {
        match event.author {
            Some(user) => {
                if user.bot {
                    return;
                }
            }
            None => {
                eprintln!("Message update event without author");
                return;
            }
        };

        if let Some(_channel_id) = event
            .channel_id
            .to_channel(&ctx.http)
            .await
            .ok()
            .and_then(|channel| channel.private())
        {
            let pool = match &self.config.db_pool {
                Some(p) => p,
                None => return,
            };

            let message = match get_thread_message_by_dm_message_id(event.id, pool).await {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("Failed to get thread message by DM message ID: {}", e);
                    return;
                }
            };

            if let Some(thread) =
                get_thread_by_user_id(UserId::new(message.user_id as u64), pool).await
                && let Some(content) = event.content
            {
                let inbox_builder = MessageBuilder::user_message(
                    &ctx,
                    &self.config,
                    UserId::new(message.user_id as u64),
                    message.user_name,
                )
                .content(content.clone());
                let edit_msg = inbox_builder.build_edit_message().await;

                let channel_id_parse = match thread.channel_id.parse::<u64>() {
                    Ok(id) => ChannelId::new(id),
                    Err(e) => {
                        eprintln!("Failed to parse channel ID: {}", e);
                        return;
                    }
                };

                if let Some(inbox_message_id) = message.inbox_message_id {
                    let Ok(inbox_id) = inbox_message_id.parse::<u64>() else {
                        eprintln!("Invalid inbox message id: {}", inbox_message_id);
                        return;
                    };
                    if let Err(e) = channel_id_parse
                        .edit_message(&ctx.http, MessageId::new(inbox_id), edit_msg)
                        .await
                    {
                        eprintln!("Failed to edit mirrored staff message: {}", e);
                        return;
                    }

                    if self.config.logs.show_log_on_edit {
                        let old_content: String = if let Some(old) = old_if_available {
                            old.content
                        } else {
                            String::new()
                        };
                        let before = if old_content.is_empty() {
                            "`(inconnu)`".to_string()
                        } else {
                            format!("`{}`", old_content)
                        };
                        let after = format!("`{}`", content.clone());

                        let guild_id = self.config.bot.get_community_guild_id();
                        let message_link = format!(
                            "https://discord.com/channels/{}/{}/{}",
                            guild_id,
                            channel_id_parse.get(),
                            inbox_message_id
                        );

                        let mut params = HashMap::new();
                        params.insert("before".to_string(), before);
                        params.insert("after".to_string(), after);
                        params.insert("link".to_string(), message_link);

                        let _ = MessageBuilder::system_message(&ctx, &self.config)
                            .translated_content(
                                "edit.modification_from_user",
                                Some(&params),
                                Some(UserId::new(message.user_id as u64)),
                                Some(guild_id),
                            )
                            .await
                            .to_channel(channel_id_parse)
                            .send(true)
                            .await;
                    }

                    let _ = update_message_content(&inbox_message_id, &content, pool).await;
                }
            }
        }
    }
}
