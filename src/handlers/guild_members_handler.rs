use serenity::{
    all::{Context, EventHandler, GuildId, User},
    async_trait,
};
use crate::config::Config;
use crate::db::operations::update_thread_user_left;
use std::collections::HashMap;
use serenity::all::{ButtonStyle, Member};
use crate::db::close_thread;
use crate::db::threads::get_thread_by_user_id;
use crate::features::make_buttons;
use crate::i18n::get_translated_message;
use crate::utils::message::message_builder::MessageBuilder;

pub struct GuildMembersHandler {
    pub config: Config,
}

impl GuildMembersHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for GuildMembersHandler {
    async fn guild_member_removal(&self, ctx: Context, guild_id: GuildId, user: User, _member: Option<Member>) {
        
        if !self.config.bot.is_community_guild(guild_id.get()) {
            return;
        }

        let pool = match &self.config.db_pool {
            Some(pool) => pool,
            None => {
                eprintln!("Database pool is not set in config.");
                return;
            }
        };

        let (thread, channel_id) = match get_thread_by_user_id(user.id, pool).await {
            Some(thread) => match thread.channel_id.parse::<u64>() {
                Ok(channel_id_num) => {
                    (thread, serenity::all::ChannelId::new(channel_id_num))
                },
                Err(err) => {
                    eprintln!("Invalid channel ID format for user {} : {}", user.id, err);
                    return ();
                }
            },
            None => return (),
        };

        let mut params = HashMap::new();
        params.insert("username".to_string(), user.name.clone());
        params.insert("user_id".to_string(), user.id.to_string());

        let close_buttons = make_buttons(&[
            (
                get_translated_message(&self.config, "thread.ask_to_keep_open", None, None, None, None).await.as_ref(),
                "ticket:keep",
                ButtonStyle::Success
            ),
            (
                get_translated_message(&self.config, "thread.ask_to_close", None, None, None, None).await.as_ref(),
                "ticket:delete",
                ButtonStyle::Danger
            ),
        ]);

        let _ = MessageBuilder::system_message(&ctx, &self.config)
            .translated_content(
                "user.left_server_notification",
                Some(&params),
                Some(user.id),
                Some(guild_id.get())
            ).await
            .to_channel(channel_id)
            .components(close_buttons)
            .send()
            .await;

        if let Err(e) = update_thread_user_left(&thread.channel_id, pool).await {
            eprintln!("Erreur lors de la mise à jour du statut du thread: {:?}", e);
        }

        let _ = close_thread(&thread.id, pool).await;
    }
} 