use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::i18n::*;
use crate::prelude::utils::*;
use regex::Regex;
use serenity::all::{Context, Message};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn snippet_command(
    ctx: Context,
    msg: Message,
    config: &Config,
    _handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let pool = config
        .db_pool
        .as_ref()
        .ok_or_else(database_connection_failed)?;

    let content = match extract_reply_content(&msg.content, &config.command.prefix, &["snippet"]) {
        Some(c) => c,
        None => {
            MessageBuilder::system_message(&ctx, config)
                .translated_content(
                    "snippet.text_usage",
                    None,
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await
                .reply_to(msg)
                .send(true)
                .await?;
            return Ok(());
        }
    };

    let mut parts = content.splitn(2, ' ');
    let subcommand = parts.next().unwrap_or("").trim();
    let args = parts.next().unwrap_or("").trim();

    match subcommand {
        "create" => handle_create(&ctx, &msg, args, pool, config).await,
        "list" => handle_list(&ctx, &msg, pool, config).await,
        "show" => handle_show(&ctx, &msg, args, pool, config).await,
        "edit" => handle_edit(&ctx, &msg, args, pool, config).await,
        "delete" => handle_delete(&ctx, &msg, args, pool, config).await,
        _ => {
            MessageBuilder::system_message(&ctx, config)
                .translated_content(
                    "snippet.unknown_text_subcommand",
                    None,
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await
                .reply_to(msg)
                .send(true)
                .await?;
            Ok(())
        }
    }
}

async fn handle_create(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let key = parts.next().unwrap_or("").trim();
    let content = parts.next().unwrap_or("").trim();

    if key.is_empty() || content.is_empty() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.text_create_usage",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await?;
        return Ok(());
    }

    let key_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !key_regex.is_match(key) {
        return Err(ModmailError::Command(CommandError::InvalidSnippetKeyFormat));
    }

    if content.len() > 4000 {
        return Err(ModmailError::Command(CommandError::SnippetContentTooLong));
    }

    let created_by = msg.author.id.to_string();
    match create_snippet(key, content, &created_by, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetAlreadyExists(
                key.to_string(),
            )));
        }
    }

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());

    MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.created",
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .reply_to(msg.clone())
        .send(true)
        .await?;

    Ok(())
}

async fn handle_list(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let snippets = get_all_snippets(pool).await?;

    if snippets.is_empty() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.list_empty",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await?;
        return Ok(());
    }

    let title = get_translated_message(
        config,
        "snippet.list_title",
        None,
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let mut response = format!("{}\n\n", title);
    for (index, snippet) in snippets.iter().enumerate() {
        response.push_str(&format!("`{}` {}\n\n", index + 1, snippet.key));
    }

    MessageBuilder::system_message(ctx, config)
        .content(response)
        .reply_to(msg.clone())
        .send(true)
        .await?;

    Ok(())
}

async fn handle_show(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let key = args.trim();

    if key.is_empty() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.text_show_usage",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await?;
        return Ok(());
    }

    match get_snippet_by_key(key, pool).await? {
        Some(snippet) => {
            let mut params = HashMap::new();
            params.insert("key".to_string(), snippet.key.clone());

            let title = get_translated_message(
                config,
                "snippet.show_title",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let created_by_label = get_translated_message(
                config,
                "snippet.created_by",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let created_at_label = get_translated_message(
                config,
                "snippet.created_at",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
                None,
            )
            .await;

            let response = format!(
                "{}\n\n{}\n\n*{}: <@{}> | {}: {}*",
                title,
                snippet.content,
                created_by_label,
                snippet.created_by,
                created_at_label,
                snippet.created_at
            );

            MessageBuilder::system_message(ctx, config)
                .content(response)
                .reply_to(msg.clone())
                .send(true)
                .await?;
        }
        None => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    }

    Ok(())
}

async fn handle_edit(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let key = parts.next().unwrap_or("").trim();
    let content = parts.next().unwrap_or("").trim();

    if key.is_empty() || content.is_empty() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.text_edit_usage",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await?;
        return Ok(());
    }

    if content.len() > 4000 {
        return Err(ModmailError::Command(CommandError::SnippetContentTooLong));
    }

    match update_snippet(key, content, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    };

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());

    MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.updated",
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .reply_to(msg.clone())
        .send(true)
        .await?;

    Ok(())
}

async fn handle_delete(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
    config: &Config,
) -> ModmailResult<()> {
    let key = args.trim();

    if key.is_empty() {
        MessageBuilder::system_message(ctx, config)
            .translated_content(
                "snippet.text_delete_usage",
                None,
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .reply_to(msg.clone())
            .send(true)
            .await?;
        return Ok(());
    }

    match delete_snippet(&key, pool).await {
        Ok(_) => {}
        Err(_) => {
            return Err(ModmailError::Command(CommandError::SnippetNotFound(
                key.to_string(),
            )));
        }
    };

    let mut params = HashMap::new();
    params.insert("key".to_string(), key.to_string());

    MessageBuilder::system_message(ctx, config)
        .translated_content(
            "snippet.deleted",
            Some(&params),
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await
        .reply_to(msg.clone())
        .send(true)
        .await?;

    Ok(())
}
