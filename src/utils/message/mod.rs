pub mod message_builder;
pub mod reply_intent;

pub mod ui_components;
pub mod ui {
    use super::{ui_components::ButtonsBuilder, ui_components::ModalBuilder};
    pub fn modal(id: impl Into<String>, title: impl Into<String>) -> ModalBuilder {
        ModalBuilder::new(id, title)
    }
    pub fn buttons() -> ButtonsBuilder {
        ButtonsBuilder::new()
    }
}