use crate::prelude::commands::*;
use crate::prelude::config::*;
use crate::prelude::features::*;
use crate::prelude::modules::*;
use crate::prelude::types::*;
use serenity::all::{Context, EventHandler, Interaction};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

#[derive(Clone)]
pub struct InteractionHandler {
    pub config: Arc<Config>,
    pub registry: Arc<CommandRegistry>,
    pub shutdown: Arc<Receiver<bool>>,
    pub pagination: PaginationStore,
}

impl InteractionHandler {
    pub fn new(
        config: &Config,
        registry: Arc<CommandRegistry>,
        shutdown: Receiver<bool>,
        pagination: PaginationStore,
    ) -> Self {
        Self {
            config: Arc::new(config.clone()),
            registry,
            shutdown: Arc::new(shutdown),
            pagination,
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
                if let Err(..) = handle_command_component_interaction(
                    &ctx,
                    &self.config,
                    &mut comp,
                    self.pagination.clone(),
                )
                .await
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
                    let result = handler
                        .run(&ctx, &command, &options, &config, Arc::new(self.clone()))
                        .await;

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
