use super::fields::{
    ColorField, Divider, FieldGrid, SettingsCard, TextAreaField, TextField, ToggleGrid, edit,
    join_ids, parse_id_list,
};
use crate::components::ui::ToggleRow;
use crate::i18n::yew::use_translation;
use rustmail_types::*;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SettingsSection {
    Bot,
    ServerMode,
    Commands,
    Tickets,
    Languages,
    Notifications,
    ErrorHandling,
    LogsReminders,
    Updates,
}

impl SettingsSection {
    pub const ALL: [SettingsSection; 9] = [
        SettingsSection::Bot,
        SettingsSection::ServerMode,
        SettingsSection::Commands,
        SettingsSection::Tickets,
        SettingsSection::Languages,
        SettingsSection::Notifications,
        SettingsSection::ErrorHandling,
        SettingsSection::LogsReminders,
        SettingsSection::Updates,
    ];

    pub fn anchor(self) -> &'static str {
        match self {
            SettingsSection::Bot => "settings-bot",
            SettingsSection::ServerMode => "settings-server-mode",
            SettingsSection::Commands => "settings-commands",
            SettingsSection::Tickets => "settings-tickets",
            SettingsSection::Languages => "settings-languages",
            SettingsSection::Notifications => "settings-notifications",
            SettingsSection::ErrorHandling => "settings-error-handling",
            SettingsSection::LogsReminders => "settings-logs-reminders",
            SettingsSection::Updates => "settings-updates",
        }
    }

    pub fn title_key(self) -> &'static str {
        match self {
            SettingsSection::Bot => "panel.configuration.sections.bot",
            SettingsSection::ServerMode => "panel.configuration.sections.server_mode",
            SettingsSection::Commands => "panel.configuration.sections.commands",
            SettingsSection::Tickets => "panel.configuration.sections.threads",
            SettingsSection::Languages => "panel.configuration.sections.languages",
            SettingsSection::Notifications => "panel.configuration.sections.notifications",
            SettingsSection::ErrorHandling => "panel.configuration.sections.error_handling",
            SettingsSection::LogsReminders => "panel.configuration.sections.logs_reminders",
            SettingsSection::Updates => "panel.configuration.sections.updates",
        }
    }

    fn description_key(self) -> String {
        format!("{}_description", self.title_key())
    }
}

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub config: UseStateHandle<ConfigResponse>,
}

#[derive(Properties, PartialEq)]
struct SectionCardProps {
    section: SettingsSection,
    children: Html,
}

#[function_component(SectionCard)]
fn section_card(props: &SectionCardProps) -> Html {
    let (i18n, _) = use_translation();
    html! {
        <SettingsCard
            id={props.section.anchor()}
            title={i18n.t(props.section.title_key())}
            description={i18n.t(&props.section.description_key())}
        >
            {props.children.clone()}
        </SettingsCard>
    }
}

