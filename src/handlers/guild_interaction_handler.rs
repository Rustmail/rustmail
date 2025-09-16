use crate::config::Config;
use crate::features::handle_feature_component_interaction;
use crate::modules::threads::{
    handle_thread_component_interaction, handle_thread_modal_interaction,
};
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, EventHandler, Interaction};
use serenity::async_trait;
use serenity::builder::CreateInteractionResponse;
use crate ::commands;

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
            Interaction::Command(command) => {
                let content: String = match command.data.name.as_str() {
                    "id" => commands::id::run(&command, &command.data.options(), &self.config).await,
                    _ => {
                        println!("Command not implemented: {}", command.data.name);
                        return;
                    },
                };

                let response = CreateInteractionResponse::Message(
                    MessageBuilder::system_message(&ctx, &self.config)
                        .content(content)
                        .to_channel(command.channel_id)
                        .build_interaction_message()
                        .await,
                );
                let _ = command.create_response(&ctx.http, response).await;
            }
            _ => {}
        }
    }
}
