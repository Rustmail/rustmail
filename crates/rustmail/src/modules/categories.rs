use crate::modules::threads::create_or_get_thread_for_user;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use chrono::Utc;
use serenity::all::{
    ButtonStyle, ChannelId, ComponentInteraction, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, Message, ReactionType, UserId,
};
use serenity::builder::{CreateActionRow, CreateButton};
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

pub const CATEGORY_BUTTON_MAX_PER_ROW: usize = 5;
pub const CATEGORY_BUTTON_MAX_ROWS: usize = 5;

fn build_category_components(
    categories: &[TicketCategory],
    default_label: &str,
) -> Vec<CreateActionRow> {
    let mut buttons: Vec<CreateButton> = Vec::new();

    for cat in categories.iter().take(CATEGORY_BUTTON_HARD_LIMIT) {
        let mut btn = CreateButton::new(format!("category:pick:{}", cat.id))
            .label(cat.name.clone())
            .style(ButtonStyle::Primary);
        if let Some(emoji) = cat.emoji.as_deref() {
            if let Some(react) = parse_emoji(emoji) {
                btn = btn.emoji(react);
            }
        }
        buttons.push(btn);
    }

    buttons.push(
        CreateButton::new("category:default")
            .label(default_label.to_string())
            .style(ButtonStyle::Secondary),
    );

    let mut rows: Vec<CreateActionRow> = Vec::new();
    for chunk in buttons.chunks(CATEGORY_BUTTON_MAX_PER_ROW) {
        if rows.len() >= CATEGORY_BUTTON_MAX_ROWS {
            break;
        }
        rows.push(CreateActionRow::Buttons(chunk.to_vec()));
    }
    rows
}

fn parse_emoji(raw: &str) -> Option<ReactionType> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    trimmed.parse::<ReactionType>().ok()
}

pub async fn maybe_start_category_selection(ctx: &Context, config: &Config, msg: &Message) -> bool {
    let pool = match &config.db_pool {
        Some(p) => p,
        None => return false,
    };

    let settings = match get_category_settings(pool).await {
        Ok(s) => s,
        Err(_) => return false,
    };
    if !settings.enabled {
        return false;
    }

    if let Ok(Some(_)) = get_pending_selection(msg.author.id.get() as i64, pool).await {
        match append_queued_message(msg.author.id.get() as i64, &msg.id.to_string(), pool).await {
            Ok(_) => return true,
            Err(err) => {
                eprintln!(
                    "failed to append queued message for user {} and message {}: {}",
                    msg.author.id.get(),
                    msg.id,
                    err
                );
                return false;
            }
        }
    }

    let categories = match list_enabled_categories(pool).await {
        Ok(c) => c,
        Err(_) => return false,
    };
    if categories.is_empty() {
        return false;
    }

    let timeout_minutes = (settings.selection_timeout_s / 60).max(1);
    let mut params = HashMap::new();
    params.insert("timeout_minutes".to_string(), timeout_minutes.to_string());

    let default_label = get_translated_message(
        config,
        "category.default_button_label",
        None,
        Some(msg.author.id),
        None,
        None,
    )
    .await;

    let components = build_category_components(&categories, &default_label);

    let sent = MessageBuilder::system_message(ctx, config)
        .translated_content(
            "category.prompt_message",
            Some(&params),
            Some(msg.author.id),
            None,
        )
        .await
        .to_user(msg.author.id)
        .components(components)
        .send(true)
        .await;

    let prompt = match sent {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to send category prompt: {e:?}");
            return false;
        }
    };

    let now = Utc::now().timestamp();
    let expires_at = now + settings.selection_timeout_s;
    let dm_channel_id = prompt.channel_id.to_string();

    if let Err(e) = upsert_pending_selection(
        msg.author.id.get() as i64,
        &prompt.id.to_string(),
        &dm_channel_id,
        now,
        expires_at,
        &[msg.id.to_string()],
        pool,
    )
    .await
    {
        eprintln!("Failed to persist pending selection: {e:?}");
        return false;
    }

    schedule_category_timeout(ctx, config, msg.author.id.get() as i64, expires_at);
    true
}

