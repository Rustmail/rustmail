use super::sections::{
    BotSettings, CommandSettings, ErrorHandlingSettings, LanguageSettings, LogsReminderSettings,
    NotificationSettings, ServerModeSettings, SettingsSection, TicketSettings, UpdateSettings,
};
use crate::components::ui::{Button, ButtonVariant, Notice, StatusDot};
use crate::i18n::yew::use_translation;
use rustmail_types::ConfigResponse;
use serde_json::Value;
use yew::prelude::*;

fn count_changes(original: &ConfigResponse, edited: &ConfigResponse) -> usize {
    let (Ok(Value::Object(before)), Ok(Value::Object(after))) =
        (serde_json::to_value(original), serde_json::to_value(edited))
    else {
        return usize::from(original != edited);
    };
    after
        .iter()
        .map(|(group, fields)| match (before.get(group), fields) {
            (Some(Value::Object(old)), Value::Object(new)) => new
                .iter()
                .filter(|(key, value)| old.get(key.as_str()) != Some(value))
                .count(),
            (old, new) => usize::from(old != Some(new)),
        })
        .sum()
}

fn scroll_to_section(section: SettingsSection) {
    if let Some(element) = gloo_utils::document().get_element_by_id(section.anchor()) {
        element.scroll_into_view_with_bool(true);
    }
}

#[derive(Properties, PartialEq)]
pub struct ConfigFormProps {
    pub config: ConfigResponse,
    pub saving: bool,
    pub save_error: Option<AttrValue>,
    pub on_save: Callback<ConfigResponse>,
}

#[function_component(ConfigForm)]
pub fn config_form(props: &ConfigFormProps) -> Html {
    let (i18n, _) = use_translation();
    let config = use_state(|| props.config.clone());
    let active = use_state(|| SettingsSection::Bot);

    let changes = count_changes(&props.config, &config);
    let dirty = *config != props.config;

    let nav = SettingsSection::ALL.iter().map(|&section| {
        let is_active = *active == section;
        let onclick = {
            let active = active.clone();
            Callback::from(move |e: MouseEvent| {
                e.prevent_default();
                active.set(section);
                scroll_to_section(section);
            })
        };
        let state = if is_active {
            "border-accent bg-surface-raised text-fg"
        } else {
            "border-transparent text-fg-secondary hover:bg-surface-raised hover:text-fg"
        };
        html! {
            <a
                href={format!("#{}", section.anchor())}
                aria-current={is_active.then_some("true")}
                class={classes!("flex", "h-8", "items-center", "rounded-md", "border-l-2", "px-3", "text-[13px]", "transition-colors", "duration-150", state)}
                {onclick}
            >
                {i18n.t(section.title_key())}
            </a>
        }
    });

    let reset = {
        let config = config.clone();
        let original = props.config.clone();
        Callback::from(move |_: MouseEvent| config.set(original.clone()))
    };

    let save = {
        let config = config.clone();
        let on_save = props.on_save.clone();
        Callback::from(move |_: MouseEvent| on_save.emit((*config).clone()))
    };

    let summary = match changes {
        0 => i18n.t("panel.configuration.unsaved_changes"),
        1 => i18n.t("panel.configuration.unsaved_changes_one"),
        n => i18n
            .t("panel.configuration.unsaved_changes_count")
            .replace("{count}", &n.to_string()),
    };

    html! {
        <div class="flex items-start gap-8 pb-24">
            <nav class="sticky top-7 hidden w-[200px] shrink-0 flex-col gap-0.5 lg:flex">
                <span class="px-3 pb-2 text-xs text-fg-muted">{i18n.t("panel.configuration.settings_nav")}</span>
                { for nav }
            </nav>
            <div class="flex min-w-0 flex-1 flex-col gap-5">
                <BotSettings config={config.clone()} />
                <ServerModeSettings config={config.clone()} />
                <CommandSettings config={config.clone()} />
                <TicketSettings config={config.clone()} />
                <LanguageSettings config={config.clone()} />
                <NotificationSettings config={config.clone()} />
                <ErrorHandlingSettings config={config.clone()} />
                <LogsReminderSettings config={config.clone()} />
                <UpdateSettings config={config.clone()} />
            </div>
            if dirty || props.saving {
                <div class="fixed bottom-6 left-1/2 z-40 flex w-max max-w-[calc(100vw-2rem)] -translate-x-1/2 flex-col items-stretch gap-2 lg:left-[calc(50%+124px)]">
                    if let Some(error) = props.save_error.clone() {
                        <div class="rounded-lg bg-surface-raised">
                            <Notice>{error}</Notice>
                        </div>
                    }
                    <div class="flex items-center gap-3 rounded-xl border border-line-strong bg-surface-raised py-2 pl-4 pr-2 shadow-[0_8px_24px_rgba(0,0,0,0.4)]">
                        <StatusDot class="bg-accent" />
                        <span class="whitespace-nowrap text-[13px] text-fg">{summary}</span>
                        <div class="ml-4 flex items-center gap-1">
                            <Button variant={ButtonVariant::Ghost} disabled={props.saving} onclick={reset}>
                                {i18n.t("panel.configuration.reset")}
                            </Button>
                            <Button variant={ButtonVariant::Primary} loading={props.saving} onclick={save}>
                                {i18n.t("panel.configuration.save_button")}
                            </Button>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}
