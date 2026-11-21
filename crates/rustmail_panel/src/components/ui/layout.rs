use super::icon::{Icon, IconName};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <header class="flex flex-wrap items-end gap-4">
            <div class="flex min-w-0 flex-1 flex-col gap-1">
                <h1 class="text-title font-semibold text-fg">{props.title.clone()}</h1>
                if let Some(description) = props.description.clone() {
                    <p class="text-sm text-fg-secondary">{description}</p>
                }
            </div>
            <div class="flex items-center gap-2">{props.children.clone()}</div>
        </header>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardHeaderProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(CardHeader)]
pub fn card_header(props: &CardHeaderProps) -> Html {
    html! {
        <div class="flex items-start gap-4">
            <div class="flex min-w-0 flex-1 flex-col gap-1">
                <h2 class="text-base font-semibold text-fg">{props.title.clone()}</h2>
                if let Some(description) = props.description.clone() {
                    <p class="text-[13px] text-fg-secondary">{description}</p>
                }
            </div>
            {props.children.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    pub label: AttrValue,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    html! {
        <div class={classes!("flex", "items-center", "justify-center", "gap-2", "py-16", "text-[13px]", "text-fg-muted", props.class.clone())}>
            <Icon name={IconName::LoaderCircle} class="animate-spin" />
            {props.label.clone()}
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct EmptyStateProps {
    pub icon: IconName,
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProps) -> Html {
    html! {
        <div class="flex flex-col items-center gap-3 px-8 py-12 text-center">
            <span class="inline-flex size-10 items-center justify-center rounded-xl bg-surface-raised text-fg-secondary">
                <Icon name={props.icon} />
            </span>
            <p class="text-base font-medium text-fg">{props.title.clone()}</p>
            if let Some(description) = props.description.clone() {
                <p class="max-w-xs text-[13px] text-fg-secondary">{description}</p>
            }
            {props.children.clone()}
        </div>
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum NoticeTone {
    #[default]
    Negative,
    Positive,
    Warning,
    Neutral,
}

#[derive(Properties, PartialEq)]
pub struct NoticeProps {
    pub children: Html,
    #[prop_or_default]
    pub tone: NoticeTone,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(Notice)]
pub fn notice(props: &NoticeProps) -> Html {
    let (tone, icon) = match props.tone {
        NoticeTone::Negative => ("bg-negative-subtle text-negative", IconName::CircleAlert),
        NoticeTone::Positive => ("bg-positive-subtle text-positive", IconName::CircleCheck),
        NoticeTone::Warning => ("bg-warning-subtle text-warning", IconName::TriangleAlert),
        NoticeTone::Neutral => (
            "border border-line bg-surface text-fg-secondary",
            IconName::Info,
        ),
    };
    html! {
        <div class={classes!("flex", "items-start", "gap-2.5", "rounded-lg", "px-3", "py-2.5", "text-[13px]", tone, props.class.clone())}>
            <Icon name={icon} class="mt-0.5" />
            <div class="min-w-0 flex-1 text-fg">{props.children.clone()}</div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub footer: Html,
    #[prop_or(480)]
    pub width: u32,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let close = {
        let cb = props.on_close.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };
    let stop = Callback::from(|e: MouseEvent| e.stop_propagation());
    html! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 p-4" onclick={close.clone()}>
            <div
                role="dialog"
                aria-modal="true"
                style={format!("max-width:{}px", props.width)}
                class="flex max-h-[calc(100vh-2rem)] w-full flex-col gap-5 overflow-y-auto rounded-2xl border border-line-strong bg-surface p-6 pb-5 shadow-[0_16px_48px_rgba(0,0,0,0.5)]"
                onclick={stop}
            >
                <div class="flex items-start gap-3">
                    <div class="flex min-w-0 flex-1 flex-col gap-1">
                        <h2 class="text-base font-semibold text-fg">{props.title.clone()}</h2>
                        if let Some(description) = props.description.clone() {
                            <p class="text-[13px] text-fg-secondary">{description}</p>
                        }
                    </div>
                    <super::button::IconButton icon={IconName::X} label="Close" onclick={close} />
                </div>
                {props.children.clone()}
                <div class="flex items-center justify-end gap-2 border-t border-line pt-4">
                    {props.footer.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct SegmentedProps {
    pub options: Vec<(AttrValue, AttrValue)>,
    pub value: AttrValue,
    pub onchange: Callback<AttrValue>,
}

#[function_component(Segmented)]
pub fn segmented(props: &SegmentedProps) -> Html {
    html! {
        <div class="inline-flex gap-0.5 rounded-lg border border-line bg-surface-raised p-0.5">
            { for props.options.iter().map(|(value, label)| {
                let active = *value == props.value;
                let onclick = {
                    let cb = props.onchange.clone();
                    let value = value.clone();
                    Callback::from(move |_| cb.emit(value.clone()))
                };
                let state = if active {
                    "bg-surface-hover text-fg"
                } else {
                    "text-fg-secondary hover:text-fg"
                };
                html! {
                    <button type="button" class={classes!("h-7", "rounded-md", "px-3", "text-[13px]", "font-medium", "transition-colors", "duration-150", state)} {onclick}>
                        {label.clone()}
                    </button>
                }
            }) }
        </div>
    }
}
