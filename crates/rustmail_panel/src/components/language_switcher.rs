use crate::components::ui::{Icon, IconName, select_value};
use crate::i18n::yew::use_translation;
use yew::prelude::*;

const LANGUAGES: [(&str, &str); 2] = [("en", "English"), ("fr", "Français")];

#[derive(Properties, PartialEq, Clone)]
pub struct LanguageSwitcherProps {
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(LanguageSwitcher)]
pub fn language_switcher(props: &LanguageSwitcherProps) -> Html {
    let (i18n, set_language) = use_translation();
    let current = i18n.get_current_language().to_string();
    let label = if props.compact {
        current.to_uppercase()
    } else {
        LANGUAGES
            .iter()
            .find(|(code, _)| *code == current)
            .map(|(_, name)| name.to_string())
            .unwrap_or_else(|| current.to_uppercase())
    };

    let onchange = Callback::from(move |e: Event| set_language.emit(select_value(&e)));

    let frame = if props.compact {
        "h-7 gap-1 px-1.5 hover:bg-surface-raised"
    } else {
        "h-8 gap-1.5 border border-line pl-2.5 pr-2 hover:bg-surface-raised"
    };

    html! {
        <label class={classes!("relative", "inline-flex", "cursor-pointer", "items-center", "rounded-md", "text-fg-secondary", "transition-colors", "duration-150", frame, props.class.clone())}>
            <Icon name={IconName::Languages} />
            <span class={if props.compact { "font-mono text-xs" } else { "text-[13px] font-medium" }}>{label}</span>
            if !props.compact {
                <Icon name={IconName::ChevronDown} class="text-fg-muted" />
            }
            <select
                aria-label={i18n.t("navbar.language")}
                class="absolute inset-0 cursor-pointer opacity-0"
                {onchange}
            >
                { for LANGUAGES.iter().map(|(code, name)| html! {
                    <option value={*code} selected={*code == current}>{*name}</option>
                }) }
            </select>
        </label>
    }
}
