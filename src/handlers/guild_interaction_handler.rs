use crate::commands::CommandRegistry;
use crate::config::Config;
use crate::features::handle_feature_component_interaction;
use crate::modules::threads::{
    handle_thread_component_interaction, handle_thread_modal_interaction,
};
use serenity::all::{Context, EventHandler, Interaction};
use std::sync::Arc;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Config,
    pub registry: Arc<CommandRegistry>,
}

impl InteractionHandler {
    pub fn new(config: &Config, register: Arc<CommandRegistry>) -> Self {
        Self {
            config: config.clone(),
            registry: register,
        }
    }
}

#[async_trait::async_trait]
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
            Interaction::Command(command) => {
                let ctx = ctx.clone();
                let command = command.clone();
                let options = command.data.options().clone();
                let config = self.config.clone();

                if let Some(handler) = self.registry.get(command.data.name.as_str()) {
                    let result = handler.run(&ctx, &command, &options, &config).await;

                    if let Err(e) = result {
                        if let Some(error_handler) = &self.config.error_handler {
                            let _ = error_handler
                                .reply_to_command_with_error(&ctx, &command, &e)
                                .await;
                        }
                    }
                } else {
                    eprintln!("Command {} not found", command.data.name);
                }
            }
            _ => {}
        }
    }
}