#[function_component(BotSettings)]
pub fn bot_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let bot = &config.bot;
    let t = |key: &str| i18n.t(&format!("panel.configuration.bot.{key}"));
    let optional = i18n.t("panel.configuration.optional");
    let not_set = i18n.t("panel.configuration.not_set");

    html! {
        <SectionCard section={SettingsSection::Bot}>
            <FieldGrid>
                <TextField
                    label={t("token")}
                    help={t("token_help")}
                    mono=true
                    value={bot.token.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.token = value)}
                />
                <TextField
                    label={t("status")}
                    value={bot.status.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.status = value)}
                />
                <TextAreaField
                    label={t("welcome_message")}
                    value={bot.welcome_message.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.welcome_message = value)}
                />
                <TextAreaField
                    label={t("close_message")}
                    value={bot.close_message.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.close_message = value)}
                />
            </FieldGrid>
            <Divider />
            <ToggleGrid>
                <ToggleRow
                    label={t("typing_proxy_user")}
                    help={t("typing_proxy_user_help")}
                    checked={bot.typing_proxy_from_user}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.typing_proxy_from_user = value)}
                />
                <ToggleRow
                    label={t("typing_proxy_staff")}
                    help={t("typing_proxy_staff_help")}
                    checked={bot.typing_proxy_from_staff}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.typing_proxy_from_staff = value)}
                />
                <ToggleRow
                    label={t("enable_rustmail_logs")}
                    help={t("enable_rustmail_logs_help")}
                    checked={bot.enable_rustmail_logs}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.enable_rustmail_logs = value)}
                />
                <ToggleRow
                    label={t("enable_discord_logs")}
                    help={t("enable_discord_logs_help")}
                    checked={bot.enable_discord_logs}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.enable_discord_logs = value)}
                />
                <ToggleRow
                    label={t("enable_features")}
                    help={t("enable_features_help")}
                    checked={bot.enable_features}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.enable_features = value)}
                />
                <ToggleRow
                    label={t("enable_panel")}
                    help={t("enable_panel_help")}
                    checked={bot.enable_panel}
                    onchange={edit(config, |cfg, value: bool| cfg.bot.enable_panel = value)}
                />
            </ToggleGrid>
            <Divider />
            <FieldGrid>
                <TextField
                    label={t("client_id")}
                    mono=true
                    value={bot.client_id.to_string()}
                    on_change={edit(config, |cfg, value: String| {
                        if let Ok(id) = value.parse::<u64>() {
                            cfg.bot.client_id = id;
                        }
                    })}
                />
                <TextField
                    label={t("client_secret")}
                    help={t("client_secret_help")}
                    mono=true
                    value={bot.client_secret.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.client_secret = value)}
                />
                <TextField
                    label={t("redirect_url")}
                    mono=true
                    value={bot.redirect_url.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.redirect_url = value)}
                />
                <TextField
                    label={t("timezone")}
                    placeholder="UTC"
                    value={bot.timezone.to_string()}
                    on_change={edit(config, |cfg, value: String| {
                        if let Ok(tz) = value.parse::<chrono_tz::Tz>() {
                            cfg.bot.timezone = tz;
                        }
                    })}
                />
                <TextField
                    label={t("logs_channel_id")}
                    help={optional.clone()}
                    placeholder={not_set.clone()}
                    mono=true
                    value={bot.logs_channel_id.map(|id| id.to_string()).unwrap_or_default()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.logs_channel_id = value.parse::<u64>().ok())}
                />
                <TextField
                    label={t("features_channel_id")}
                    help={optional}
                    placeholder={not_set}
                    mono=true
                    value={bot.features_channel_id.map(|id| id.to_string()).unwrap_or_default()}
                    on_change={edit(config, |cfg, value: String| cfg.bot.features_channel_id = value.parse::<u64>().ok())}
                />
                <TextField
                    label={t("ip")}
                    help={t("ip_help")}
                    placeholder="0.0.0.0"
                    mono=true
                    value={bot.ip.clone().unwrap_or_default()}
                    on_change={edit(config, |cfg, value: String| {
                        cfg.bot.ip = if value.is_empty() { None } else { Some(value) };
                    })}
                />
                <TextField
                    label={t("super_admin_users")}
                    help={t("super_admin_users_help")}
                    placeholder="123456789, 987654321"
                    mono=true
                    value={join_ids(&bot.panel_super_admin_users)}
                    on_change={edit(config, |cfg, value: String| cfg.bot.panel_super_admin_users = parse_id_list(&value))}
                />
                <TextField
                    label={t("super_admin_roles")}
                    help={t("super_admin_roles_help")}
                    placeholder="123456789, 987654321"
                    mono=true
                    value={join_ids(&bot.panel_super_admin_roles)}
                    on_change={edit(config, |cfg, value: String| cfg.bot.panel_super_admin_roles = parse_id_list(&value))}
                />
            </FieldGrid>
        </SectionCard>
    }
}

#[derive(Properties, PartialEq)]
struct ModeOptionProps {
    label: AttrValue,
    help: AttrValue,
    selected: bool,
    onselect: Callback<()>,
}

#[function_component(ModeOption)]
fn mode_option(props: &ModeOptionProps) -> Html {
    let onclick = {
        let onselect = props.onselect.clone();
        Callback::from(move |_: MouseEvent| onselect.emit(()))
    };
    let frame = if props.selected {
        "border-accent"
    } else {
        "border-line hover:border-line-strong"
    };
    html! {
        <button
            type="button"
            role="radio"
            aria-checked={props.selected.to_string()}
            class={classes!("flex", "items-start", "gap-3", "rounded-xl", "border", "bg-surface-raised", "px-4", "py-3", "text-left", "transition-colors", "duration-150", frame)}
            {onclick}
        >
            if props.selected {
                <span class="mt-0.5 inline-flex size-4 shrink-0 items-center justify-center rounded-full bg-accent">
                    <span class="size-1.5 rounded-full bg-fg" />
                </span>
            } else {
                <span class="mt-0.5 size-4 shrink-0 rounded-full border border-line-strong" />
            }
            <span class="flex min-w-0 flex-col gap-0.5">
                <span class="text-sm text-fg">{props.label.clone()}</span>
                <span class="help">{props.help.clone()}</span>
            </span>
        </button>
    }
}

