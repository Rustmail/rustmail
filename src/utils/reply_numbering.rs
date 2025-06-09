use crate::config::Config;
use crate::db::operations::get_message_number_by_id;
use serenity::all::{Context, Message};

pub async fn show_message_number(
    ctx: &Context,
    sent_message: &Message,
    config: &Config,
) -> Option<()> {
    let pool = config.db_pool.as_ref()?;

    let message_number = get_message_number_by_id(&sent_message.id.to_string(), pool).await?;

    let confirmation_text = format!(
        "✅ Message envoyé ! (Message #{}) - Utilisez `{}edit {}` pour modifier ce message.",
        message_number, config.command.prefix, message_number
    );

    if let Ok(confirmation_msg) = sent_message
        .channel_id
        .say(&ctx.http, confirmation_text)
        .await
    {
        let http = ctx.http.clone();
        let msg_id = confirmation_msg.id;
        let channel_id = confirmation_msg.channel_id;

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            let _ = channel_id.delete_message(&http, msg_id).await;
        });
    }

    Some(())
}

pub async fn get_next_message_number_for_thread(thread_id: &str, config: &Config) -> Option<u64> {
    let pool = config.db_pool.as_ref()?;
    Some(crate::db::operations::get_next_message_number(thread_id, pool).await)
}

pub fn format_message_number_preview(message_number: u64, prefix: &str) -> String {
    format!(
        "(Message #{} - Utilisez `{}edit {}` pour modifier)",
        message_number, prefix, message_number
    )
}

pub fn add_message_number_to_embed_footer(
    embed: serenity::all::CreateEmbed,
    message_number: u64,
    prefix: &str,
) -> serenity::all::CreateEmbed {
    let footer_text = format!(
        "Message #{} • {}edit {} pour modifier",
        message_number, prefix, message_number
    );
    embed.footer(serenity::all::CreateEmbedFooter::new(footer_text))
}

pub fn add_message_number_to_text(content: &str, message_number: u64, prefix: &str) -> String {
    format!(
        "{}\n\n*Message #{} - `{}edit {}` pour modifier*",
        content, message_number, prefix, message_number
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_message_number_preview() {
        let result = format_message_number_preview(5, "!");
        assert_eq!(result, "(Message #5 - Utilisez `!edit 5` pour modifier)");
    }

    #[test]
    fn test_add_message_number_to_text() {
        let content = "Hello world";
        let result = add_message_number_to_text(content, 3, "!");
        assert_eq!(
            result,
            "Hello world\n\n*Message #3 - `!edit 3` pour modifier*"
        );
    }

    #[test]
    fn test_format_message_number_preview_different_prefix() {
        let result = format_message_number_preview(10, "?");
        assert_eq!(result, "(Message #10 - Utilisez `?edit 10` pour modifier)");
    }
}
