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
    use futures::future::join_all;

    let download_futures = attachments.iter().map(|attachment| {
        let url = attachment.url.clone();
        let filename = attachment.filename.clone();

        async move {
            match reqwest::get(&url).await {
                Ok(response) => match response.bytes().await {
                    Ok(bytes) => Some(CreateAttachment::bytes(bytes, filename.clone())),
                    Err(_) => {
                        eprintln!("Failed to read bytes from attachment: {}", filename);
                        None
                    }
                },
                Err(_) => {
                    eprintln!("Failed to download attachment: {}", filename);
                    None
                }
            }
        }
    });

    join_all(download_futures)
        .await
        .into_iter()
        .flatten()
        .collect()
}