#[function_component(ServerModeSettings)]
pub fn server_mode_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let t = |key: &str| i18n.t(&format!("panel.configuration.server_mode.{key}"));
    let is_dual = matches!(config.bot.mode, ServerMode::Dual { .. });

    let select_mode = |dual: bool| {
        let config = config.clone();
        Callback::from(move |_| {
            if matches!(config.bot.mode, ServerMode::Dual { .. }) == dual {
                return;
            }
            let mut next = (*config).clone();
            next.bot.mode = if dual {
                ServerMode::Dual {
                    community_guild_id: 0,
                    staff_guild_id: 0,
                }
            } else {
                ServerMode::Single { guild_id: 0 }
            };
            config.set(next);
        })
    };

    let ids = match config.bot.mode.clone() {
        ServerMode::Single { guild_id } => html! {
            <TextField
                label={t("guild_id")}
                mono=true
                value={guild_id.to_string()}
                on_change={edit(config, |cfg, value: String| {
                    if let Ok(id) = value.parse::<u64>() {
                        cfg.bot.mode = ServerMode::Single { guild_id: id };
                    }
                })}
            />
        },
        ServerMode::Dual {
            community_guild_id,
            staff_guild_id,
        } => html! {
            <>
                <TextField
                    label={t("community_guild_id")}
                    mono=true
                    value={community_guild_id.to_string()}
                    on_change={edit(config, move |cfg, value: String| {
                        if let Ok(id) = value.parse::<u64>() {
                            cfg.bot.mode = ServerMode::Dual { community_guild_id: id, staff_guild_id };
                        }
                    })}
                />
                <TextField
                    label={t("staff_guild_id")}
                    mono=true
                    value={staff_guild_id.to_string()}
                    on_change={edit(config, move |cfg, value: String| {
                        if let Ok(id) = value.parse::<u64>() {
                            cfg.bot.mode = ServerMode::Dual { community_guild_id, staff_guild_id: id };
                        }
                    })}
                />
            </>
        },
    };

    html! {
        <SectionCard section={SettingsSection::ServerMode}>
            <div role="radiogroup" aria-label={t("type")} class="grid grid-cols-1 gap-4 md:grid-cols-2">
                <ModeOption label={t("single")} help={t("single_help")} selected={!is_dual} onselect={select_mode(false)} />
                <ModeOption label={t("dual")} help={t("dual_help")} selected={is_dual} onselect={select_mode(true)} />
            </div>
            <FieldGrid>{ids}</FieldGrid>
        </SectionCard>
    }
}

#[function_component(CommandSettings)]
pub fn command_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    html! {
        <SectionCard section={SettingsSection::Commands}>
            <FieldGrid>
                <TextField
                    label={i18n.t("panel.configuration.commands.prefix")}
                    help={i18n.t("panel.configuration.commands.prefix_help")}
                    mono=true
                    value={config.command.prefix.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.command.prefix = value)}
                />
            </FieldGrid>
        </SectionCard>
    }
}

