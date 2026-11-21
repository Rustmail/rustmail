use crate::components::ui::{Notice, NoticeTone};
use crate::components::wizard::auth::authed_post;
use crate::components::wizard::parts::StepNav;
use crate::components::wizard::step5_language::LOCALES;
use crate::components::wizard::types::WizardData;
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step6Props {
    pub data: WizardData,
    pub on_prev: Callback<()>,
    pub on_saved: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

fn review_row(label: String, value: String, mono: bool) -> Html {
    let value_class = if mono {
        "font-mono text-[13px]"
    } else {
        "text-sm"
    };
    html! {
        <div class="flex min-h-11 items-center gap-4 border-t border-line px-4 py-2.5 first:border-t-0">
            <span class="w-40 shrink-0 text-[13px] text-fg-secondary">{label}</span>
            <span class={classes!("min-w-0", "flex-1", "break-all", "text-fg", value_class)}>{value}</span>
        </div>
    }
}

#[function_component(Step6Review)]
pub fn step6_review(props: &Step6Props) -> Html {
    let (i18n, _) = use_translation();
    let is_saving = use_state(|| false);
    let save_error = use_state(|| None::<String>);

    let on_save = {
        let data = props.data.clone();
        let is_saving = is_saving.clone();
        let save_error = save_error.clone();
        let on_saved = props.on_saved.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        let failed_message = i18n.t("wizard.steps.step6.save_failed_generic");
        let network_message = i18n.t("wizard.steps.step6.save_network_error");

        Callback::from(move |_: MouseEvent| {
            is_saving.set(true);
            save_error.set(None);

            let is_saving = is_saving.clone();
            let save_error = save_error.clone();
            let data = data.clone();
            let on_saved = on_saved.clone();
            let on_unauthorized = on_unauthorized.clone();
            let failed_message = failed_message.clone();
            let network_message = network_message.clone();

            spawn_local(async move {
                let req_body = serde_json::json!({
                    "token": data.token,
                    "bot_status": data.status,
                    "welcome_message": data.direct_message,
                    "close_message": data.close_message,
                    "typing_proxy_from_user": true,
                    "typing_proxy_from_staff": true,
                    "server_mode": data.server_mode,
                    "guild_id": if data.server_mode == "single" { data.single_guild_id.parse::<u64>().ok() } else { None },
                    "community_guild_id": if data.server_mode == "dual" { data.community_guild_id.parse::<u64>().ok() } else { None },
                    "staff_guild_id": if data.server_mode == "dual" { data.staff_guild_id.parse::<u64>().ok() } else { None },
                    "enable_rustmail_logs": false,
                    "enable_discord_logs": false,
                    "logs_channel_id": null,
                    "enable_features": false,
                    "features_channel_id": null,
                    "enable_panel": true,
                    "panel_url": data.panel_url,
                    "api_port": data.api_port,
                    "client_id": data.client_id.parse::<u64>().ok(),
                    "client_secret": data.client_secret,
                    "redirect_url": data.redirect_url,
                    "panel_super_admin_users": [],
                    "inbox_category_id": data.inbox_category_id.parse::<u64>().unwrap_or(0),
                    "command_prefix": data.command_prefix,
                    "user_message_color": data.user_message_color,
                    "staff_message_color": data.staff_message_color,
                    "system_message_color": data.system_message_color,
                    "embedded_message": data.embedded_message,
                    "block_quote": data.block_quote,
                    "time_to_close_thread": data.time_to_close_thread,
                    "create_ticket_by_create_channel": data.create_ticket_by_create_channel,
                    "close_on_leave": data.close_on_leave,
                    "auto_archive_duration": data.auto_archive_duration,
                    "default_language": data.locale.clone(),
                    "fallback_language": data.locale,
                    "timezone": data.timezone,
                    "check_updates": data.check_updates,
                });

                let res = authed_post("/api/setup/save")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                    Ok(resp) if resp.ok() => {
                        is_saving.set(false);
                        on_saved.emit(());
                        return;
                    }
                    Ok(resp) => {
                        let message = resp
                            .json::<serde_json::Value>()
                            .await
                            .ok()
                            .and_then(|body| {
                                body.get("error")
                                    .and_then(|e| e.as_str())
                                    .map(str::to_string)
                            })
                            .unwrap_or(failed_message);
                        save_error.set(Some(message));
                    }
                    Err(_) => save_error.set(Some(network_message)),
                }

                is_saving.set(false);
            });
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_: MouseEvent| props_on_prev.emit(()))
    };

    let data = &props.data;
    let is_dual = data.server_mode == "dual";
    let mode = if is_dual {
        i18n.t("wizard.steps.step6.dual_server")
    } else {
        i18n.t("wizard.steps.step6.single_server")
    };
    let language = LOCALES
        .iter()
        .find(|(code, _)| *code == data.locale)
        .map(|(_, name)| name.to_string())
        .unwrap_or_else(|| data.locale.clone());

    html! {
        <>
            <div class="flex flex-col gap-4">
                <div class="overflow-hidden rounded-xl border border-line bg-surface">
                    {review_row(i18n.t("wizard.steps.step6.mode"), mode, false)}
                    if is_dual {
                        {review_row(i18n.t("wizard.steps.step6.community_guild"), data.community_guild_id.clone(), true)}
                        {review_row(i18n.t("wizard.steps.step6.staff_guild"), data.staff_guild_id.clone(), true)}
                    } else {
                        {review_row(i18n.t("wizard.steps.step6.guild_id"), data.single_guild_id.clone(), true)}
                    }
                    {review_row(i18n.t("wizard.steps.step6.category_id"), data.inbox_category_id.clone(), true)}
                    {review_row(i18n.t("wizard.steps.step3.prefix_label"), data.command_prefix.clone(), true)}
                    {review_row(i18n.t("wizard.steps.step5.language_label"), language, false)}
                    {review_row(i18n.t("wizard.steps.step5.timezone_label"), data.timezone.clone(), false)}
                    {review_row(i18n.t("wizard.steps.step4.panel_url_label"), data.panel_url.clone(), true)}
                </div>

                if let Some(error) = (*save_error).as_ref() {
                    <Notice tone={NoticeTone::Negative}>
                        <div class="flex flex-col gap-0.5">
                            <span class="font-medium">{i18n.t("wizard.steps.step6.save_failed")}</span>
                            <span class="text-fg-secondary">{error}</span>
                        </div>
                    </Notice>
                }
            </div>
            <StepNav
                on_back={on_prev}
                on_next={on_save}
                finish=true
                busy={*is_saving}
            />
        </>
    }
}
