use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Positive,
    Warning,
    Accent,
}

#[derive(Properties, PartialEq)]
pub struct BadgeProps {
    pub children: Html,
    #[prop_or_default]
    pub tone: Tone,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let tone = match props.tone {
        Tone::Neutral => "bg-surface-hover text-fg",
        Tone::Positive => "bg-positive-subtle text-positive",
        Tone::Warning => "bg-warning-subtle text-warning",
        Tone::Accent => "bg-accent-subtle text-accent-fg",
    };
    html! {
        <span class={classes!("inline-flex", "h-5", "items-center", "whitespace-nowrap", "rounded-md", "px-1.5", "text-xs", tone)}>
            {props.children.clone()}
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct ChipProps {
    pub children: Html,
    #[prop_or_default]
    pub on_remove: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub remove_label: AttrValue,
}

#[function_component(Chip)]
pub fn chip(props: &ChipProps) -> Html {
    html! {
        <span class="inline-flex h-6 items-center gap-1 rounded-md border border-line bg-surface-raised pl-2 pr-1 text-xs text-fg">
            {props.children.clone()}
            if let Some(on_remove) = props.on_remove.clone() {
                <button
                    type="button"
                    class="inline-flex size-4 items-center justify-center rounded text-fg-muted transition-colors hover:bg-surface-hover hover:text-fg"
                    aria-label={props.remove_label.clone()}
                    onclick={on_remove}
                >
                    <super::icon::Icon name={super::icon::IconName::X} size={12} />
                </button>
            } else {
                <span class="w-1" />
            }
        </span>
    }
}
