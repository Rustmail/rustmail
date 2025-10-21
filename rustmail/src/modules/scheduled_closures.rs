use crate::config::Config;
use crate::db::{
    close_thread, delete_scheduled_closure, get_all_scheduled_closures, get_scheduled_closure,
    get_thread_by_id,
};
use crate::utils::message::message_builder::MessageBuilder;
use chrono::Utc;
use serenity::all::{ChannelId, Context, UserId};
use tokio::time::{Duration, sleep};

fn schedule_one(ctx: &Context, config: &Config, thread_id: String, close_at: i64) {
    let now = Utc::now().timestamp();
    let delay_secs = (close_at - now).max(0) as u64;
    let ctx_clone = ctx.clone();
    let config_clone = config.clone();

    tokio::spawn(async move {
        if delay_secs > 0 {
            sleep(Duration::from_secs(delay_secs)).await;
        }
        if let Some(pool) = config_clone.db_pool.as_ref() {
            if let Ok(Some(current)) = get_scheduled_closure(&thread_id, pool).await {
                if current.close_at <= Utc::now().timestamp() {
                    if let Some(thread) = get_thread_by_id(&thread_id, pool).await {
                        let channel_id =
                            ChannelId::new(thread.channel_id.parse::<u64>().unwrap_or(0));
                        let user_id = UserId::new(thread.user_id as u64);

                        let _ = close_thread(
                            &thread_id,
                            &current.closed_by,
                            &current.category_id,
                            &current.category_name,
                            current.required_permissions.parse::<u64>().unwrap_or(0),
                            pool,
                        )
                        .await;
                        let _ = delete_scheduled_closure(&thread_id, pool).await;
                        if !current.silent {
                            let _ = MessageBuilder::system_message(&ctx_clone, &config_clone)
                                .content(&config_clone.bot.close_message)
                                .to_user(user_id)
                                .send(false)
                                .await;
                        }
                        let _ = channel_id.delete(&ctx_clone.http).await;
                    } else {
                        let _ = delete_scheduled_closure(&thread_id, pool).await;
                    }
                } else {
                    schedule_one(
                        &ctx_clone,
                        &config_clone,
                        thread_id.clone(),
                        current.close_at,
                    );
                }
            }
        }
    });
}

pub async fn hydrate_scheduled_closures(ctx: &Context, config: &Config) {
    let Some(pool) = config.db_pool.as_ref() else {
        return;
    };
    let list = match get_all_scheduled_closures(pool).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to load scheduled closures: {e:?}");
            return;
        }
    };
    for sc in list {
        if let Some(thread) = get_thread_by_id(&sc.thread_id, pool).await {
            if sc.close_at <= Utc::now().timestamp() {
                let channel_id = ChannelId::new(thread.channel_id.parse::<u64>().unwrap_or(0));
                let user_id = UserId::new(thread.user_id as u64);
                let _ = close_thread(
                    &thread.id,
                    &sc.closed_by,
                    &sc.category_id,
                    &sc.category_name,
                    sc.required_permissions.parse::<u64>().unwrap_or(0),
                    pool,
                )
                .await;
                let _ = delete_scheduled_closure(&thread.id, pool).await;
                if !sc.silent {
                    let _ = MessageBuilder::system_message(ctx, config)
                        .content(&config.bot.close_message)
                        .to_user(user_id)
                        .send(false)
                        .await;
                }
                let _ = channel_id.delete(&ctx.http).await;
            } else {
                schedule_one(ctx, config, sc.thread_id.clone(), sc.close_at);
            }
        } else {
            let _ = delete_scheduled_closure(&sc.thread_id, pool).await;
        }
    }
}
