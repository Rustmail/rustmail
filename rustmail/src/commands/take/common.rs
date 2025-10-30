use crate::prelude::config::*;
use crate::prelude::errors::*;
use crate::prelude::utils::*;
use serenity::all::{ChannelId, CommandInteraction, Context, EditChannel, Message};
use std::time::Duration;
use tokio::time::sleep;

pub async fn rename_channel_with_timeout(
    ctx: &Context,
    config: &Config,
    channel_id: ChannelId,
    new_name: String,
    msg: Option<&Message>,
    command: Option<&CommandInteraction>,
) -> ModmailResult<()> {
    let rename_future = channel_id.edit(
        &ctx.http,
        EditChannel::new().name(new_name.clone()),
    );
    let timeout = sleep(Duration::from_secs(2));

    tokio::select! {
        res = rename_future => {
            if let Err(e) = res {
                return Err(ModmailError::Discord(DiscordError::ApiError(e.to_string())));
            }
            return Ok(());
        }
        _ = timeout => {
            let message_response: Option<Message> = if let Some(message) = msg {
                let response = MessageBuilder::system_message(ctx, config)
                    .translated_content("take.timeout", None, None, None).await
                    .to_channel(message.channel_id)
                    .send(true)
                    .await;

                match response {
                    Ok(msg) => Some(msg),
                    Err(_) => None,
                }
            } else {
                None
            };

            let command_response: Option<Message> = if let Some(command) = command {
                let message = MessageBuilder::system_message(ctx, config)
                    .translated_content("take.timeout", None, None, None).await
                    .to_channel(command.channel_id)
                    .build_interaction_message_followup()
                    .await;

                match command.create_followup(&ctx.http, message).await {
                    Ok(msg) => Some(msg),
                    Err(_) => None,
                }
            } else {
                None
            };

            let _ = channel_id.edit(&ctx.http, EditChannel::new().name(new_name)).await;

            if let Some(m) = message_response {
                let _ = m.delete(&ctx.http).await;
            }
            if let Some(m) = command_response {
                let _ = m.delete(&ctx.http).await;
            }

            return Ok(());
        }
    }
}
