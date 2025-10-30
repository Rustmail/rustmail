use crate::prelude::errors::*;
use crate::prelude::types::*;
use chrono::Utc;
use serenity::all::{ChannelId, UserId};
use serenity::builder::EditChannel;
use serenity::client::Context;

pub async fn update_thread_status(ctx: &Context, ticket: &TicketState) -> ModmailResult<()> {
    let channel = ChannelId::new(ticket.channel_id as u64);

    let color = match ticket.last_message_by {
        TicketAuthor::Staff => "🔵",
        TicketAuthor::User => "🔴",
    };

    let elapsed = Utc::now().timestamp() - ticket.last_message_at;
    let minutes = elapsed / 60;
    let time_str = if minutes < 60 {
        format!("{}m", minutes)
    } else {
        format!("{}h", minutes / 60)
    };

    let owner_id = ticket.owner_id.parse().unwrap_or(0);
    let owner_name = if owner_id != 0 {
        ctx.cache
            .user(UserId::new(owner_id))
            .map(|u| u.name.clone())
            .unwrap_or_else(|| format!("User-{}", owner_id))
    } else {
        "Unknown".to_string()
    };

    let mut name = format!("{color}・{}", owner_name);

    if let Some(staff_id) = &ticket.taken_by {
        let staff_id_u64 = staff_id.parse().unwrap_or(0);
        if staff_id_u64 != 0 {
            let staff_name = ctx
                .cache
                .user(UserId::new(staff_id_u64))
                .map(|u| u.name.clone())
                .unwrap_or_else(|| format!("Staff-{}", staff_id_u64));
            name.push_str(&format!("・{}", staff_name));
        }
    }

    name.push_str(&format!("・{}", time_str));

    tokio::spawn({
        let ctx = ctx.clone();
        async move {
            let _ = channel
                .edit(&ctx.http, EditChannel::new().name(&name))
                .await;
        }
    });

    Ok(())
}
