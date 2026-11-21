use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FieldProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub help: Option<AttrValue>,
    #[prop_or_default]
    pub error: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Field)]
pub fn field(props: &FieldProps) -> Html {
    html! {
        <label class={classes!("flex", "min-w-0", "flex-col", "gap-1.5", props.class.clone())}>
            <span class="label">{props.label.clone()}</span>
            {props.children.clone()}
            if let Some(error) = props.error.clone() {
                <span class="text-xs text-negative">{error}</span>
            } else if let Some(help) = props.help.clone() {
                <span class="help">{help}</span>
            }
        </label>
    }
}

pub fn input_value(e: &InputEvent) -> String {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

pub fn textarea_value(e: &InputEvent) -> String {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlTextAreaElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

pub fn select_value(e: &Event) -> String {
    use wasm_bindgen::JsCast;
    e.target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .map(|i| i.value())
        .unwrap_or_default()
}

#[derive(Properties, PartialEq)]
pub struct SelectProps {
    pub value: AttrValue,
    pub options: Vec<(AttrValue, AttrValue)>,
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Select)]
pub fn select(props: &SelectProps) -> Html {
    let onchange = {
        let cb = props.onchange.clone();
        Callback::from(move |e: Event| cb.emit(select_value(&e)))
    };
    html! {
        <div class={classes!("relative", props.class.clone())}>
            <select class="input appearance-none pr-9" disabled={props.disabled} {onchange}>
                { for props.options.iter().map(|(value, label)| html! {
                    <option value={value.clone()} selected={*value == props.value}>{label.clone()}</option>
                }) }
            </select>
            <span class="pointer-events-none absolute inset-y-0 right-3 flex items-center text-fg-muted">
                <super::icon::Icon name={super::icon::IconName::ChevronDown} />
            </span>
        </div>
    }
}
