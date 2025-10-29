use crate::prelude::utils::*;
use serenity::all::UserId;

pub fn get_user_recap(
    user_id: UserId,
    username: &str,
    member_join_date: &str,
    logs_info: &str,
) -> String {
    format!(
        "ACCOUNT AGE **{}**, ID **{}**\nNICKNAME **{}**, JOINED **{}** ago\n\n{}",
        format_duration_since(user_id.created_at()),
        user_id,
        username,
        member_join_date,
        logs_info
    )
}
