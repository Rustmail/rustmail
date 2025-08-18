use async_trait::async_trait;
use serenity::all::{ComponentInteraction, Context, CreateInteractionResponse, CreateMessage, InputTextStyle, ButtonStyle};
use serenity::builder::{CreateActionRow, CreateButton, CreateInputText, CreateModal};
use crate::config::Config;
use super::Feature;

#[derive(Default)]
pub struct PollFeature;

#[async_trait]
impl Feature for PollFeature {
    fn key(&self) -> &'static str { "poll" }

    fn build_message(&self, _config: &Config) -> CreateMessage {
        let content = "Sondage (poll):\n\n- Commande: !poll Question ? | Option 1 | Option 2 | ...\n- Bouton ci-dessous: ouvre un formulaire pour créer un sondage dans ce salon.";
        let row = CreateActionRow::Buttons(vec![
            CreateButton::new("feature:poll:create").label("Créer un sondage").style(ButtonStyle::Primary),
        ]);
        CreateMessage::new()
            .content(content)
            .components(vec![row])
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        _config: &Config,
        interaction: &ComponentInteraction,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if action != "create" { return Ok(()); }

        let modal = CreateModal::new("feature:poll:create", "Créer un sondage")
            .components(vec![
                CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Short, "Question", "question").min_length(3).max_length(200)),
                CreateActionRow::InputText(CreateInputText::new(InputTextStyle::Paragraph, "Options (séparées par | ou retour à la ligne)", "options").min_length(3).max_length(1000)),
            ]);
        interaction
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await?;
        Ok(())
    }
}
