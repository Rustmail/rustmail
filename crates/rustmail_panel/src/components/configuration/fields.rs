use crate::components::ui::{Field, input_value, textarea_value};
use rustmail_types::ConfigResponse;
use yew::prelude::*;

pub fn edit<T: 'static>(
    config: &UseStateHandle<ConfigResponse>,
    apply: impl Fn(&mut ConfigResponse, T) + 'static,
) -> Callback<T> {
    let config = config.clone();
    Callback::from(move |value: T| {
        let mut next = (*config).clone();
        apply(&mut next, value);
        if next != *config {
            config.set(next);
        }
    })
}

pub fn parse_id_list(raw: &str) -> Vec<u64> {
    raw.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

pub fn join_ids(ids: &[u64]) -> String {
    ids.iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[derive(Properties, PartialEq)]
pub struct SettingsCardProps {
    pub id: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(SettingsCard)]
pub fn settings_card(props: &SettingsCardProps) -> Html {
    html! {
        <section id={props.id.clone()} class="card flex scroll-mt-7 flex-col gap-5 p-6">
            <header class="flex flex-col gap-1">
                <h2 class="text-section font-semibold text-fg">{props.title.clone()}</h2>
                <p class="text-[13px] text-fg-secondary">{props.description.clone()}</p>
            </header>
            {props.children.clone()}
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct FieldGridProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub columns: Option<u8>,
}

#[function_component(FieldGrid)]
pub fn field_grid(props: &FieldGridProps) -> Html {
    let columns = match props.columns {
        Some(3) => "md:grid-cols-3",
        _ => "md:grid-cols-2",
    };
    html! {
        <div class={classes!("grid", "grid-cols-1", "gap-x-4", "gap-y-5", columns)}>
            {props.children.clone()}
        </div>
    }
}

#[function_component(Divider)]
pub fn divider() -> Html {
    html! { <div class="h-px bg-line" /> }
}

#[derive(Properties, PartialEq)]
pub struct ToggleGridProps {
    pub children: Children,
}

#[function_component(ToggleGrid)]
pub fn toggle_grid(props: &ToggleGridProps) -> Html {
    let rows = props.children.len().div_ceil(2).max(1);
    html! {
        <div
            style={format!("--toggle-rows:{rows}")}
            class="grid grid-cols-1 gap-x-10 md:grid-flow-col md:grid-cols-2 md:[grid-template-rows:repeat(var(--toggle-rows),auto)]"
        >
            { for props.children.iter() }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextFieldProps {
    pub label: AttrValue,
    pub value: AttrValue,
    pub on_change: Callback<String>,
    #[prop_or_default]
    pub help: Option<AttrValue>,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub mono: bool,
    #[prop_or_default]
    pub numeric: bool,
    #[prop_or_default]
    pub suffix: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| on_change.emit(input_value(&e)))
    };
    let input_class = classes!(
        "input",
        props.mono.then_some("input-mono"),
        props.suffix.as_ref().map(|_| "pr-20")
    );
    html! {
        <Field label={props.label.clone()} help={props.help.clone()} class={props.class.clone()}>
            <div class="relative">
                <input
                    type={if props.numeric { "number" } else { "text" }}
                    class={input_class}
                    value={props.value.clone()}
                    placeholder={props.placeholder.clone()}
                    {oninput}
                />
                if let Some(suffix) = props.suffix.clone() {
                    <span class="pointer-events-none absolute inset-y-0 right-3 flex items-center text-[13px] text-fg-muted">
                        {suffix}
                    </span>
                }
            </div>
        </Field>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextAreaFieldProps {
    pub label: AttrValue,
    pub value: AttrValue,
    pub on_change: Callback<String>,
}

#[function_component(TextAreaField)]
pub fn text_area_field(props: &TextAreaFieldProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| on_change.emit(textarea_value(&e)))
    };
    html! {
        <Field label={props.label.clone()}>
            <textarea class="textarea min-h-[72px] resize-y" rows="3" value={props.value.clone()} {oninput} />
        </Field>
    }
}

#[derive(Properties, PartialEq)]
pub struct ColorFieldProps {
    pub label: AttrValue,
    pub value: AttrValue,
    pub on_change: Callback<String>,
}

#[function_component(ColorField)]
pub fn color_field(props: &ColorFieldProps) -> Html {
    let oninput = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            on_change.emit(input_value(&e).trim_start_matches('#').to_string())
        })
    };
    html! {
        <Field label={props.label.clone()}>
            <div class="relative">
                <span
                    class="pointer-events-none absolute left-3 top-1/2 size-4 -translate-y-1/2 rounded border border-line-strong"
                    style={format!("background-color:#{}", props.value)}
                />
                <span class="pointer-events-none absolute inset-y-0 left-9 flex items-center font-mono text-[13px] text-fg-muted">{"#"}</span>
                <input
                    type="text"
                    class="input input-mono pl-12 uppercase"
                    maxlength="6"
                    placeholder="3D54FF"
                    value={props.value.clone()}
                    {oninput}
                />
            </div>
        </Field>
    }
}