#[function_component(TicketSettings)]
pub fn ticket_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let thread = &config.thread;
    let t = |key: &str| i18n.t(&format!("panel.configuration.threads.{key}"));

    html! {
        <SectionCard section={SettingsSection::Tickets}>
            <FieldGrid>
                <TextField
                    label={t("inbox_category_id")}
                    mono=true
                    value={thread.inbox_category_id.to_string()}
                    on_change={edit(config, |cfg, value: String| {
                        if let Ok(id) = value.parse::<u64>() {
                            cfg.thread.inbox_category_id = id;
                        }
                    })}
                />
                <TextField
                    label={t("time_to_close")}
                    help={t("time_to_close_help")}
                    numeric=true
                    suffix={i18n.t("panel.configuration.units.seconds")}
                    value={thread.time_to_close_thread.to_string()}
                    on_change={edit(config, |cfg, value: String| {
                        if let Ok(seconds) = value.parse::<u64>() {
                            cfg.thread.time_to_close_thread = seconds;
                        }
                    })}
                />
            </FieldGrid>
            <FieldGrid columns={3}>
                <ColorField
                    label={t("user_message_color")}
                    value={thread.user_message_color.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.thread.user_message_color = value)}
                />
                <ColorField
                    label={t("staff_message_color")}
                    value={thread.staff_message_color.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.thread.staff_message_color = value)}
                />
                <ColorField
                    label={t("system_message_color")}
                    value={thread.system_message_color.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.thread.system_message_color = value)}
                />
            </FieldGrid>
            <div class="flex flex-col">
                <ToggleRow
                    label={t("embedded_message")}
                    help={t("embedded_message_help")}
                    checked={thread.embedded_message}
                    onchange={edit(config, |cfg, value: bool| cfg.thread.embedded_message = value)}
                />
                <ToggleRow
                    label={t("block_quote")}
                    help={t("block_quote_help")}
                    checked={thread.block_quote}
                    onchange={edit(config, |cfg, value: bool| cfg.thread.block_quote = value)}
                />
                <ToggleRow
                    label={t("create_by_channel")}
                    help={t("create_by_channel_help")}
                    checked={thread.create_ticket_by_create_channel}
                    onchange={edit(config, |cfg, value: bool| cfg.thread.create_ticket_by_create_channel = value)}
                />
            </div>
        </SectionCard>
    }
}

#[function_component(LanguageSettings)]
pub fn language_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let language = &config.language;
    let t = |key: &str| i18n.t(&format!("panel.configuration.languages.{key}"));

    html! {
        <SectionCard section={SettingsSection::Languages}>
            <FieldGrid>
                <TextField
                    label={t("default")}
                    placeholder="fr"
                    mono=true
                    value={language.default_language.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.language.default_language = value)}
                />
                <TextField
                    label={t("fallback")}
                    help={t("fallback_help")}
                    placeholder="en"
                    mono=true
                    value={language.fallback_language.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.language.fallback_language = value)}
                />
                <TextField
                    label={t("supported")}
                    help={t("supported_help")}
                    placeholder={t("supported_placeholder")}
                    mono=true
                    value={language.supported_languages.join(", ")}
                    on_change={edit(config, |cfg, value: String| {
                        cfg.language.supported_languages = value
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    })}
                />
            </FieldGrid>
        </SectionCard>
    }
}

#[function_component(NotificationSettings)]
pub fn notification_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let notifications = &config.notifications;
    let t = |key: &str| i18n.t(&format!("panel.configuration.notifications.{key}"));

    html! {
        <SectionCard section={SettingsSection::Notifications}>
            <ToggleGrid>
                <ToggleRow
                    label={t("success_reply")}
                    checked={notifications.show_success_on_reply}
                    onchange={edit(config, |cfg, value: bool| cfg.notifications.show_success_on_reply = value)}
                />
                <ToggleRow
                    label={t("success_edit")}
                    checked={notifications.show_success_on_edit}
                    onchange={edit(config, |cfg, value: bool| cfg.notifications.show_success_on_edit = value)}
                />
                <ToggleRow
                    label={t("partial_success_edit")}
                    checked={notifications.show_partial_success_on_edit}
                    onchange={edit(config, |cfg, value: bool| cfg.notifications.show_partial_success_on_edit = value)}
                />
                <ToggleRow
                    label={t("failure_edit")}
                    checked={notifications.show_failure_on_edit}
                    onchange={edit(config, |cfg, value: bool| cfg.notifications.show_failure_on_edit = value)}
                />
                <ToggleRow
                    label={t("success_delete")}
                    checked={notifications.show_success_on_delete}
                    onchange={edit(config, |cfg, value: bool| cfg.notifications.show_success_on_delete = value)}
                />
            </ToggleGrid>
        </SectionCard>
    }
}

