use crate::utils::message::message_builder::MessageBuilder;
use crate::utils::thread::fetch_thread::fetch_thread;
use crate::{
    config::Config,
    db::{close_thread, delete_scheduled_closure, get_scheduled_closure, upsert_scheduled_closure},
    errors::{CommandError, ModmailError, ModmailResult, common},
};
use chrono::Utc;
use serenity::all::{Context, GuildId, Message, UserId};
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

fn parse_duration_spec(spec: &str) -> Option<Duration> {
    if spec.is_empty() {
        return None;
    }
    let mut total: u64 = 0;
    let mut num: u64 = 0;
    let mut has_unit_segment = false;
    for ch in spec.chars() {
        if ch.is_ascii_digit() {
            let digit = ch.to_digit(10)? as u64;
            num = num.saturating_mul(10).saturating_add(digit);
        } else {
            let unit_secs = match ch {
                's' | 'S' => 1,
                'm' | 'M' => 60,
                'h' | 'H' => 3600,
                'd' | 'D' => 86400,
                _ => return None,
            };
            total = total.saturating_add(num.saturating_mul(unit_secs));
            num = 0;
            has_unit_segment = true;
        }
    }
    if num > 0 {
        if has_unit_segment {
            total = total.saturating_add(num);
        } else {
            total = total.saturating_add(num * 60);
        }
    }
    if total == 0 {
        None
    } else {
        Some(Duration::from_secs(total))
    }
}

