use crate::db::operations::ticket_categories::CATEGORY_BUTTON_HARD_LIMIT;
use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn category_command(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let content = match extract_reply_content(&msg.content, &config.command.prefix, &["category"]) {
        Some(c) => c,
        None => {
            return send_translated(&ctx, config, &msg, "category.text_usage", None).await;
        }
    };

    let mut parts = content.splitn(2, ' ');
    let sub = parts.next().unwrap_or("").trim();
    let args = parts.next().unwrap_or("").trim();

    match sub {
        "create" => handle_create(&ctx, &msg, args, pool, config).await,
        "list" => handle_list(&ctx, &msg, pool, config).await,
        "rename" => handle_rename(&ctx, &msg, args, pool, config).await,
        "move" => handle_move(&ctx, &msg, args, pool, config).await,
        "delete" | "remove" => handle_delete(&ctx, &msg, args, pool, config).await,
        "enable" => handle_enable_one(&ctx, &msg, args, pool, config, true).await,
        "disable" => handle_enable_one(&ctx, &msg, args, pool, config, false).await,
        "timeout" => handle_timeout(&ctx, &msg, args, pool, config).await,
        "on" => handle_feature_toggle(&ctx, &msg, pool, config, true).await,
        "off" => handle_feature_toggle(&ctx, &msg, pool, config, false).await,
        _ => send_translated(&ctx, config, &msg, "category.unknown_subcommand", None).await,
    }
}

async fn send_translated(
    ctx: &Context,
    config: &Config,
    msg: &Message,
    key: &str,
    params: Option<&HashMap<String, String>>,
) -> ModmailResult<()> {
    let mut p = HashMap::new();
    p.insert("prefix".to_string(), config.command.prefix.clone());
    if let Some(extra) = params {
        for (k, v) in extra {
            p.insert(k.clone(), v.clone());
        }
    }
    MessageBuilder::system_message(ctx, config)
        .translated_content(
            key,
            Some(&p),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .reply_to(msg.clone())
        .send(true)
        .await?;
    Ok(())
}

async fn handle_create(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    // Format: <discord_category_id> <name> [| description] [| emoji]
    let mut parts = args.splitn(2, ' ');
    let discord_id_raw = parts.next().unwrap_or("").trim();
    let rest = parts.next().unwrap_or("").trim();

    if discord_id_raw.is_empty() || rest.is_empty() {
        return send_translated(ctx, config, msg, "category.create_usage", None).await;
    }

    if discord_id_raw.parse::<u64>().is_err() {
        return send_translated(ctx, config, msg, "category.invalid_discord_category", None).await;
    }

    let segments: Vec<&str> = rest.split('|').map(|s| s.trim()).collect();
    let name = segments.first().map(|s| s.to_string()).unwrap_or_default();
    if name.is_empty() {
        return send_translated(ctx, config, msg, "category.create_usage", None).await;
    }
    let description = segments
        .get(1)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty());
    let emoji = segments
        .get(2)
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty());

    let enabled = list_enabled_categories(pool).await?;
    if enabled.len() >= CATEGORY_BUTTON_HARD_LIMIT {
        let mut params = HashMap::new();
        params.insert("max".to_string(), CATEGORY_BUTTON_HARD_LIMIT.to_string());
        return send_translated(ctx, config, msg, "category.too_many_enabled", Some(&params)).await;
    }

    if let Some(_) = get_category_by_name(&name, pool).await? {
        return send_translated(ctx, config, msg, "category.already_exists", None).await;
    }

    let created = create_category(
        &name,
        description.as_deref(),
        emoji.as_deref(),
        discord_id_raw,
        pool,
    )
    .await?;

    let mut params = HashMap::new();
    params.insert("name".to_string(), created.name.clone());
    send_translated(ctx, config, msg, "category.created", Some(&params)).await
}

