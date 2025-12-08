use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::i18n::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use crate::prelude::utils::*;
use crate::wrap_command;
use serenity::all::{GuildId, MessageId, UserId};
use serenity::{
    all::{ChannelId, Context, EventHandler, Message, MessageUpdateEvent},
    async_trait,
};
use std::collections::HashSet;
use std::sync::{LazyLock, Mutex};
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::watch::Receiver;

static SUPPRESSED_DELETES: LazyLock<Mutex<HashSet<u64>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

type CommandFunc = Arc<StaticCommandFunc>;
type StaticCommandFunc = dyn for<'a> Fn(
        Context,
        Message,
        &'a Config,
        Arc<GuildMessagesHandler>,
    ) -> Pin<Box<dyn Future<Output = ModmailResult<()>> + Send + 'a>>
    + Send
    + Sync
    + 'static;

#[derive(Clone)]
pub struct GuildMessagesHandler {
    pub config: Arc<Config>,
    pub commands: Arc<AsyncMutex<HashMap<String, CommandFunc>>>,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
    pub pagination: PaginationStore,
}

impl GuildMessagesHandler {
    pub async fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        shutdown: Receiver<bool>,
        pagination: PaginationStore,
    ) -> Self {
        let h = Self {
            config: Arc::new(config.clone()),
            commands: Arc::new(AsyncMutex::new(HashMap::new())),
            registry,
            shutdown: Arc::new(shutdown),
            pagination,
        };

        let mut lock = h.commands.lock().await;

        wrap_command!(lock, ["reply", "r"], reply);
        wrap_command!(lock, ["edit", "e"], edit);
        wrap_command!(lock, ["close", "c"], close);
        wrap_command!(lock, "recover", recover);
        wrap_command!(lock, "alert", alert);
        wrap_command!(lock, ["move", "mv"], move_thread);
        wrap_command!(lock, ["nt", "new_thread"], new_thread);
        wrap_command!(lock, "delete", delete);
        wrap_command!(lock, ["anonreply", "ar"], anonreply);
        wrap_command!(lock, ["force_close", "fc"], force_close);
        wrap_command!(lock, ["addmod", "am"], add_staff);
        wrap_command!(lock, ["delmod", "dm"], remove_staff);
        wrap_command!(lock, "id", id);
        wrap_command!(lock, "help", help);
        wrap_command!(lock, ["remind", "rem"], add_reminder);
        wrap_command!(lock, ["unremind", "urem"], remove_reminder);
        wrap_command!(lock, "logs", logs);
        wrap_command!(lock, "take", take);
        wrap_command!(lock, "release", release);
        wrap_command!(lock, "ping", ping);
        wrap_command!(lock, ["snippet", "s"], snippet_command);

        drop(lock);
        h
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

    if thread_exists(msg.author.id, pool).await {
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

            if let Ok(thread) = fetch_thread(pool, &channel_id_str).await {
                if let Ok(existed) = delete_scheduled_closure(&thread.id, pool).await {
                    if existed {
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
            }
        }
    } else {
        create_channel(ctx, msg, config).await;
    }

    drop(guard);
    Ok(())
}

#[async_trait]
impl EventHandler for GuildMessagesHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id.is_none() {
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

        let message_content = &msg.content;
        if message_content.starts_with(&self.config.command.prefix) {
            let mut command_name = &message_content[1..];

            if let Some(i) = message_content.find(" ") {
                command_name = &message_content[self.config.command.prefix.len()..i];
            }

            let commands_lock = self.commands.lock().await;

            if let Some(command_func) = commands_lock.get(command_name)
                && let Err(error) = command_func(
                    ctx.clone(),
                    msg.clone(),
                    &self.config.clone(),
                    Arc::new(self.clone()),
                )
                .await
            {
                if let Some(error_handler) = &self.config.error_handler {
                    let _ = error_handler
                        .reply_to_msg_with_error(&ctx, &msg, &error)
                        .await;
                } else {
                    eprintln!("Command error: {}", error);
                }
            }
            return;
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
                    if let Err(e) =
                        edit_inbox_message(&ctx, channel_id_parse, &inbox_message_id, edit_msg)
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
