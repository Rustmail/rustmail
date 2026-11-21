use super::icon::{Icon, IconName};
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    Primary,
    #[default]
    Secondary,
    Ghost,
    Danger,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Md,
    Sm,
}

pub fn button_classes(variant: ButtonVariant, size: ButtonSize) -> Classes {
    let base = "inline-flex items-center justify-center rounded-lg font-medium text-[13px] leading-5 whitespace-nowrap select-none transition-[background-color,border-color,color,transform,opacity] duration-150 ease-out active:scale-[0.98] disabled:opacity-40 disabled:pointer-events-none";
    let size = match size {
        ButtonSize::Md => "h-9 px-3.5 gap-2",
        ButtonSize::Sm => "h-7 px-2.5 gap-1.5",
    };
    let variant = match variant {
        ButtonVariant::Primary => {
            "bg-surface-inverse text-fg-inverse hover:bg-surface-inverse/90 shadow-[inset_0_1px_0_rgba(255,255,255,0.12)]"
        }
        ButtonVariant::Secondary => {
            "border border-line bg-surface-raised text-fg hover:bg-surface-hover"
        }
        ButtonVariant::Ghost => "text-fg-secondary hover:bg-surface-raised hover:text-fg",
        ButtonVariant::Danger => "bg-negative-subtle text-negative hover:bg-negative/20",
    };
    classes!(base, size, variant)
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub size: ButtonSize,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub trailing_icon: Option<IconName>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub submit: bool,
    #[prop_or_default]
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let class = classes!(
        button_classes(props.variant, props.size),
        props.class.clone()
    );
    let leading = if props.loading {
        html! { <Icon name={IconName::LoaderCircle} class="animate-spin" /> }
    } else {
        props
            .icon
            .map(|name| html! { <Icon {name} />})
            .unwrap_or_default()
    };
    let trailing = props
        .trailing_icon
        .map(|name| html! { <Icon {name} />})
        .unwrap_or_default();
    let content = html! { <>{leading}{props.children.clone()}{trailing}</> };

    if let Some(href) = &props.href {
        return html! { <a {class} href={href.clone()} title={props.title.clone()}>{content}</a> };
    }

    let onclick = props.onclick.clone().unwrap_or_default();
    html! {
        <button
            {class}
            type={if props.submit { "submit" } else { "button" }}
            disabled={props.disabled || props.loading}
            title={props.title.clone()}
            {onclick}
        >
            {content}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    pub icon: IconName,
    pub label: AttrValue,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let onclick = props.onclick.clone().unwrap_or_default();
    html! {
        <button
            type="button"
            class={classes!("inline-flex", "size-8", "items-center", "justify-center", "rounded-lg", "text-fg-muted", "transition-colors", "duration-150", "hover:bg-surface-raised", "hover:text-fg", "disabled:opacity-40", props.class.clone())}
            aria-label={props.label.clone()}
            title={props.label.clone()}
            disabled={props.disabled}
            {onclick}
        >
            <Icon name={props.icon} />
        </button>
    }
}