pub fn schedule_category_timeout(ctx: &Context, config: &Config, user_id: i64, expires_at: i64) {
    let ctx_clone = ctx.clone();
    let config_clone = config.clone();
    let now = Utc::now().timestamp();
    let delay = (expires_at - now).max(0) as u64;

    tokio::spawn(async move {
        if delay > 0 {
            sleep(Duration::from_secs(delay)).await;
        }
        let pool = match config_clone.db_pool.as_ref() {
            Some(p) => p,
            None => return,
        };
        match get_pending_selection(user_id, pool).await {
            Ok(Some(pending)) => {
                if pending.expires_at <= Utc::now().timestamp() {
                    if let Err(e) =
                        finalize_with_category(&ctx_clone, &config_clone, user_id, None).await
                    {
                        eprintln!("Failed to finalize expired selection: {e:?}");
                    }
                } else {
                    schedule_category_timeout(
                        &ctx_clone,
                        &config_clone,
                        user_id,
                        pending.expires_at,
                    );
                }
            }
            _ => {}
        }
    });
}

pub async fn finalize_with_category(
    ctx: &Context,
    config: &Config,
    user_id: i64,
    category_id: Option<&str>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = match &config.db_pool {
        Some(p) => p,
        None => return Ok(()),
    };

    let pending = match get_pending_selection(user_id, pool).await? {
        Some(p) => p,
        None => return Ok(()),
    };

    let user = UserId::new(user_id as u64);

    let (discord_override, ticket_cat_id) = if let Some(id) = category_id {
        match get_category_by_id(id, pool).await? {
            Some(cat) if cat.enabled => {
                let parent = cat.discord_category_id.parse::<u64>().ok();
                (parent, Some(cat.id.clone()))
            }
            Some(_) | None => (None, None),
        }
    } else {
        (None, None)
    };

    let (target_channel_id, _is_new) = create_or_get_thread_for_user(
        ctx,
        config,
        user,
        discord_override,
        ticket_cat_id.as_deref(),
    )
    .await?;

    let dm_channel = ChannelId::new(pending.dm_channel_id.parse::<u64>().unwrap_or(0));
    for mid in &pending.queued_msg_ids {
        if let Ok(id_u64) = mid.parse::<u64>() {
            if let Ok(m) = dm_channel.message(&ctx.http, id_u64).await {
                if let Err(e) = send_to_thread(ctx, target_channel_id, &m, config, false).await {
                    eprintln!("Failed to forward queued DM message: {e:?}");
                }
            }
        }
    }

    if !delete_pending_selection(user_id, pool).await? {
        return Err("Failed to clear pending selection".into());
    }

    Ok(())
}

pub async fn handle_category_component_interaction(
    ctx: &Context,
    config: &Config,
    interaction: &ComponentInteraction,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let custom_id = &interaction.data.custom_id;
    if !custom_id.starts_with("category:") {
        return Ok(false);
    }

    let _ = interaction
        .create_response(
            &ctx.http,
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new().components(vec![]),
            ),
        )
        .await;

    let user_id = interaction.user.id.get() as i64;

    let category_id: Option<String> = if custom_id == "category:default" {
        None
    } else if let Some(rest) = custom_id.strip_prefix("category:pick:") {
        Some(rest.to_string())
    } else {
        return Ok(true);
    };

    finalize_with_category(ctx, config, user_id, category_id.as_deref()).await?;
    Ok(true)
}

pub async fn hydrate_pending_category_selections(ctx: &Context, config: &Config) {
    let Some(pool) = config.db_pool.as_ref() else {
        return;
    };
    let list = match list_all_pending_selections(pool).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to load pending category selections: {e:?}");
            return;
        }
    };
    for pending in list {
        if pending.expires_at <= Utc::now().timestamp() {
            if let Err(e) = finalize_with_category(ctx, config, pending.user_id, None).await {
                eprintln!("Failed to hydrate-finalize pending selection: {e:?}");
            }
        } else {
            schedule_category_timeout(ctx, config, pending.user_id, pending.expires_at);
        }
    }
}
