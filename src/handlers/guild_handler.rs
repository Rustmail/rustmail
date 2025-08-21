use async_trait::async_trait;
use serenity::all::{ButtonStyle, Context, GuildChannel};
use serenity::client::EventHandler;
use crate::config::Config;
use crate::features::make_buttons;
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;
use crate::db::operations::get_thread_by_channel_id;

pub struct GuildHandler {
    pub config: Config,
}

impl GuildHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for GuildHandler {
    async fn channel_create(&self, ctx: Context, thread: GuildChannel) {
        if self.config.bot.is_dual_mode() && self.config.bot.is_community_guild(thread.guild_id.get()) {
            return;
        }
        if !self.config.thread.create_ticket_by_create_channel {
            return;
        }

        let expected_category = self.config.thread.inbox_category_id;
        if let Some(parent_id) = thread.parent_id {
            if parent_id.get() != expected_category {
                return;
            }
        } else {
            return;
        }

        if let Some(topic) = &thread.topic {
            if topic == "modmail:managed" {
                return;
            }
        }

        if let Some(pool) = &self.config.db_pool {
            if let Some(_) = get_thread_by_channel_id(&thread.id.to_string(), pool).await {
                return;
            }
        }

        let res_button = make_buttons(&[
            (
                get_translated_message(&self.config, "general.yes", None, None, None, None).await.as_ref(),
                "ticket:wants_to_create",
                ButtonStyle::Success
            ),
            (
                get_translated_message(&self.config, "general.no", None, None, None, None).await.as_ref(),
                "ticket:dont_create",
                ButtonStyle::Danger
            ),
        ]);

        let _ = MessageBuilder::system_message(&ctx, &self.config)
            .translated_content("thread.ask_create_ticket", None, None, None).await
            .components(res_button)
            .to_channel(thread.id)
            .send()
            .await;
    }
}