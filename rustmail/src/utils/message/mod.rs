pub mod message_builder;
pub mod reply_intent;

pub mod category;
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

pub use category::*;
pub use message_builder::*;
pub use reply_intent::*;
pub use ui::*;
pub use ui_components::*;
