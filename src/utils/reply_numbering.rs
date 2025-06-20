use crate::config::Config;
use crate::db::operations::get_message_number_by_id;
use serenity::all::{Context, Message};
use crate::i18n::get_translated_message;
use std::collections::HashMap;

pub async fn show_message_number(
    ctx: &Context,
    sent_message: &Message,
    config: &Config,
) -> Option<()> {
    let pool = config.db_pool.as_ref()?;

    let message_number = get_message_number_by_id(&sent_message.id.to_string(), pool).await?;

    let mut params = HashMap::new();
    params.insert("number".to_string(), message_number.to_string());
    params.insert("prefix".to_string(), config.command.prefix.clone());
    let confirmation_text = get_translated_message(
        config,
        "reply_numbering.confirmation",
        Some(&params),
        Some(sent_message.author.id),
        sent_message.guild_id.map(|g| g.get()),
        None
    ).await;

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

pub async fn format_message_number_preview(message_number: u64, prefix: &str, config: &Config, msg: &Message) -> String {
    let mut params = HashMap::new();
    params.insert("number".to_string(), message_number.to_string());
    params.insert("prefix".to_string(), prefix.to_string());
    get_translated_message(
        config,
        "reply_numbering.preview",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None
    ).await
}

pub async fn add_message_number_to_embed_footer(
    embed: serenity::all::CreateEmbed,
    message_number: u64,
    prefix: &str,
    config: &Config,
    msg: &Message,
) -> serenity::all::CreateEmbed {
    let mut params = HashMap::new();
    params.insert("number".to_string(), message_number.to_string());
    params.insert("prefix".to_string(), prefix.to_string());
    let footer_text = get_translated_message(
        config,
        "reply_numbering.footer",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None
    ).await;
    embed.footer(serenity::all::CreateEmbedFooter::new(footer_text))
}

pub async fn add_message_number_to_text(content: &str, message_number: u64, prefix: &str, config: &Config, msg: &Message) -> String {
    let mut params = HashMap::new();
    params.insert("number".to_string(), message_number.to_string());
    params.insert("prefix".to_string(), prefix.to_string());
    let footer = get_translated_message(
        config,
        "reply_numbering.text_footer",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None
    ).await;
    format!("{}\n\n{}", content, footer)
}
