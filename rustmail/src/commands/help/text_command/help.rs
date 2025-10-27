use crate::config::Config;
use crate::errors::ModmailResult;
use crate::handlers::guild_messages_handler::GuildMessagesHandler;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message};
use std::sync::Arc;

pub async fn help(
    ctx: Context,
    msg: Message,
    config: &Config,
    handler: Arc<GuildMessagesHandler>,
) -> ModmailResult<()> {
    let mut docs_message = String::new();

    for (name, command) in &handler.registry.commands {
        let doc = command.doc(&config).await;

        docs_message.push_str(&format!("**{}** — {}\n\n", name, doc))
    }

    MessageBuilder::system_message(&ctx, config)
        .content(docs_message)
        .to_channel(msg.channel_id)
        .send(false)
        .await?;

    Ok(())
}
