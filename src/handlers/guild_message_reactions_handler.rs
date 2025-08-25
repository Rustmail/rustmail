use crate::config::Config;
use crate::db::operations::{
    get_message_ids_by_message_id, get_thread_channel_by_user_id, get_user_id_from_channel_id,
};
use crate::errors::MessageError::{DmAccessFailed, MessageEmpty, MessageNotFound};
use crate::errors::types::ConfigError::ParseError;
use crate::errors::{ModmailError, ModmailResult};
use serenity::all::{ChannelId, Context, EventHandler, MessageId, Reaction, UserId};
use serenity::async_trait;

#[derive(Clone)]
pub struct GuildMessageReactionsHandler {
    pub config: Config,
}

impl GuildMessageReactionsHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for GuildMessageReactionsHandler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = handle_reaction_add(&ctx, &reaction, &self.config).await {
            eprintln!("Error handling reaction add: {}", e);
        }
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        if let Err(e) = handle_reaction_remove(&ctx, &reaction, &self.config).await {
            eprintln!("Error handling reaction remove: {}", e);
        }
    }

    async fn reaction_remove_all(
        &self,
        ctx: Context,
        channel_id: ChannelId,
        removed_from_message_id: MessageId,
    ) {
        if let Err(e) =
            handle_all_reaction_remove(&ctx, &removed_from_message_id, channel_id, &self.config)
                .await
        {
            eprintln!("Error handling all reaction remove: {}", e);
        }
    }
}

async fn handle_reaction_add(
    ctx: &Context,
    reaction: &Reaction,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    if reaction.user_id.unwrap_or_default() == ctx.cache.current_user().id {
        return Ok(());
    }

    let channel_id = reaction.channel_id.to_string();

    if let Some(user_id) = get_user_id_from_channel_id(&channel_id, pool).await {
        handle_thread_reaction_to_dm(ctx, reaction, user_id, config).await?;
    } else if reaction.guild_id.is_none() {
        handle_dm_reaction_to_thread(ctx, reaction, config).await?;
    }

    Ok(())
}

async fn handle_all_reaction_remove(
    ctx: &Context,
    removed_from_message_id: &MessageId,
    channel_id: ChannelId,
    config: &Config,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .clone()
        .ok_or_else(|| ModmailError::Config(ParseError("Database pool not available".into())))?;

    let dm_message_id_str =
        match get_message_ids_by_message_id(&removed_from_message_id.to_string(), &pool).await {
            Some(ids) => match ids.dm_message_id {
                Some(id) => id,
                None => return Err(ModmailError::Message(MessageEmpty)),
            },
            None => {
                return Err(ModmailError::Message(MessageNotFound(
                    removed_from_message_id.to_string(),
                )));
            }
        };

    let dm_message_id_parsed = dm_message_id_str
        .parse::<u64>()
        .map_err(|e| ModmailError::Message(MessageNotFound(e.to_string())))?;

    let user_id = match get_user_id_from_channel_id(&channel_id.to_string(), &pool).await {
        Some(id) if id > 0 => id as u64,
        _ => return Ok(()),
    };

    let dm_channel = UserId::new(user_id)
        .create_dm_channel(&ctx.http)
        .await
        .map_err(|e| ModmailError::Message(DmAccessFailed(e.to_string())))?;

    let dm_message = dm_channel
        .message(&ctx.http, dm_message_id_parsed)
        .await
        .map_err(|e| ModmailError::Message(DmAccessFailed(e.to_string())))?;

    let bot_user_id = Some(ctx.cache.current_user().id);
    for reaction in &dm_message.reactions {
        let _ = dm_message
            .delete_reaction(&ctx.http, bot_user_id, reaction.reaction_type.clone())
            .await;
    }

    Ok(())
}

async fn handle_reaction_remove(
    ctx: &Context,
    reaction: &Reaction,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    if reaction.user_id.unwrap_or_default() == ctx.cache.current_user().id {
        return Ok(());
    }

    let channel_id = reaction.channel_id.to_string();

    if let Some(user_id) = get_user_id_from_channel_id(&channel_id, pool).await {
        handle_thread_reaction_remove_from_dm(ctx, reaction, user_id, config).await?;
    } else if reaction.guild_id.is_none() {
        handle_dm_reaction_remove_from_thread(ctx, reaction, config).await?;
    }

    Ok(())
}

