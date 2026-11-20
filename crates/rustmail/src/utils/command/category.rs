use crate::errors::{CommandError, ModmailError, ModmailResult};
use serenity::all::{ChannelId, Context, PermissionOverwrite};

pub async fn get_category_permissions_overwrites(
    ctx: &Context,
    category_id: ChannelId,
) -> ModmailResult<Vec<PermissionOverwrite>> {
    let category = match category_id.to_channel(&ctx.http).await {
        Ok(channel) => channel.category(),
        Err(..) => {
            return Err(ModmailError::Command(CommandError::NotInThread()));
        }
    };

    let permissions = match category {
        Some(category) => category.permission_overwrites.clone(),
        None => return Err(ModmailError::Command(CommandError::NotInThread())),
    };

    Ok(permissions)
}
