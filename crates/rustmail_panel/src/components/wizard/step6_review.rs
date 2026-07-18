use crate::components::wizard::auth::authed_post;
use crate::components::wizard::success::SuccessScreen;
use crate::components::wizard::types::WizardData;
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step6Props {
    pub data: WizardData,
    pub on_prev: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Step6Review)]
pub fn step6_review(props: &Step6Props) -> Html {
    let (i18n, _) = use_translation();
    let is_saving = use_state(|| false);
    let save_success = use_state(|| false);
    let save_error = use_state(|| None::<String>);

    let on_save = {
        let data = props.data.clone();
        let is_saving = is_saving.clone();
        let save_success = save_success.clone();
        let save_error = save_error.clone();
        let on_unauthorized = props.on_unauthorized.clone();

        Callback::from(move |_| {
            is_saving.set(true);
            save_error.set(None);

            let is_saving = is_saving.clone();
            let save_success = save_success.clone();
            let save_error = save_error.clone();
            let data = data.clone();
            let on_unauthorized = on_unauthorized.clone();

            spawn_local(async move {
                // Map WizardData to SaveConfigRequest format expected by backend
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
                });

                let res = authed_post("/api/setup/save")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                    Ok(resp) => {
                        if resp.ok() {
                            save_success.set(true);
                        } else {
                            if let Ok(err_json) = resp.json::<serde_json::Value>().await {
                                if let Some(err_msg) =
                                    err_json.get("error").and_then(|e| e.as_str())
                                {
                                    save_error.set(Some(err_msg.to_string()));
                                } else {
                                    save_error
                                        .set(Some("Failed to save configuration.".to_string()));
                                }
                            } else {
                                save_error.set(Some("Failed to save configuration.".to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        save_error.set(Some("Network error while saving.".to_string()));
                    }
                }

                is_saving.set(false);
            });
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_| {
            props_on_prev.emit(());
        })
    };

    if *save_success {
        return html! {
            <SuccessScreen panel_url={props.data.panel_url.clone()} api_port={props.data.api_port} />
        };
    }

    html! {
        <div class="flex flex-col gap-6 animate-fade-in">
            <div class="bg-slate-800/30 p-5 rounded-xl border border-slate-700/50">
                <h3 class="text-lg font-medium text-white mb-4">{ i18n.t("wizard.steps.step6.title") }</h3>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4">
                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step6.mode") }</span>
                        <span class="text-white text-sm font-medium">{ if props.data.server_mode == "dual" { i18n.t("wizard.steps.step6.dual_server") } else { i18n.t("wizard.steps.step6.single_server") } }</span>
                    </div>

                    if props.data.server_mode == "single" {
                        <div class="flex justify-between border-b border-slate-700/50 pb-2">
                            <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step6.guild_id") }</span>
                            <span class="text-white text-sm font-mono">{ &props.data.single_guild_id }</span>
                        </div>
                    } else {
                        <div class="flex justify-between border-b border-slate-700/50 pb-2">
                            <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step6.community_guild") }</span>
                            <span class="text-white text-sm font-mono">{ &props.data.community_guild_id }</span>
                        </div>
                        <div class="flex justify-between border-b border-slate-700/50 pb-2">
                            <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step6.staff_guild") }</span>
                            <span class="text-white text-sm font-mono">{ &props.data.staff_guild_id }</span>
                        </div>
                    }

                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step6.category_id") }</span>
                        <span class="text-white text-sm font-mono">{ &props.data.inbox_category_id }</span>
                    </div>

                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step3.prefix_label") }</span>
                        <span class="text-white text-sm font-mono">{ &props.data.command_prefix }</span>
                    </div>

                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step5.language_label") }</span>
                        <span class="text-white text-sm uppercase">{ &props.data.locale }</span>
                    </div>

                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step5.timezone_label") }</span>
                        <span class="text-white text-sm">{ &props.data.timezone }</span>
                    </div>

                    <div class="flex justify-between border-b border-slate-700/50 pb-2">
                        <span class="text-gray-400 text-sm">{ i18n.t("wizard.steps.step4.panel_url_label") }</span>
                        <span class="text-white text-sm font-mono">{ &props.data.panel_url }</span>
                    </div>
                </div>
            </div>

            if let Some(error) = (*save_error).as_ref() {
                <div class="bg-red-500/10 border border-red-500/20 rounded-xl p-4 flex items-start gap-3">
                    <svg class="w-5 h-5 text-red-400 mt-0.5 shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                    <div>
                        <h3 class="text-red-400 font-medium">{ i18n.t("wizard.steps.step6.save_failed") }</h3>
                        <p class="text-sm text-red-300/80 mt-0.5">{ error }</p>
                    </div>
                </div>
            }

            <div class="flex justify-between pt-4 mt-2 border-t border-slate-800/50">
                <button
                    class="px-6 py-2.5 bg-slate-800 hover:bg-slate-700 text-white font-medium rounded-lg transition-colors"
                    onclick={on_prev}
                    disabled={*is_saving}
                >
                    { i18n.t("wizard.common.back") }
                </button>
                <button
                    class="px-6 py-2.5 bg-green-600 hover:bg-green-500 text-white font-medium rounded-lg transition-colors shadow-lg shadow-green-600/20 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center min-w-[160px]"
                    onclick={on_save}
                    disabled={*is_saving}
                >
                    if *is_saving {
                        <svg class="animate-spin h-5 w-5 text-white mr-2" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                        { i18n.t("wizard.steps.step6.saving") }
                    } else {
                        { i18n.t("wizard.steps.step6.save_finish") }
                    }
                </button>
            </div>
        </div>
    }
}
