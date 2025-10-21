use super::{Feature, make_buttons};
use crate::config::Config;
use crate::utils::message::message_builder::MessageBuilder;
use async_trait::async_trait;
use serenity::all::{
    ButtonStyle, ComponentInteraction, Context, CreateInteractionResponse, CreateMessage,
    InputTextStyle,
};
use serenity::builder::{CreateActionRow, CreateInputText, CreateModal};

#[derive(Default)]
pub struct PollFeature;

#[async_trait]
impl<'a> Feature<'a> for PollFeature {
    fn key(&self) -> &'static str {
        "poll"
    }

    async fn build_message(&self, ctx: &'a Context, config: &'a Config) -> CreateMessage {
        let row = make_buttons(&[
            (
                "Créer un sondage",
                "feature:poll:create",
                ButtonStyle::Success,
                false,
            ),
            ("Test", "feature:poll:delete", ButtonStyle::Danger, false),
        ]);

        MessageBuilder::system_message(ctx, config)
            .content("Sondage (poll):\n\n- Commande: !poll Question ? | Option 1 | Option 2 | ...\n- Bouton ci-dessous: ouvre un formulaire pour créer un sondage dans ce salon.")
            .components(row)
            .build()
            .await
    }

    async fn handle_interaction(
        &self,
        ctx: &Context,
        _config: &Config,
        interaction: &ComponentInteraction,
        action: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if action != "create" {
            return Ok(());
        }

        let modal = CreateModal::new("feature:poll:create", "Créer un sondage").components(vec![
            CreateActionRow::InputText(
                CreateInputText::new(InputTextStyle::Short, "Question", "question")
                    .min_length(3)
                    .max_length(200),
            ),
            CreateActionRow::InputText(
                CreateInputText::new(
                    InputTextStyle::Paragraph,
                    "Options (séparées par | ou retour à la ligne)",
                    "options",
                )
                .min_length(3)
                .max_length(1000),
            ),
        ]);
        interaction
            .create_response(&ctx.http, CreateInteractionResponse::Modal(modal))
            .await?;
        Ok(())
    }
}
