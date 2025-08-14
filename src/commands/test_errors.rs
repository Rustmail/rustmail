use crate::config::Config;
use crate::errors::{
    CommandError, DatabaseError, DiscordError, MessageError, ModmailError, ModmailResult,
    ValidationError, common,
};
use serenity::all::{Colour, Context, CreateEmbed, CreateMessage, Message};
use std::collections::HashMap;

pub async fn test_errors(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let content = msg.content.trim();
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(ModmailError::Command(CommandError::MissingArguments));
    }

    let error_type = parts[1].to_lowercase();

    let demo_error = match error_type.as_str() {
        "database" | "db" => ModmailError::Database(DatabaseError::ConnectionFailed),
        "discord" => ModmailError::Discord(DiscordError::PermissionDenied),
        "command" | "cmd" => ModmailError::Command(CommandError::InvalidFormat),
        "validation" | "val" => ModmailError::Validation(ValidationError::InvalidInput(
            "Test validation error".to_string(),
        )),
        "message" | "msg" => ModmailError::Message(MessageError::MessageNotFound("reason".to_string())),
        "thread" => common::thread_not_found(),
        "permission" | "perm" => common::permission_denied(),
        "user" => common::user_not_found(),
        "channel" => common::channel_not_found(),
        "number" => common::message_number_not_found(42),
        "success" => {
            let mut params = HashMap::new();
            params.insert("number".to_string(), "5".to_string());

            let _ = error_handler
                .send_success_message(
                    ctx,
                    msg.channel_id,
                    "success.message_sent",
                    Some(params),
                    Some(msg.author.id),
                    msg.guild_id.map(|g| g.get()),
                )
                .await?;

            return Ok(());
        }
        _ => {
            let help_embed = CreateEmbed::new()
                .title("🔧 Error System Test Command")
                .description("Test the error handling system with different error types")
                .color(Colour::BLUE)
                .field(
                    "Usage",
                    format!("`{}test_errors <error_type>`", config.command.prefix),
                    false,
                )
                .field(
                    "Available Error Types",
                    "• `database` / `db` - Database connection error\n\
                     • `discord` - Discord permission error\n\
                     • `command` / `cmd` - Invalid command format\n\
                     • `validation` / `val` - Validation error\n\
                     • `message` / `msg` - Message not found\n\
                     • `thread` - Thread not found\n\
                     • `permission` / `perm` - Permission denied\n\
                     • `user` - User not found\n\
                     • `channel` - Channel not found\n\
                     • `number` - Message number not found\n\
                     • `success` - Show success message",
                    false,
                )
                .field(
                    "Language Support",
                    "Errors are automatically translated based on your language preference!",
                    false,
                )
                .footer(serenity::all::CreateEmbedFooter::new(
                    "Error messages will appear in your configured language",
                ));

            msg.channel_id
                .send_message(&ctx.http, CreateMessage::new().embed(help_embed))
                .await
                .map_err(|_| common::validation_failed("Failed to send help message"))?;

            return Ok(());
        }
    };

    Err(demo_error)
}

pub async fn test_language(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let content = msg.content.trim();
    let parts: Vec<&str> = content.split_whitespace().collect();

    if parts.len() < 2 {
        let help_embed = CreateEmbed::new()
            .title("🌍 Language Test Command")
            .description("Test error messages in different languages")
            .color(Colour::DARK_GREEN)
            .field(
                "Usage",
                format!("`{}test_language <language_code>`", config.command.prefix),
                false,
            )
            .field(
                "Supported Languages",
                "• `en` - English 🇺🇸\n\
                 • `fr` - Français 🇫🇷\n\
                 • `es` - Español 🇪🇸\n\
                 • `de` - Deutsch 🇩🇪\n\
                 • `it` - Italiano 🇮🇹\n\
                 • `pt` - Português 🇵🇹\n\
                 • `nl` - Nederlands 🇳🇱\n\
                 • `ru` - Русский 🇷🇺\n\
                 • `ja` - 日本語 🇯🇵\n\
                 • `ko` - 한국어 🇰🇷\n\
                 • `zh` - 中文 🇨🇳",
                false,
            )
            .footer(serenity::all::CreateEmbedFooter::new(
                "This will set your language preference and show a test error",
            ));

        msg.channel_id
            .send_message(&ctx.http, CreateMessage::new().embed(help_embed))
            .await
            .map_err(|_| common::validation_failed("Failed to send help message"))?;

        return Ok(());
    }

    let language_code = parts[1].to_lowercase();

    let language = crate::i18n::languages::Language::from_str(&language_code).ok_or_else(|| {
        ModmailError::Validation(ValidationError::InvalidInput(
            "Unsupported language code".to_string(),
        ))
    })?;

    let preferences = crate::i18n::languages::LanguagePreferences::new(language);
    error_handler
        .set_user_language(msg.author.id, preferences)
        .await;

    let confirmation_embed = CreateEmbed::new()
        .title("✅ Language Updated")
        .description(format!(
            "Your language has been set to: {} {}",
            language.native_name(),
            language.flag_emoji()
        ))
        .color(Colour::DARK_GREEN)
        .footer(serenity::all::CreateEmbedFooter::new(
            "Now triggering a test error in your new language...",
        ));

    msg.channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(confirmation_embed))
        .await
        .map_err(|_| common::validation_failed("Failed to send confirmation"))?;

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Err(common::thread_not_found())
}

pub async fn test_all_errors(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let error_handler = config
        .error_handler
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let embed = CreateEmbed::new()
        .title("🧪 Testing All Error Types")
        .description("This will demonstrate various error types in sequence...")
        .color(Colour::PURPLE)
        .footer(serenity::all::CreateEmbedFooter::new(
            "Each error will be shown in your configured language",
        ));

    let mut test_msg = msg
        .channel_id
        .send_message(&ctx.http, CreateMessage::new().embed(embed))
        .await
        .map_err(|_| common::validation_failed("Failed to send test message"))?;

    let error_tests = vec![
        (
            "Database Error",
            ModmailError::Database(DatabaseError::ConnectionFailed),
        ),
        (
            "Discord Error",
            ModmailError::Discord(DiscordError::PermissionDenied),
        ),
        (
            "Command Error",
            ModmailError::Command(CommandError::InvalidFormat),
        ),
        ("Thread Error", common::thread_not_found()),
        ("Message Error", common::message_not_found("reason")),
        ("User Error", common::user_not_found()),
        (
            "Validation Error",
            common::validation_failed("Test validation"),
        ),
        ("Permission Error", common::permission_denied()),
    ];

    for (error_name, error) in error_tests {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let updated_embed = CreateEmbed::new()
            .title("🧪 Testing All Error Types")
            .description(format!("Currently testing: **{}**", error_name))
            .color(Colour::PURPLE);

        let _ = test_msg
            .edit(
                &ctx.http,
                serenity::all::EditMessage::new().embed(updated_embed),
            )
            .await;

        let _ = error_handler.reply_with_error(ctx, msg, &error).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    let _ = error_handler
        .send_success_message(
            ctx,
            msg.channel_id,
            "success.message_sent",
            None,
            Some(msg.author.id),
            msg.guild_id.map(|g| g.get()),
        )
        .await?;

    let _ = test_msg.delete(&ctx.http).await;

    Ok(())
}
