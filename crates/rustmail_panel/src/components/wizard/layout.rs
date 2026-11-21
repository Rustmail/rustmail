use crate::components::language_switcher::LanguageSwitcher;
use crate::components::ui::{Button, ButtonSize, ButtonVariant, Icon, IconName, Modal};
use crate::components::wizard::parts::{STEP_KEYS, step_title_key};
use crate::i18n::yew::use_translation;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub current_step: usize,
    pub on_restart: Callback<()>,
    pub children: Children,
}

#[derive(Clone, Copy, PartialEq)]
enum StepState {
    Done,
    Current,
    Upcoming,
}

fn step_marker(state: StepState, number: usize) -> Html {
    let frame = match state {
        StepState::Done => "border-line-strong bg-surface-raised text-fg",
        StepState::Current => "border-accent bg-accent-subtle text-accent-fg",
        StepState::Upcoming => "border-line text-fg-muted",
    };
    html! {
        <span class={classes!("inline-flex", "size-6", "shrink-0", "items-center", "justify-center", "rounded-full", "border", "font-mono", "text-xs", frame)}>
            if state == StepState::Done {
                <Icon name={IconName::Check} size={12} />
            } else {
                {number}
            }
        </span>
    }
}

#[function_component(WizardLayout)]
pub fn wizard_layout(props: &LayoutProps) -> Html {
    let (i18n, _) = use_translation();
    let confirming_restart = use_state(|| false);

    let open_restart = {
        let confirming_restart = confirming_restart.clone();
        Callback::from(move |_: MouseEvent| confirming_restart.set(true))
    };
    let close_restart = {
        let confirming_restart = confirming_restart.clone();
        Callback::from(move |_| confirming_restart.set(false))
    };
    let confirm_restart = {
        let confirming_restart = confirming_restart.clone();
        let on_restart = props.on_restart.clone();
        Callback::from(move |_: MouseEvent| {
            confirming_restart.set(false);
            on_restart.emit(());
        })
    };
    let cancel_restart = {
        let close_restart = close_restart.clone();
        Callback::from(move |_: MouseEvent| close_restart.emit(()))
    };

    let step_key = STEP_KEYS[props.current_step.min(STEP_KEYS.len() - 1)];
    let position = i18n
        .t("wizard.step_of")
        .replace("{current}", &(props.current_step + 1).to_string())
        .replace("{total}", &STEP_KEYS.len().to_string());

    html! {
        <div class="flex min-h-screen bg-surface-page">
            <aside class="sticky top-0 hidden h-screen w-80 shrink-0 flex-col border-r border-line bg-surface-sunken p-6 md:flex">
                <div class="flex items-center gap-2.5">
                    <img src="/logo.png" alt="" class="size-7" />
                    <span class="text-base font-medium text-fg">{i18n.t("wizard.title")}</span>
                </div>
                <ol class="mt-8 flex flex-col gap-1">
                    { for STEP_KEYS.iter().enumerate().map(|(index, key)| {
                        let state = if index < props.current_step {
                            StepState::Done
                        } else if index == props.current_step {
                            StepState::Current
                        } else {
                            StepState::Upcoming
                        };
                        let row = if state == StepState::Current { "bg-surface-raised" } else { "" };
                        let name = if state == StepState::Upcoming { "text-fg-secondary" } else { "text-fg" };
                        html! {
                            <li
                                aria-current={(state == StepState::Current).then_some("step")}
                                class={classes!("flex", "items-start", "gap-3", "rounded-lg", "px-2", "py-2", row)}
                            >
                                {step_marker(state, index + 1)}
                                <div class="flex min-w-0 flex-col">
                                    <span class={classes!("text-sm", "font-medium", name)}>{i18n.t(&step_title_key(key))}</span>
                                    <span class="truncate text-xs text-fg-muted">{i18n.t(&format!("wizard.steps.{key}.caption"))}</span>
                                </div>
                            </li>
                        }
                    }) }
                </ol>
                <div class="mt-auto flex items-center justify-between gap-2">
                    <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::RotateCcw} onclick={open_restart.clone()}>
                        {i18n.t("wizard.restart")}
                    </Button>
                    <LanguageSwitcher />
                </div>
            </aside>

            <main class="min-w-0 flex-1 px-6 pb-16 pt-10 md:px-24 md:pt-[72px]">
                <div class="mx-auto flex w-full max-w-[640px] flex-col gap-8 md:mx-0">
                    <div class="flex items-center justify-between gap-2 md:hidden">
                        <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::RotateCcw} onclick={open_restart}>
                            {i18n.t("wizard.restart")}
                        </Button>
                        <LanguageSwitcher compact=true />
                    </div>
                    <header class="flex flex-col gap-2">
                        <span class="font-mono text-xs text-fg-muted">{position}</span>
                        <h1 class="text-title font-semibold text-fg">{i18n.t(&step_title_key(step_key))}</h1>
                        <p class="text-sm text-fg-secondary">{i18n.t(&format!("wizard.steps.{step_key}.description"))}</p>
                    </header>
                    { for props.children.iter() }
                </div>
            </main>

            if *confirming_restart {
                <Modal
                    title={i18n.t("wizard.restart_title")}
                    description={i18n.t("wizard.restart_confirm")}
                    on_close={close_restart}
                    footer={html! {
                        <>
                            <Button variant={ButtonVariant::Ghost} onclick={cancel_restart}>
                                {i18n.t("wizard.common.cancel")}
                            </Button>
                            <Button variant={ButtonVariant::Danger} icon={IconName::RotateCcw} onclick={confirm_restart}>
                                {i18n.t("wizard.restart")}
                            </Button>
                        </>
                    }}
                />
            }
        </div>
    }
}
