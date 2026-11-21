use super::icon::{Icon, IconName};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SwitchProps {
    pub checked: bool,
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub label: AttrValue,
}

#[function_component(Switch)]
pub fn switch(props: &SwitchProps) -> Html {
    let onclick = {
        let onchange = props.onchange.clone();
        let checked = props.checked;
        Callback::from(move |_| onchange.emit(!checked))
    };
    let track = if props.checked {
        "bg-accent"
    } else {
        "bg-surface-hover"
    };
    let knob = if props.checked {
        "translate-x-4"
    } else {
        "translate-x-0"
    };
    html! {
        <button
            type="button"
            role="switch"
            aria-checked={props.checked.to_string()}
            aria-label={props.label.clone()}
            disabled={props.disabled}
            class={classes!("relative", "inline-flex", "h-5", "w-9", "shrink-0", "items-center", "rounded-full", "p-0.5", "transition-colors", "duration-150", "disabled:opacity-40", track)}
            {onclick}
        >
            <span class={classes!("size-4", "rounded-full", "bg-white", "shadow-sm", "transition-transform", "duration-150", "ease-out", knob)} />
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct CheckboxProps {
    pub checked: bool,
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub label: AttrValue,
}

#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProps) -> Html {
    let onclick = {
        let onchange = props.onchange.clone();
        let checked = props.checked;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            onchange.emit(!checked)
        })
    };
    let state = if props.checked {
        "bg-accent border-accent text-white"
    } else {
        "bg-transparent border-line-strong"
    };
    html! {
        <button
            type="button"
            role="checkbox"
            aria-checked={props.checked.to_string()}
            aria-label={props.label.clone()}
            disabled={props.disabled}
            class={classes!("inline-flex", "size-4", "shrink-0", "items-center", "justify-center", "rounded-sm", "border", "transition-colors", "duration-150", "disabled:opacity-40", state)}
            {onclick}
        >
            if props.checked {
                <Icon name={IconName::Check} size={12} />
            }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct ToggleRowProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub help: Option<AttrValue>,
    pub checked: bool,
    pub onchange: Callback<bool>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(ToggleRow)]
pub fn toggle_row(props: &ToggleRowProps) -> Html {
    html! {
        <div class="flex items-center gap-4 py-3">
            <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                <span class="text-sm text-fg">{props.label.clone()}</span>
                if let Some(help) = props.help.clone() {
                    <span class="help">{help}</span>
                }
            </div>
            <Switch checked={props.checked} onchange={props.onchange.clone()} disabled={props.disabled} label={props.label.clone()} />
        </div>
    }
}