async fn handle_thread_reaction_to_dm(
    ctx: &Context,
    reaction: &Reaction,
    user_id: i64,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    let message_ids =
        match get_message_ids_by_message_id(&reaction.message_id.to_string(), pool).await {
            Some(ids) => ids,
            None => {
                return Ok(());
            }
        };

    let dm_message_id = if message_ids.inbox_message_id.is_some() {
        match message_ids.dm_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    } else {
        match message_ids.dm_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    };

    let user = UserId::new(user_id as u64);
    if let Ok(dm_channel) = user.create_dm_channel(&ctx.http).await {
        let dm_message_id_parsed = dm_message_id.parse::<u64>()?;
        let dm_message = dm_channel.message(&ctx.http, dm_message_id_parsed).await?;

        dm_message.react(&ctx.http, reaction.emoji.clone()).await?;
    }

    Ok(())
}

async fn handle_dm_reaction_to_thread(
    ctx: &Context,
    reaction: &Reaction,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    let user_id = reaction.user_id.unwrap_or_default();
    let thread_channel_id = match get_thread_channel_by_user_id(user_id, pool).await {
        Some(id) => id,
        None => return Ok(()),
    };

    let thread_channel_id_parsed = thread_channel_id.parse::<u64>()?;
    let thread_channel = ChannelId::new(thread_channel_id_parsed);

    let message_ids =
        match get_message_ids_by_message_id(&reaction.message_id.to_string(), pool).await {
            Some(ids) => ids,
            None => {
                return Ok(());
            }
        };

    let thread_message_id = if message_ids.inbox_message_id.is_some() {
        match message_ids.inbox_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    } else {
        reaction.message_id.to_string()
    };

    let thread_message_id_parsed = thread_message_id.parse::<u64>()?;
    let thread_message = thread_channel
        .message(&ctx.http, thread_message_id_parsed)
        .await?;

    thread_message
        .react(&ctx.http, reaction.emoji.clone())
        .await?;

    Ok(())
}

async fn handle_thread_reaction_remove_from_dm(
    ctx: &Context,
    reaction: &Reaction,
    user_id: i64,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    let message_ids =
        match get_message_ids_by_message_id(&reaction.message_id.to_string(), pool).await {
            Some(ids) => ids,
            None => {
                return Ok(());
            }
        };

    let dm_message_id = if message_ids.inbox_message_id.is_some() {
        match message_ids.dm_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    } else {
        match message_ids.dm_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    };

    let user = UserId::new(user_id as u64);
    if let Ok(dm_channel) = user.create_dm_channel(&ctx.http).await {
        let dm_message_id_parsed = dm_message_id.parse::<u64>()?;
        let dm_message = dm_channel.message(&ctx.http, dm_message_id_parsed).await?;

        let bot_user_id = Some(ctx.cache.current_user().id);
        let _ = dm_message
            .delete_reaction(&ctx.http, bot_user_id, reaction.emoji.clone())
            .await;
    }

    Ok(())
}

async fn handle_dm_reaction_remove_from_thread(
    ctx: &Context,
    reaction: &Reaction,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or("Database pool not available")?;

    let user_id = reaction.user_id.unwrap_or_default();
    let thread_channel_id = match get_thread_channel_by_user_id(user_id, pool).await {
        Some(id) => id,
        None => return Ok(()),
    };

    let thread_channel_id_parsed = thread_channel_id.parse::<u64>()?;
    let thread_channel = ChannelId::new(thread_channel_id_parsed);

    let message_ids =
        match get_message_ids_by_message_id(&reaction.message_id.to_string(), pool).await {
            Some(ids) => ids,
            None => {
                return Ok(());
            }
        };

    let thread_message_id = if message_ids.inbox_message_id.is_some() {
        match message_ids.inbox_message_id {
            Some(id) => id,
            None => return Ok(()),
        }
    } else {
        reaction.message_id.to_string()
    };

    let thread_message_id_parsed = thread_message_id.parse::<u64>()?;
    let thread_message = thread_channel
        .message(&ctx.http, thread_message_id_parsed)
        .await?;

    let bot_user_id = Some(ctx.cache.current_user().id);
    let _ = thread_message
        .delete_reaction(&ctx.http, bot_user_id, reaction.emoji.clone())
        .await;

    Ok(())
}
