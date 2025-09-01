use crate::config::Config;
use crate::features::handle_feature_component_interaction;
use crate::modules::threads::{
    handle_thread_component_interaction, handle_thread_modal_interaction,
};
use serenity::all::{Context, EventHandler, Interaction};
use serenity::async_trait;
use serenity::builder::CreateInteractionResponse;
use crate::utils::message::message_builder::MessageBuilder;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Config,
}

impl InteractionHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for InteractionHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Component(mut comp) => {
                if let Err(..) =
                    handle_feature_component_interaction(&ctx, &self.config, &comp).await
                {
                    return;
                }
                if let Err(..) =
                    handle_thread_component_interaction(&ctx, &self.config, &mut comp).await
                {
                    return;
                }
            }
            Interaction::Modal(mut modal) => {
                if let Err(..) =
                    handle_thread_modal_interaction(&ctx, &self.config, &mut modal).await
                {
                    return;
                }
            }
            _ => {}
        }
    }
}