pub async fn close(ctx: &Context, msg: &Message, config: &Config) -> ModmailResult<()> {
    let db_pool = config
        .db_pool
        .as_ref()
        .ok_or_else(common::database_connection_failed)?;

    let content = msg.content.trim();
    let prefix = &config.command.prefix;
    let command_name = "close";
    if !content.starts_with(&format!("{}{}", prefix, command_name)) {
        return Err(ModmailError::Command(CommandError::UnknownCommand(
            command_name.to_string(),
        )));
    }
    let args_part = content[prefix.len() + command_name.len()..].trim();

    let mut silent = false;
    let mut duration: Option<Duration> = None;
    let mut cancel = false;
    if !args_part.is_empty() {
        let tokens: Vec<&str> = args_part.split_whitespace().collect();
        for &tok in &tokens {
            if tok.eq_ignore_ascii_case("-s") || tok.eq_ignore_ascii_case("--silent") {
                silent = true;
                continue;
            }
            if tok.eq_ignore_ascii_case("cancel")
                || tok.eq_ignore_ascii_case("-c")
                || tok.eq_ignore_ascii_case("--cancel")
            {
                cancel = true;
                continue;
            }
            if duration.is_none() {
                if let Some(dur) = parse_duration_spec(tok) {
                    duration = Some(dur);
                    continue;
                } else {
                    return Err(ModmailError::Command(CommandError::InvalidArguments(
                        tok.to_string(),
                    )));
                }
            } else {
                return Err(ModmailError::Command(CommandError::InvalidArguments(
                    tok.to_string(),
                )));
            }
        }
    }

    let thread = fetch_thread(db_pool, &msg.channel_id.to_string()).await?;
    let user_id = UserId::new(thread.user_id as u64);
    let community_guild_id = GuildId::new(config.bot.get_community_guild_id());

    if cancel {
        let existed = delete_scheduled_closure(&thread.id, db_pool)
            .await
            .unwrap_or(false);
        if existed {
            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content("close.closure_canceled", None, Some(msg.author.id), msg.guild_id.map(|g| g.get())).await
                .to_channel(msg.channel_id)
                .send()
                .await;
        } else {
            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content("close.no_scheduled_closures_to_cancel", None, Some(msg.author.id), msg.guild_id.map(|g| g.get())).await
                .to_channel(msg.channel_id)
                .send()
                .await;
        }
        return Ok(());
    }

    if duration.is_none() {
        if let Ok(Some(existing)) = get_scheduled_closure(&thread.id, db_pool).await {
            let remaining = existing.close_at - Utc::now().timestamp();

            let mut params = HashMap::new();
            params.insert("seconds".to_string(), remaining.to_string());

            if remaining > 0 {
                let _ = MessageBuilder::system_message(ctx, config)
                    .translated_content("close.closure_already_scheduled", Some(&params), Some(msg.author.id), msg.guild_id.map(|g| g.get())).await
                    .to_channel(msg.channel_id)
                    .send().await;
                return Ok(());
            }
        }
    }

    if let Some(delay) = duration {
        let delay_secs = delay.as_secs();
        let human = if delay_secs < 60 {
            format!("{}s", delay_secs)
        } else if delay_secs < 3600 {
            format!("{}m", delay_secs / 60)
        } else if delay_secs < 86400 {
            format!("{}h{}m", delay_secs / 3600, (delay_secs % 3600) / 60)
        } else {
            format!("{}d{}h", delay_secs / 86400, (delay_secs % 86400) / 3600)
        };
        let mut params = HashMap::new();
        params.insert("time".to_string(), human);
        
        let _ = if silent {
            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content("close.silent_closing", Some(&params), Some(msg.author.id), msg.guild_id.map(|g| g.get())).await
                .to_channel(msg.channel_id)
                .send()
                .await;
        } else {
            let _ = MessageBuilder::system_message(ctx, config)
                .translated_content("close.closing", Some(&params), Some(msg.author.id), msg.guild_id.map(|g| g.get())).await
                .to_channel(msg.channel_id)
                .send()
                .await;
        };

        let thread_id = thread.id.clone();
        let close_at = Utc::now().timestamp() + delay.as_secs() as i64;
        if let Err(e) = upsert_scheduled_closure(&thread_id, close_at, silent, db_pool).await {
            eprintln!("Failed to persist scheduled closure: {e:?}");
        }
        let channel_id = msg.channel_id;
        let config_clone = config.clone();
        let ctx_clone = ctx.clone();
        let user_id_clone = user_id;
        let thread_id_for_task = thread_id.clone();

        tokio::spawn(async move {
            sleep(delay).await;
            if let Some(pool) = config_clone.db_pool.as_ref() {
                if let Ok(Some(record)) = get_scheduled_closure(&thread_id_for_task, pool).await {
                    if record.close_at <= Utc::now().timestamp() {
                        let _ = close_thread(&thread_id_for_task, pool).await;
                        let _ = delete_scheduled_closure(&thread_id_for_task, pool).await;

                        let community_guild_id =
                            GuildId::new(config_clone.bot.get_community_guild_id());

                        let user_still_member = community_guild_id
                            .member(&ctx_clone.http, user_id_clone)
                            .await
                            .is_ok();

                        if !record.silent && user_still_member {
                            let _ = MessageBuilder::system_message(&ctx_clone, &config_clone)
                                .content(&config_clone.bot.close_message)
                                .to_user(user_id_clone)
                                .send()
                                .await;
                        }
                        let _ = channel_id.delete(&ctx_clone.http).await;
                    } else {
                        let delay2 = (record.close_at - Utc::now().timestamp()).max(1) as u64;
                        let config_clone2 = config_clone.clone();
                        let ctx_clone2 = ctx_clone.clone();
                        let thread_id_again = thread_id_for_task.clone();

                        tokio::spawn(async move {
                            sleep(Duration::from_secs(delay2)).await;
                            if let Some(pool2) = config_clone2.db_pool.as_ref() {
                                if let Ok(Some(r2)) =
                                    get_scheduled_closure(&thread_id_again, pool2).await
                                {
                                    if r2.close_at <= Utc::now().timestamp() {
                                        let _ = close_thread(&thread_id_again, pool2).await;
                                        let _ =
                                            delete_scheduled_closure(&thread_id_again, pool2).await;
                                        let community_guild_id = GuildId::new(
                                            config_clone2.bot.get_community_guild_id(),
                                        );
                                        let user_still_member = community_guild_id
                                            .member(&ctx_clone2.http, user_id_clone)
                                            .await
                                            .is_ok();
                                        if !r2.silent && user_still_member {
                                            let _ = MessageBuilder::system_message(
                                                &ctx_clone2,
                                                &config_clone2,
                                            )
                                            .content(&config_clone2.bot.close_message)
                                            .to_user(user_id_clone)
                                            .send()
                                            .await;
                                        }
                                        let _ = channel_id.delete(&ctx_clone2.http).await;
                                    }
                                }
                            }
                        });
                    }
                }
            }
        });
        return Ok(());
    }

    let user_still_member = community_guild_id.member(&ctx.http, user_id).await.is_ok();

    if user_still_member && !silent {
        let _ = MessageBuilder::system_message(ctx, config)
            .content(&config.bot.close_message)
            .to_user(user_id)
            .send()
            .await;
    } else if !user_still_member {
        let mut params = HashMap::new();
        params.insert("username".to_string(), thread.user_name.clone());

        let _ = MessageBuilder::system_message(ctx, config)
            .translated_content(
                "user.left_server_close",
                Some(&params),
                Some(msg.author.id),
                msg.guild_id.map(|g| g.get()),
            )
            .await
            .to_channel(msg.channel_id)
            .send()
            .await;
    }

    close_thread(&thread.id, db_pool).await?;
    let _ = delete_scheduled_closure(&thread.id, db_pool).await;

    let _ = msg.channel_id.delete(&ctx.http).await?;

    Ok(())
}
