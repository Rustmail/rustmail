use serenity::all::{Attachment, CreateAttachment};

pub enum ReplyIntent {
    Text(String),
    Attachments(Vec<CreateAttachment>),
    TextAndAttachments(String, Vec<CreateAttachment>),
}

pub async fn extract_intent(
    content: Option<String>,
    attachments: &[Attachment],
) -> Option<ReplyIntent> {
    let attachments = download_attachments(attachments).await;

    match (content, attachments.is_empty()) {
        (Some(c), true) => Some(ReplyIntent::Text(c)),
        (None, false) => Some(ReplyIntent::Attachments(attachments)),
        (Some(c), false) => Some(ReplyIntent::TextAndAttachments(c, attachments)),
        (None, true) => None,
    }
}

pub async fn download_attachments(attachments: &[Attachment]) -> Vec<CreateAttachment> {
    let mut downloaded_attachments = Vec::new();

    for attachment in attachments {
        if let Ok(response) = reqwest::get(&attachment.url).await {
            if let Ok(bytes) = response.bytes().await {
                downloaded_attachments
                    .push(CreateAttachment::bytes(bytes, attachment.filename.clone()));
            } else {
                eprintln!(
                    "Failed to read bytes from attachment: {}",
                    attachment.filename
                );
            }
        } else {
            eprintln!("Failed to download attachment: {}", attachment.filename);
        }
    }

    downloaded_attachments
}
