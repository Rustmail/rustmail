use serenity::all::{ComponentInteraction, Context, EventHandler, Interaction};
use serenity::async_trait;
use crate::config::Config;
use crate::features::handle_feature_interaction;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use crate::modules::threads::handle_thread_interaction;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Config,
}

impl InteractionHandler {
    pub fn new(config: &Config) -> Self {
        Self { config: config.clone() }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Component(mut comp) => {
                if let Err(e) = handle_feature_interaction(&ctx, &self.config, &comp).await {
                    eprintln!("Error handling feature component: {}", e);
                    return;
                }
                if let Err(e) = handle_thread_interaction(&ctx, &self.config, &mut comp).await {
                    eprintln!("Error handling thread component: {}", e);
                    return;
                }
            }
            Interaction::Modal(modal) => {
                let _ = modal.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new().content(format!("Feature actionné {}", modal.data.custom_id)).ephemeral(true)
                )).await;
                return;
            }
            _ => {}
        }
    }
}
