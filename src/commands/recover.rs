use crate::config::Config;
use crate::errors::{ModmailResult, common};
use crate::i18n::get_translated_message;
use crate::modules::message_recovery::recover_missing_messages;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message};
use std::collections::HashMap;

pub async fn recover(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let _ = config
        .db_pool
        .as_ref()
        .ok_or_else(|| common::database_connection_failed())?;

    let mut params = HashMap::new();
    params.insert("user".to_string(), msg.author.name.clone());

    let confirmation_message = get_translated_message(
        config,
        "recovery.started",
        Some(&params),
        Some(msg.author.id),
        msg.guild_id.map(|g| g.get()),
        None,
    )
    .await;

    let _ = MessageBuilder::system_message(&ctx, config)
        .content(confirmation_message)
        .to_channel(msg.channel_id)
        .send()
        .await;

    let ctx_clone = ctx.clone();
    let config_clone = config.clone();
    let channel_id = msg.channel_id;

    tokio::spawn(async move {
        let recovery_results = recover_missing_messages(&ctx_clone, &config_clone).await;

        let total_recovered: u32 = recovery_results.iter().map(|r| r.recovered_count).sum();
        let successful_threads = recovery_results.iter().filter(|r| r.success).count();
        let failed_threads = recovery_results.len() - successful_threads;

        let mut params = HashMap::new();
        params.insert("total".to_string(), total_recovered.to_string());
        params.insert("threads".to_string(), successful_threads.to_string());
        params.insert("failed".to_string(), failed_threads.to_string());

        let summary_message = get_translated_message(
            &config_clone,
            "recovery.summary",
            Some(&params),
            None,
            None,
            None,
        )
        .await;

        let _ = MessageBuilder::system_message(&ctx_clone, &config_clone)
            .content(summary_message)
            .to_channel(channel_id)
            .send()
            .await;
    });

    Ok(())
}
