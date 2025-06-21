use serenity::{
    all::{Context, EventHandler, GuildId, User},
    async_trait,
};
use crate::config::Config;
use crate::db::operations::{get_thread_channel_by_user_id, update_thread_user_left};
use crate::utils::format_ticket_message::{Sender, format_ticket_message};
use crate::utils::build_message_from_ticket::build_message_from_ticket;
use serenity::all::CreateMessage;
use crate::i18n::get_translated_message;
use std::collections::HashMap;

pub struct MemberHandler {
    pub config: Config,
}

impl MemberHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait]
impl EventHandler for MemberHandler {
    async fn guild_member_removal(&self, ctx: Context, _guild_id: GuildId, user: User, _member: Option<serenity::all::Member>) {
        if let Some(pool) = &self.config.db_pool {
            if let Some(channel_id_str) = get_thread_channel_by_user_id(user.id, pool).await {
                if let Ok(channel_id_num) = channel_id_str.parse::<u64>() {
                    let channel_id = serenity::all::ChannelId::new(channel_id_num);
                    
                    let user_id = ctx.cache.current_user().id;
                    let username = ctx.cache.current_user().name.clone();
                    
                    let mut params = HashMap::new();
                    params.insert("username".to_string(), user.name.clone());
                    params.insert("user_id".to_string(), user.id.to_string());
                    
                    let notification_message = get_translated_message(
                        &self.config,
                        "user.left_server_notification",
                        Some(&params),
                        Some(user.id),
                        Some(_guild_id.get()),
                        None
                    ).await;

                    let response = format_ticket_message(
                        &ctx,
                        Sender::System {
                            user_id,
                            username,
                        },
                        &notification_message,
                        &self.config,
                    ).await;

                    let thread_message = build_message_from_ticket(response, CreateMessage::new());
                    
                    if let Err(e) = channel_id.send_message(&ctx.http, thread_message).await {
                        eprintln!("Erreur lors de l'envoi de la notification de départ: {:?}", e);
                    }

                    if let Err(e) = update_thread_user_left(&channel_id_str, pool).await {
                        eprintln!("Erreur lors de la mise à jour du statut du thread: {:?}", e);
                    }

                    println!("Utilisateur {} a quitté le serveur, notification envoyée dans le thread {}", 
                             user.name, channel_id_str);
                }
            }
        }
    }
} 