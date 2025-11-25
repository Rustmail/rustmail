use crate::prelude::config::*;
use crate::prelude::db::*;
use crate::prelude::errors::*;
use crate::prelude::handlers::*;
use crate::prelude::utils::*;
use regex::Regex;
use serenity::all::{Context, Message};
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
            msg.reply(
                &ctx.http,
                "❌ Usage: `!snippet <create|list|show|edit|delete> [args]`",
            )
            .await?;
            return Ok(());
        }
    };

    let mut parts = content.splitn(2, ' ');
    let subcommand = parts.next().unwrap_or("").trim();
    let args = parts.next().unwrap_or("").trim();

    match subcommand {
        "create" => handle_create(&ctx, &msg, args, pool).await,
        "list" => handle_list(&ctx, &msg, pool).await,
        "show" => handle_show(&ctx, &msg, args, pool).await,
        "edit" => handle_edit(&ctx, &msg, args, pool).await,
        "delete" => handle_delete(&ctx, &msg, args, pool).await,
        _ => {
            msg.reply(
                &ctx.http,
                "❌ Unknown subcommand. Use: `create`, `list`, `show`, `edit`, or `delete`",
            )
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
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let key = parts.next().unwrap_or("").trim();
    let content = parts.next().unwrap_or("").trim();

    if key.is_empty() || content.is_empty() {
        msg.reply(
            &ctx.http,
            "❌ Usage: `!snippet create <key> <content>`",
        )
        .await?;
        return Ok(());
    }

    let key_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    if !key_regex.is_match(key) {
        msg.reply(
            &ctx.http,
            "❌ Snippet key must contain only alphanumeric characters, dashes, and underscores.",
        )
        .await?;
        return Ok(());
    }

    if content.len() > 4000 {
        msg.reply(
            &ctx.http,
            "❌ Snippet content must be 4000 characters or less.",
        )
        .await?;
        return Ok(());
    }

    let created_by = msg.author.id.to_string();
    match create_snippet(key, content, &created_by, pool).await {
        Ok(_) => {
            msg.reply(&ctx.http, format!("✅ Snippet `{}` created successfully!", key))
                .await?;
        }
        Err(e) => {
            msg.reply(&ctx.http, format!("❌ Failed to create snippet: {}", e))
                .await?;
        }
    }

    Ok(())
}

async fn handle_list(
    ctx: &Context,
    msg: &Message,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let snippets = get_all_snippets(pool).await?;

    if snippets.is_empty() {
        msg.reply(&ctx.http, "📝 No snippets found.").await?;
        return Ok(());
    }

    let mut response = String::from("📝 **Available Snippets:**\n\n");
    for snippet in snippets.iter().take(25) {
        let preview = if snippet.content.len() > 50 {
            format!("{}...", &snippet.content[..50])
        } else {
            snippet.content.clone()
        };
        response.push_str(&format!("**`{}`** - {}\n", snippet.key, preview));
    }

    if snippets.len() > 25 {
        response.push_str(&format!("\n*...and {} more*", snippets.len() - 25));
    }

    msg.reply(&ctx.http, response).await?;
    Ok(())
}

async fn handle_show(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let key = args.trim();

    if key.is_empty() {
        msg.reply(&ctx.http, "❌ Usage: `!snippet show <key>`")
            .await?;
        return Ok(());
    }

    match get_snippet_by_key(key, pool).await? {
        Some(snippet) => {
            let response = format!(
                "📝 **Snippet: {}**\n\n{}\n\n*Created by <@{}> at {}*",
                snippet.key, snippet.content, snippet.created_by, snippet.created_at
            );
            msg.reply(&ctx.http, response).await?;
        }
        None => {
            msg.reply(&ctx.http, format!("❌ Snippet `{}` not found.", key))
                .await?;
        }
    }

    Ok(())
}

async fn handle_edit(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let mut parts = args.splitn(2, ' ');
    let key = parts.next().unwrap_or("").trim();
    let content = parts.next().unwrap_or("").trim();

    if key.is_empty() || content.is_empty() {
        msg.reply(&ctx.http, "❌ Usage: `!snippet edit <key> <content>`")
            .await?;
        return Ok(());
    }

    if content.len() > 4000 {
        msg.reply(
            &ctx.http,
            "❌ Snippet content must be 4000 characters or less.",
        )
        .await?;
        return Ok(());
    }

    match update_snippet(key, content, pool).await {
        Ok(_) => {
            msg.reply(&ctx.http, format!("✅ Snippet `{}` updated successfully!", key))
                .await?;
        }
        Err(e) => {
            msg.reply(&ctx.http, format!("❌ Failed to update snippet: {}", e))
                .await?;
        }
    }

    Ok(())
}

async fn handle_delete(
    ctx: &Context,
    msg: &Message,
    args: &str,
    pool: &sqlx::SqlitePool,
) -> ModmailResult<()> {
    let key = args.trim();

    if key.is_empty() {
        msg.reply(&ctx.http, "❌ Usage: `!snippet delete <key>`")
            .await?;
        return Ok(());
    }

    match delete_snippet(key, pool).await {
        Ok(_) => {
            msg.reply(&ctx.http, format!("✅ Snippet `{}` deleted successfully!", key))
                .await?;
        }
        Err(e) => {
            msg.reply(&ctx.http, format!("❌ Failed to delete snippet: {}", e))
                .await?;
        }
    }

    Ok(())
}
