use std::sync::Arc;
use serenity::all::{ChannelId, ComponentInteraction, Context, CreateMessage};
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::all::ButtonStyle;
use async_trait::async_trait;
use crate::config::Config;
use crate::db::operations::{get_feature_message, upsert_feature_message};

mod poll;

pub use poll::PollFeature;

#[async_trait]
pub trait Feature<'a>: Send + Sync {
    fn key(&self) -> &'static str;
    async fn build_message(&self, ctx: &'a Context, config: &'a Config) -> CreateMessage;
    async fn handle_interaction(
        &self,
        ctx: &Context,
        config: &Config,
        interaction: &ComponentInteraction,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

pub fn registry<'a>() -> Vec<Arc<dyn Feature<'a>>> {
    vec![
        Arc::new(PollFeature::default()),
    ]
}

pub fn make_buttons(pairs: &[(&str, &str, ButtonStyle)]) -> Vec<CreateActionRow> {
    let mut row = CreateActionRow::Buttons(vec![]);
    let mut buttons: Vec<CreateButton> = Vec::new();
    for (label, custom_id, style) in pairs {
        let b = CreateButton::new(custom_id.to_string())
            .label(label.to_string())
            .style(*style);
        buttons.push(b);
    }

    if let CreateActionRow::Buttons(ref mut v) = row {
        v.extend(buttons);
    }

    vec![row]
}

pub async fn sync_features(ctx: &Context, config: &Config) {
    if !config.bot.enable_features {
        return;
    }
    let channel_id_u64 = match config.bot.features_channel_id {
        Some(id) => id,
        None => return,
    };
    let channel_id = ChannelId::new(channel_id_u64);

    for feature in registry() {
        let key = feature.key();
        let pool = match &config.db_pool {
            Some(p) => p,
            None => continue,
        };

        let mut send_new = false;

        if let Ok(Some((stored_channel, stored_message))) = get_feature_message(key, pool).await {
            if let (Ok(stored_channel_id), Ok(stored_message_id)) = (
                stored_channel.parse::<u64>(),
                stored_message.parse::<u64>(),
            ) {
                let ch = ChannelId::new(stored_channel_id);
                if ch.message(&ctx.http, stored_message_id).await.is_err() {
                    send_new = true;
                }
            } else {
                send_new = true;
            }
        } else {
            send_new = true;
        }

        if send_new {
            let msg_create = feature.build_message(ctx, config).await;
            match channel_id.send_message(&ctx.http, msg_create).await {
                Ok(sent) => {
                    let _ = upsert_feature_message(
                        key,
                        &channel_id_u64.to_string(),
                        &sent.id.to_string(),
                        pool,
                    )
                    .await;
                }
                Err(err) => {
                    eprintln!("Failed to send feature {}: {}", key, err);
                }
            }
        }
    }
}

fn parse_custom_id(custom_id: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = custom_id.split(':').collect();
    if parts.len() >= 3 && parts[0] == "feature" {
        Some((parts[1].to_string(), parts[2].to_string()))
    } else {
        None
    }
}

pub async fn handle_feature_interaction(
    ctx: &Context,
    config: &Config,
    interaction: &ComponentInteraction,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    if let Some((key, action)) = parse_custom_id(&interaction.data.custom_id) {
        for f in registry() {
            if f.key() == key {
                f.handle_interaction(ctx, config, interaction, &action).await?;
                return Ok(true);
            }
        }
    }
    Ok(false)
}