async fn handle_list(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let cats = list_all_categories(pool).await?;
    if cats.is_empty() {
        return send_translated(ctx, config, msg, "category.list_empty", None).await;
    }

    let header = get_translated_message(
        config,
        "category.list_header",
        None,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    let enabled_label = get_translated_message(
        config,
        "category.state_enabled",
        None,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;
    let disabled_label = get_translated_message(
        config,
        "category.state_disabled",
        None,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let mut body = format!("**{}**\n\n", header);
    for cat in &cats {
        let state = if cat.enabled {
            enabled_label.clone()
        } else {
            disabled_label.clone()
        };
        let emoji = cat.emoji.clone().unwrap_or_default();
        body.push_str(&format!(
            "`{}` {} **{}** — {}\n",
            cat.position, emoji, cat.name, state
        ));
    }

    MessageBuilder::system_message(ctx, config)
        .content(body)
        .reply_to(msg.clone())
        .send(true)
        .await?;
    Ok(())
}

async fn handle_rename(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let old = parts.next().unwrap_or("").trim();
    let new = parts.next().unwrap_or("").trim();
    if old.is_empty() || new.is_empty() {
        return send_translated(ctx, config, msg, "category.text_usage", None).await;
    }
    let cat = match get_category_by_name(old, pool).await? {
        Some(c) => c,
        None => return send_translated(ctx, config, msg, "category.not_found", None).await,
    };
    if let Some(existing) = get_category_by_name(new, pool).await? {
        if existing.id != cat.id {
            return send_translated(ctx, config, msg, "category.already_exists", None).await;
        }
    }
    update_category(&cat.id, Some(new), None, None, None, None, None, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), new.to_string());
    send_translated(ctx, config, msg, "category.renamed", Some(&params)).await
}

async fn handle_move(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let name = parts.next().unwrap_or("").trim();
    let pos_s = parts.next().unwrap_or("").trim();
    let position: i64 = match pos_s.parse() {
        Ok(p) => p,
        Err(_) => return send_translated(ctx, config, msg, "category.text_usage", None).await,
    };
    let cat = match get_category_by_name(name, pool).await? {
        Some(c) => c,
        None => return send_translated(ctx, config, msg, "category.not_found", None).await,
    };
    update_category(&cat.id, None, None, None, None, Some(position), None, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    params.insert("position".to_string(), position.to_string());
    send_translated(ctx, config, msg, "category.moved", Some(&params)).await
}

async fn handle_delete(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let name = args.trim();
    if name.is_empty() {
        return send_translated(ctx, config, msg, "category.text_usage", None).await;
    }
    let cat = match get_category_by_name(name, pool).await? {
        Some(c) => c,
        None => return send_translated(ctx, config, msg, "category.not_found", None).await,
    };
    delete_category(&cat.id, pool).await?;
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    send_translated(ctx, config, msg, "category.deleted", Some(&params)).await
}

async fn handle_enable_one(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
    enable: bool,
) -> ModmailResult<()> {
    let name = args.trim();
    if name.is_empty() {
        return send_translated(ctx, config, msg, "category.text_usage", None).await;
    }
    let cat = match get_category_by_name(name, pool).await? {
        Some(c) => c,
        None => return send_translated(ctx, config, msg, "category.not_found", None).await,
    };
    if enable && !cat.enabled {
        let enabled_count = count_enabled_categories(pool).await?;
        if enabled_count as usize >= CATEGORY_BUTTON_HARD_LIMIT {
            let mut params = HashMap::new();
            params.insert("max".to_string(), CATEGORY_BUTTON_HARD_LIMIT.to_string());
            return send_translated(ctx, config, msg, "category.too_many_enabled", Some(&params))
                .await;
        }
    }
    update_category(&cat.id, None, None, None, None, None, Some(enable), pool).await?;
    let key = if enable {
        "category.enabled_one"
    } else {
        "category.disabled_one"
    };
    let mut params = HashMap::new();
    params.insert("name".to_string(), cat.name);
    send_translated(ctx, config, msg, key, Some(&params)).await
}

async fn handle_timeout(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let secs: i64 = match args.trim().parse() {
        Ok(v) if v >= 30 => v,
        _ => return send_translated(ctx, config, msg, "category.text_usage", None).await,
    };
    let settings = get_category_settings(pool).await?;
    update_category_settings(settings.enabled, secs, pool).await?;
    let mut params = HashMap::new();
    params.insert("seconds".to_string(), secs.to_string());
    send_translated(ctx, config, msg, "category.timeout_updated", Some(&params)).await
}

async fn handle_feature_toggle(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::SqlitePool,
    config: &Config,
    enable: bool,
) -> ModmailResult<()> {
    let settings = get_category_settings(pool).await?;
    update_category_settings(enable, settings.selection_timeout_s, pool).await?;
    let key = if enable {
        "category.feature_enabled"
    } else {
        "category.feature_disabled"
    };
    send_translated(ctx, config, msg, key, None).await
}
