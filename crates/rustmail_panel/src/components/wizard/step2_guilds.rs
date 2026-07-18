use crate::components::wizard::auth::authed_post;
use crate::components::wizard::types::{ValidateGuildRequest, ValidateGuildResponse, WizardData};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step2Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Step2Guilds)]
pub fn step2_guilds(props: &Step2Props) -> Html {
    let (i18n, _) = use_translation();
    let server_mode = use_state(|| {
        if props.data.server_mode.is_empty() {
            "single".to_string()
        } else {
            props.data.server_mode.clone()
        }
    });

    let single_guild_id = use_state(|| props.data.single_guild_id.clone());
    let community_guild_id = use_state(|| props.data.community_guild_id.clone());
    let staff_guild_id = use_state(|| props.data.staff_guild_id.clone());

    let single_guild_result = use_state(|| None::<ValidateGuildResponse>);
    let community_guild_result = use_state(|| None::<ValidateGuildResponse>);
    let staff_guild_result = use_state(|| None::<ValidateGuildResponse>);

    let is_validating_single = use_state(|| false);
    let is_validating_community = use_state(|| false);
    let is_validating_staff = use_state(|| false);

    let on_mode_change = {
        let server_mode = server_mode.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            server_mode.set(input.value());
        })
    };

    let validate_guild = |id_val: String,
                          token_val: String,
                          is_validating: UseStateHandle<bool>,
                          result_state: UseStateHandle<Option<ValidateGuildResponse>>,
                          on_unauthorized: Callback<()>| {
        if id_val.trim().is_empty() {
            return;
        }

        is_validating.set(true);

        spawn_local(async move {
            let req_body = ValidateGuildRequest {
                token: token_val,
                guild_id: id_val,
            };

            let res = authed_post("/api/setup/validate-guild")
                .json(&req_body)
                .unwrap()
                .send()
                .await;

            match res {
                Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                Ok(resp) => {
                    if let Ok(data) = resp.json::<ValidateGuildResponse>().await {
                        result_state.set(Some(data));
                    } else {
                        result_state.set(Some(ValidateGuildResponse {
                            valid: false,
                            guild: None,
                            error: Some("Invalid response from server".to_string()),
                        }));
                    }
                }
                Err(_) => {
                    result_state.set(Some(ValidateGuildResponse {
                        valid: false,
                        guild: None,
                        error: Some("Network error".to_string()),
                    }));
                }
            }

            is_validating.set(false);
        });
    };

    let on_validate_single = {
        let id = single_guild_id.clone();
        let token = props.data.token.clone();
        let is_validating = is_validating_single.clone();
        let result_state = single_guild_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        Callback::from(move |_| {
            validate_guild(
                (*id).clone(),
                token.clone(),
                is_validating.clone(),
                result_state.clone(),
                on_unauthorized.clone(),
            )
        })
    };

    let on_validate_community = {
        let id = community_guild_id.clone();
        let token = props.data.token.clone();
        let is_validating = is_validating_community.clone();
        let result_state = community_guild_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        Callback::from(move |_| {
            validate_guild(
                (*id).clone(),
                token.clone(),
                is_validating.clone(),
                result_state.clone(),
                on_unauthorized.clone(),
            )
        })
    };

    let on_validate_staff = {
        let id = staff_guild_id.clone();
        let token = props.data.token.clone();
        let is_validating = is_validating_staff.clone();
        let result_state = staff_guild_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        Callback::from(move |_| {
            validate_guild(
                (*id).clone(),
                token.clone(),
                is_validating.clone(),
                result_state.clone(),
                on_unauthorized.clone(),
            )
        })
    };

    let is_valid = if *server_mode == "single" {
        single_guild_result
            .as_ref()
            .map(|r| r.valid)
            .unwrap_or(false)
    } else {
        community_guild_result
            .as_ref()
            .map(|r| r.valid)
            .unwrap_or(false)
            && staff_guild_result
                .as_ref()
                .map(|r| r.valid)
                .unwrap_or(false)
    };

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let server_mode = server_mode.clone();
        let single_guild_id = single_guild_id.clone();
        let community_guild_id = community_guild_id.clone();
        let staff_guild_id = staff_guild_id.clone();
        let current_data = props.data.clone();

        Callback::from(move |_| {
            let mut new_data = current_data.clone();
            new_data.server_mode = (*server_mode).clone();
            new_data.single_guild_id = (*single_guild_id).clone();
            new_data.community_guild_id = (*community_guild_id).clone();
            new_data.staff_guild_id = (*staff_guild_id).clone();
            props_on_update.emit(new_data);
            props_on_next.emit(());
        })
    };

    let on_prev = {
        let props_on_prev = props.on_prev.clone();
        Callback::from(move |_| {
            props_on_prev.emit(());
        })
    };

    let render_guild_input = |label: &str,
                              desc: &str,
                              state: UseStateHandle<String>,
                              result: UseStateHandle<Option<ValidateGuildResponse>>,
                              is_validating: UseStateHandle<bool>,
                              on_validate: Callback<MouseEvent>| {
        let on_change = {
            let state = state.clone();
            let result = result.clone();
            Callback::from(move |e: Event| {
                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                state.set(input.value());
                result.set(None);
            })
        };

        html! {
            <div class="flex flex-col gap-2 mt-4 bg-slate-800/30 p-4 rounded-xl border border-slate-700/50">
                <p class="text-xs font-medium uppercase tracking-wide text-gray-500">{ label }</p>
                <label class="text-base font-semibold text-white mb-1">{ desc }</label>
                <div class="flex gap-3">
                    <input
                        type="text"
                        class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder="e.g. 123456789012345678"
                        value={(*state).clone()}
                        onchange={on_change}
                    />
                    <button
                        class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center min-w-[100px]"
                        onclick={on_validate}
                        disabled={*is_validating || state.trim().is_empty()}
                    >
                        if *is_validating {
                            <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                        } else {
                            { i18n.t("wizard.common.verify") }
                        }
                    </button>
                </div>

                if let Some(res) = (*result).as_ref() {
                    if res.valid {
                        if let Some(guild) = &res.guild {
                            <div class="mt-3 flex items-center gap-3 text-sm text-green-400">
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                { format!("Found server: {}", guild.name) }
                            </div>
                        }
                    } else if let Some(error) = &res.error {
                        <div class="mt-3 flex items-center gap-3 text-sm text-red-400">
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                            { error }
                        </div>
                    }
                }
            </div>
        }
    };

    html! {
        <div class="flex flex-col gap-6 animate-fade-in">
            <div class="flex flex-col gap-4">
                <label class="text-sm font-medium text-gray-300">{ i18n.t("wizard.steps.step2.mode_label") }</label>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <label class={format!("relative flex cursor-pointer rounded-lg border bg-slate-900/50 p-4 shadow-sm focus:outline-none transition-all {}",
                        if *server_mode == "single" { "border-indigo-500 ring-1 ring-indigo-500" } else { "border-slate-700 hover:border-slate-600" })}>
                        <input type="radio" name="server_mode" value="single" class="sr-only" checked={*server_mode == "single"} onchange={on_mode_change.clone()} />
                        <span class="flex flex-1">
                            <span class="flex flex-col">
                                <span class="block text-sm font-medium text-white">{ i18n.t("wizard.steps.step2.mode_single") }</span>
                            </span>
                        </span>
                        if *server_mode == "single" {
                            <svg class="h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
                        }
                    </label>

                    <label class={format!("relative flex cursor-pointer rounded-lg border bg-slate-900/50 p-4 shadow-sm focus:outline-none transition-all {}",
                        if *server_mode == "dual" { "border-indigo-500 ring-1 ring-indigo-500" } else { "border-slate-700 hover:border-slate-600" })}>
                        <input type="radio" name="server_mode" value="dual" class="sr-only" checked={*server_mode == "dual"} onchange={on_mode_change} />
                        <span class="flex flex-1">
                            <span class="flex flex-col">
                                <span class="block text-sm font-medium text-white">{ i18n.t("wizard.steps.step2.mode_dual") }</span>
                            </span>
                        </span>
                        if *server_mode == "dual" {
                            <svg class="h-5 w-5 text-indigo-500" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd" /></svg>
                        }
                    </label>
                </div>
            </div>

            if *server_mode == "single" {
                { render_guild_input(
                    &i18n.t("wizard.steps.step2.guilds_label"),
                    &i18n.t("wizard.steps.step2.main_server"),
                    single_guild_id.clone(),
                    single_guild_result.clone(),
                    is_validating_single.clone(),
                    on_validate_single
                ) }
            } else {
                <div class="flex flex-col gap-4">
                    { render_guild_input(
                        &i18n.t("wizard.steps.step2.guilds_label"),
                        &i18n.t("wizard.steps.step2.community_server"),
                        community_guild_id.clone(),
                        community_guild_result.clone(),
                        is_validating_community.clone(),
                        on_validate_community
                    ) }
                    { render_guild_input(
                        &i18n.t("wizard.steps.step2.guilds_label"),
                        &i18n.t("wizard.steps.step2.staff_server"),
                        staff_guild_id.clone(),
                        staff_guild_result.clone(),
                        is_validating_staff.clone(),
                        on_validate_staff
                    ) }
                </div>
            }

            <div class="flex justify-between pt-4 mt-2 border-t border-slate-800/50">
                <button
                    class="px-6 py-2.5 bg-slate-800 hover:bg-slate-700 text-white font-medium rounded-lg transition-colors"
                    onclick={on_prev}
                >
                    { i18n.t("wizard.common.back") }
                </button>
                <button
                    class="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-lg transition-colors shadow-lg shadow-indigo-600/20 disabled:opacity-50 disabled:cursor-not-allowed"
                    onclick={on_next}
                    disabled={!is_valid}
                >
                    { i18n.t("wizard.common.next") }
                </button>
            </div>
        </div>
    }
}
