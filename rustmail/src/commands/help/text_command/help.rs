use crate::config::Config;
use crate::errors::ModmailResult;
use crate::types::logs::PaginationStore;
use crate::utils::message::message_builder::MessageBuilder;
use serenity::all::{Context, Message};
use std::sync::Arc;
use tokio::sync::watch::Receiver;

pub async fn help(
    ctx: &Context,
    msg: &Message,
    config: &Config,
    _shutdown: Arc<Receiver<bool>>,
    _pagination: PaginationStore,
) -> ModmailResult<()> {
    let help_message = "# Available commands:\n\n\
        **!add_staff <staff_id>** - Add a staff to an hidden ticket\n\
        **!remove_staff <staff_id>** - Remove a staff from a ticket\n\
        **!alert** - Alert staff in the current thread when a new user answer arrives \n\
        **!help** - Show this help message\n\
        **!reply <message>** - Reply to a ticket\n\
        **!annonreply <message>** - Reply anonymously to a ticket\n\
        **!close [reason]** - Close the current thread with an optional reason\n\
        **!delete [reason]** - Delete the current thread with an optional reason\n\
        **!add_rap** - Add reminder\n\
        **!new_thread <user_id>** - Create a new ticket with a specific user\n\
        **!alert [cancel]** - Set or cancel an alert for staff in the current thread\n\
        **!force_close** - Force close the current thread if it's orphaned\n\
        **!id** - Show the user ID associated with the current thread";

    MessageBuilder::system_message(ctx, config)
        .content(help_message)
        .to_channel(msg.channel_id)
        .send(false)
        .await?;

    Ok(())
}
