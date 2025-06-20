use crate::config::Config;
use crate::db::repr::Thread;
use crate::db::threads::get_all_opened_threads;
use crate::events;
use serenity::all::{ActivityData, GetMessages, UserId};
use serenity::{
    all::{Context, EventHandler, Ready},
    async_trait,
};

#[derive(Clone)]
pub struct ReadyHandler {
    pub config: Config,
}

impl ReadyHandler {
    pub fn new(config: &Config) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

async fn recover_downtime_thread_message(ctx: &Context, thread: Thread) {
    let dm_channel = match ctx.http.get_user(UserId::new(thread.user_id as u64)).await {
        Ok(user) => match user.create_dm_channel(&ctx.http).await {
            Ok(channel) => channel,
            Err(err) => {
                eprintln!("Erreur lors de la création du DM : {}", err);
                return;
            }
        },
        Err(err) => {
            eprintln!("Erreur lors de la récupération des threads : {}", err);
            return;
        }
    };

    //dm_channel.messages(&ctx.http, GetMessages::new().after(message_id)); Pour récupérer tous les messages envoyés après le dernier message reçu par le bot
}

async fn recover_downtime_thread(ctx: &Context, config: &Config) {
    let pool = match &config.db_pool {
        Some(pool) => pool,
        None => {
            eprintln!("Database pool is not set in config.");
            return;
        }
    };
    let opened_threads = get_all_opened_threads(pool).await;

    for thread in opened_threads {}
}

#[async_trait]
impl EventHandler for ReadyHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        events::ready::handle(&ctx, ready).await;
        ctx.set_activity(Option::from(ActivityData::playing(&self.config.bot.status)));
    }
}