#[function_component(ErrorHandlingSettings)]
pub fn error_handling_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let errors = &config.error_handling;
    let t = |key: &str| i18n.t(&format!("panel.configuration.error_handling.{key}"));

    html! {
        <SectionCard section={SettingsSection::ErrorHandling}>
            <ToggleGrid>
                <ToggleRow
                    label={t("show_detailed")}
                    help={t("show_detailed_help")}
                    checked={errors.show_detailed_errors}
                    onchange={edit(config, |cfg, value: bool| cfg.error_handling.show_detailed_errors = value)}
                />
                <ToggleRow
                    label={t("log_errors")}
                    help={t("log_errors_help")}
                    checked={errors.log_errors}
                    onchange={edit(config, |cfg, value: bool| cfg.error_handling.log_errors = value)}
                />
                <ToggleRow
                    label={t("send_embeds")}
                    checked={errors.send_error_embeds}
                    onchange={edit(config, |cfg, value: bool| cfg.error_handling.send_error_embeds = value)}
                />
                <ToggleRow
                    label={t("auto_delete")}
                    help={t("auto_delete_help")}
                    checked={errors.auto_delete_error_messages}
                    onchange={edit(config, |cfg, value: bool| cfg.error_handling.auto_delete_error_messages = value)}
                />
            </ToggleGrid>
            <FieldGrid>
                <TextField
                    label={t("ttl")}
                    help={i18n.t("panel.configuration.optional")}
                    numeric=true
                    suffix={i18n.t("panel.configuration.units.seconds")}
                    value={errors.error_message_ttl.map(|v| v.to_string()).unwrap_or_default()}
                    on_change={edit(config, |cfg, value: String| cfg.error_handling.error_message_ttl = value.parse::<u64>().ok())}
                />
            </FieldGrid>
        </SectionCard>
    }
}

#[function_component(LogsReminderSettings)]
pub fn logs_reminder_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let logs = &config.logs;
    let t = |key: &str| i18n.t(&format!("panel.configuration.logs.{key}"));

    html! {
        <SectionCard section={SettingsSection::LogsReminders}>
            <ToggleGrid>
                <ToggleRow
                    label={t("show_edit")}
                    checked={logs.show_log_on_edit}
                    onchange={edit(config, |cfg, value: bool| cfg.logs.show_log_on_edit = value)}
                />
                <ToggleRow
                    label={t("show_delete")}
                    checked={logs.show_log_on_delete}
                    onchange={edit(config, |cfg, value: bool| cfg.logs.show_log_on_delete = value)}
                />
                <ToggleRow
                    label={t("hide_ticket_renames")}
                    checked={logs.hide_ticket_rename_logs}
                    onchange={edit(config, |cfg, value: bool| cfg.logs.hide_ticket_rename_logs = value)}
                />
            </ToggleGrid>
            <FieldGrid>
                <ColorField
                    label={i18n.t("panel.configuration.reminders.embed_color")}
                    value={config.reminders.embed_color.clone()}
                    on_change={edit(config, |cfg, value: String| cfg.reminders.embed_color = value)}
                />
            </FieldGrid>
        </SectionCard>
    }
}

#[function_component(UpdateSettings)]
pub fn update_settings(props: &SectionProps) -> Html {
    let (i18n, _) = use_translation();
    let config = &props.config;
    let updates = &config.updates;
    let t = |key: &str| i18n.t(&format!("panel.configuration.updates.{key}"));

    html! {
        <SectionCard section={SettingsSection::Updates}>
            <ToggleRow
                label={t("enabled")}
                help={t("enabled_help")}
                checked={updates.enabled}
                onchange={edit(config, |cfg, value: bool| cfg.updates.enabled = value)}
            />
            <FieldGrid>
                <TextField
                    label={t("check_interval_hours")}
                    help={t("check_interval_hours_help")}
                    numeric=true
                    suffix={i18n.t("panel.configuration.units.hours")}
                    value={updates.check_interval_hours.to_string()}
                    on_change={edit(config, |cfg, value: String| {
                        if let Ok(hours) = value.parse::<u64>()
                            && hours > 0
                        {
                            cfg.updates.check_interval_hours = hours;
                        }
                    })}
                />
                <TextField
                    label={t("notify_channel_id")}
                    help={t("notify_channel_id_help")}
                    placeholder={i18n.t("panel.configuration.not_set")}
                    mono=true
                    value={updates.notify_channel_id.map(|id| id.to_string()).unwrap_or_default()}
                    on_change={edit(config, |cfg, value: String| {
                        let trimmed = value.trim();
                        if trimmed.is_empty() {
                            cfg.updates.notify_channel_id = None;
                        } else if let Ok(id) = trimmed.parse::<u64>() {
                            cfg.updates.notify_channel_id = Some(id);
                        }
                    })}
                />
            </FieldGrid>
        </SectionCard>
    }
}
