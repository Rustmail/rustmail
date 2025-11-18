use serenity::all::{ButtonStyle as SButtonStyle, InputTextStyle};
use serenity::builder::{CreateActionRow, CreateButton, CreateInputText, CreateModal};

pub struct ModalBuilder {
    id: String,
    title: String,
    inputs: Vec<CreateActionRow>,
}

impl ModalBuilder {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            inputs: Vec::new(),
        }
    }

    pub fn short_input(
        mut self,
        custom_id: impl Into<String>,
        label: impl Into<String>,
        placeholder: Option<&str>,
        required: bool,
    ) -> Self {
        let mut it = CreateInputText::new(InputTextStyle::Short, label, custom_id);
        if let Some(ph) = placeholder {
            it = it.placeholder(ph);
        }
        it = it.required(required);
        self.inputs.push(CreateActionRow::InputText(it));
        self
    }

    pub fn _paragraph_input(
        mut self,
        custom_id: impl Into<String>,
        label: impl Into<String>,
        placeholder: Option<&str>,
        required: bool,
    ) -> Self {
        let mut it = CreateInputText::new(InputTextStyle::Paragraph, custom_id, label);
        if let Some(ph) = placeholder {
            it = it.placeholder(ph);
        }
        it = it.required(required);
        self.inputs.push(CreateActionRow::InputText(it));
        self
    }

    pub fn build(self) -> CreateModal {
        CreateModal::new(self.id, self.title).components(self.inputs)
    }
}

pub struct _ButtonsBuilder {
    rows: Vec<CreateActionRow>,
    current: Vec<CreateButton>,
}

impl _ButtonsBuilder {
    pub fn _new() -> Self {
        Self {
            rows: Vec::new(),
            current: Vec::new(),
        }
    }

    pub fn _primary(mut self, custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        self.current.push(
            CreateButton::new(custom_id)
                .label(label)
                .style(SButtonStyle::Primary),
        );
        self
    }

    pub fn _secondary(mut self, custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        self.current.push(
            CreateButton::new(custom_id)
                .label(label)
                .style(SButtonStyle::Secondary),
        );
        self
    }

    pub fn _success(mut self, custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        self.current.push(
            CreateButton::new(custom_id)
                .label(label)
                .style(SButtonStyle::Success),
        );
        self
    }

    pub fn _danger(mut self, custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        self.current.push(
            CreateButton::new(custom_id)
                .label(label)
                .style(SButtonStyle::Danger),
        );
        self
    }

    pub fn _link(mut self, url: impl Into<String>, label: impl Into<String>) -> Self {
        self.current.push(CreateButton::new_link(url).label(label));
        self
    }

    pub fn _row(mut self) -> Self {
        if !self.current.is_empty() {
            let buttons = std::mem::take(&mut self.current);
            self.rows.push(CreateActionRow::Buttons(buttons));
        }
        self
    }

    pub fn _build(mut self) -> Vec<CreateActionRow> {
        if !self.current.is_empty() {
            self = self._row();
        }
        self.rows
    }
}
