use serenity::all::CreateMessage;
use crate::utils::format_ticket_message::TicketMessage;

pub fn build_message_from_ticket(
    tmsg: TicketMessage,
    mut msg_builder: CreateMessage,
) -> CreateMessage {
    match tmsg {
        TicketMessage::Plain(txt) => {
            msg_builder = msg_builder.content(txt);
        }
        TicketMessage::Embed(embed) => {
            msg_builder = msg_builder.embed(embed);
        }
    }

    msg_builder
}
