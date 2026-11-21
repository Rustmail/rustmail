use yew::prelude::*;

pub fn initials(name: &str) -> String {
    let letters: String = name
        .chars()
        .filter(|c| c.is_alphanumeric())
        .take(2)
        .collect();
    letters.to_uppercase()
}

#[derive(Properties, PartialEq)]
pub struct AvatarProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub url: Option<AttrValue>,
    #[prop_or(32)]
    pub size: u32,
    #[prop_or_default]
    pub member: bool,
}

#[function_component(Avatar)]
pub fn avatar(props: &AvatarProps) -> Html {
    let style = format!(
        "width:{0}px;height:{0}px;font-size:{1}px",
        props.size,
        (props.size as f32 * 0.36).round()
    );
    if let Some(url) = props.url.clone().filter(|u| !u.is_empty()) {
        return html! {
            <img src={url} alt="" {style} class="shrink-0 rounded-full bg-surface-hover object-cover" />
        };
    }
    let letters = initials(&props.name);
    if letters.is_empty() {
        return html! {
            <span {style} class="inline-flex shrink-0 items-center justify-center rounded-full bg-surface-hover text-fg-secondary">
                <super::icon::Icon name={super::icon::IconName::User} />
            </span>
        };
    }
    let tone = if props.member {
        "bg-blurple text-white"
    } else {
        "bg-surface-hover text-fg-secondary"
    };
    html! {
        <span {style} class={classes!("inline-flex", "shrink-0", "items-center", "justify-center", "rounded-full", "font-medium", tone)}>
            {letters}
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct TicketStubProps {
    pub number: AttrValue,
}

const STUB_MASK: &str = "-webkit-mask:radial-gradient(circle 4px at 0 50%,#0000 98%,#000) left/51% 100% no-repeat,radial-gradient(circle 4px at 100% 50%,#0000 98%,#000) right/51% 100% no-repeat;mask:radial-gradient(circle 4px at 0 50%,#0000 98%,#000) left/51% 100% no-repeat,radial-gradient(circle 4px at 100% 50%,#0000 98%,#000) right/51% 100% no-repeat";

#[function_component(TicketStub)]
pub fn ticket_stub(props: &TicketStubProps) -> Html {
    html! {
        <span
            style={STUB_MASK}
            class="inline-flex h-5 min-w-[60px] items-center justify-center rounded border border-line-strong bg-surface-raised px-2.5 font-mono text-xs text-fg-secondary"
        >
            {format!("#{}", props.number)}
        </span>
    }
}

#[derive(Properties, PartialEq)]
pub struct StatusDotProps {
    pub class: Classes,
}

#[function_component(StatusDot)]
pub fn status_dot(props: &StatusDotProps) -> Html {
    html! { <span class={classes!("inline-block", "size-2", "shrink-0", "rounded-full", props.class.clone())} /> }
}
