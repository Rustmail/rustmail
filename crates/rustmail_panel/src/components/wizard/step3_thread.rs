use crate::components::wizard::auth::authed_post;
use crate::components::wizard::types::{
    ValidateChannelRequest, ValidateChannelResponse, WizardData,
};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Step3Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Step3Thread)]
pub fn step3_thread(props: &Step3Props) -> Html {
    let (i18n, _) = use_translation();
    let inbox_category_id = use_state(|| props.data.inbox_category_id.clone());
    let command_prefix = use_state(|| props.data.command_prefix.clone());
    let user_color = use_state(|| props.data.user_message_color.clone());
    let staff_color = use_state(|| props.data.staff_message_color.clone());
    let system_color = use_state(|| props.data.system_message_color.clone());

    let embedded_message = use_state(|| props.data.embedded_message);
    let block_quote = use_state(|| props.data.block_quote);
    let create_ticket_by_create_channel = use_state(|| props.data.create_ticket_by_create_channel);
    let close_on_leave = use_state(|| props.data.close_on_leave);

    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateChannelResponse>);

    // Use staff_guild_id if dual mode, else single_guild_id
    let target_guild_id = if props.data.server_mode == "dual" {
        props.data.staff_guild_id.clone()
    } else {
        props.data.single_guild_id.clone()
    };

    let on_validate_category = {
        let category_id = inbox_category_id.clone();
        let token = props.data.token.clone();
        let guild_id = target_guild_id.clone();
        let is_validating = is_validating.clone();
        let validation_result = validation_result.clone();
        let on_unauthorized = props.on_unauthorized.clone();

        Callback::from(move |_| {
            let cat_id = (*category_id).clone();
            if cat_id.trim().is_empty() {
                return;
            }

            is_validating.set(true);
            let token = token.clone();
            let guild_id = guild_id.clone();
            let validation_result = validation_result.clone();
            let is_validating = is_validating.clone();
            let on_unauthorized = on_unauthorized.clone();

            spawn_local(async move {
                let req_body = ValidateChannelRequest {
                    token,
                    guild_id,
                    channel_id: cat_id,
                };

                let res = authed_post("/api/setup/validate-channel")
                    .json(&req_body)
                    .unwrap()
                    .send()
                    .await;

                match res {
                    Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                    Ok(resp) => {
                        if let Ok(data) = resp.json::<ValidateChannelResponse>().await {
                            if data.valid {
                                if let Some(chan) = &data.channel {
                                    if chan.kind != 4 {
                                        validation_result.set(Some(ValidateChannelResponse {
                                            valid: false,
                                            channel: Some(chan.clone()),
                                            error: Some(
                                                "The specified ID is not a category".to_string(),
                                            ),
                                        }));
                                    } else {
                                        validation_result.set(Some(data));
                                    }
                                }
                            } else {
                                validation_result.set(Some(data));
                            }
                        } else {
                            validation_result.set(Some(ValidateChannelResponse {
                                valid: false,
                                channel: None,
                                error: Some("Invalid response".to_string()),
                            }));
                        }
                    }
                    Err(_) => {
                        validation_result.set(Some(ValidateChannelResponse {
                            valid: false,
                            channel: None,
                            error: Some("Network error".to_string()),
                        }));
                    }
                }

                is_validating.set(false);
            });
        })
    };

    let is_valid = validation_result.as_ref().map(|r| r.valid).unwrap_or(false);

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let data = props.data.clone();

        let inbox_category_id = inbox_category_id.clone();
        let command_prefix = command_prefix.clone();
        let user_color = user_color.clone();
        let staff_color = staff_color.clone();
        let system_color = system_color.clone();
        let embedded_message = embedded_message.clone();
        let block_quote = block_quote.clone();
        let create_ticket_by_create_channel = create_ticket_by_create_channel.clone();
        let close_on_leave = close_on_leave.clone();

        Callback::from(move |_| {
            let mut new_data = data.clone();
            new_data.inbox_category_id = (*inbox_category_id).clone();
            new_data.command_prefix = (*command_prefix).clone();
            new_data.user_message_color = (*user_color).clone().replace("#", "");
            new_data.staff_message_color = (*staff_color).clone().replace("#", "");
            new_data.system_message_color = (*system_color).clone().replace("#", "");
            new_data.embedded_message = *embedded_message;
            new_data.block_quote = *block_quote;
            new_data.create_ticket_by_create_channel = *create_ticket_by_create_channel;
            new_data.close_on_leave = *close_on_leave;

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

    html! {
        <div class="flex flex-col gap-6 animate-fade-in">
            // Inbox Category
            <div class="flex flex-col gap-2 bg-slate-800/30 p-4 rounded-xl border border-slate-700/50">
                <label class="text-sm font-medium text-gray-300">{ i18n.t("wizard.steps.step3.category_label") }</label>
                <div class="flex gap-3">
                    <input
                        type="text"
                        class="flex-1 bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-colors"
                        placeholder={i18n.t("wizard.steps.step3.category_placeholder")}
                        value={(*inbox_category_id).clone()}
                        onchange={
                            let inbox_category_id = inbox_category_id.clone();
                            let validation_result = validation_result.clone();
                            Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                inbox_category_id.set(input.value());
                                validation_result.set(None);
                            })
                        }
                    />
                    <button
                        class="px-4 py-2 bg-slate-700 hover:bg-slate-600 text-white font-medium rounded-lg transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center min-w-[100px]"
                        onclick={on_validate_category}
                        disabled={*is_validating || inbox_category_id.trim().is_empty()}
                    >
                        if *is_validating {
                            <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                        } else {
                            { i18n.t("wizard.common.verify") }
                        }
                    </button>
                </div>
                if let Some(res) = (*validation_result).as_ref() {
                    if res.valid {
                        if let Some(chan) = &res.channel {
                            <div class="mt-3 flex items-center gap-3 text-sm text-green-400">
                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path></svg>
                                { format!("Found category: {}", chan.name) }
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

            // Other Settings (no validation required)
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                // Left Column
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-medium text-gray-300">{ i18n.t("wizard.steps.step3.prefix_label") }</label>
                        <input
                            type="text"
                            placeholder={i18n.t("wizard.steps.step3.prefix_placeholder")}
                            class="bg-slate-900 border border-slate-700 rounded-lg px-4 py-2 text-white"
                            value={(*command_prefix).clone()}
                            onchange={
                                let state = command_prefix.clone();
                                Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    state.set(input.value());
                                })
                            }
                        />
                    </div>

                    <div class="flex items-center gap-3 mt-2">
                        <input
                            type="checkbox"
                            id="embedded_message"
                            class="w-4 h-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                            checked={*embedded_message}
                            onchange={
                                let state = embedded_message.clone();
                                Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    state.set(input.checked());
                                })
                            }
                        />
                        <label for="embedded_message" class="text-sm text-gray-300">{ i18n.t("wizard.steps.step3.embed_label") }</label>
                    </div>

                    <div class="flex items-center gap-3">
                        <input
                            type="checkbox"
                            id="block_quote"
                            class="w-4 h-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                            checked={*block_quote}
                            onchange={
                                let state = block_quote.clone();
                                Callback::from(move |e: Event| {
                                    let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                    state.set(input.checked());
                                })
                            }
                        />
                        <label for="block_quote" class="text-sm text-gray-300">{ i18n.t("wizard.steps.step3.use_block_quotes") }</label>
                    </div>
                </div>

                // Right Column (Colors)
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col gap-2">
                        <label class="text-sm text-gray-300">{ i18n.t("wizard.steps.step3.user_message_color") }</label>
                        <input type="color" class="w-8 h-8 rounded cursor-pointer bg-transparent border-0" value={format!("#{}", (*user_color).clone().replace("#", ""))}
                            onchange={let state = user_color.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })}
                        />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="text-sm text-gray-300">{ i18n.t("wizard.steps.step3.staff_message_color") }</label>
                        <input type="color" class="w-8 h-8 rounded cursor-pointer bg-transparent border-0" value={format!("#{}", (*staff_color).clone().replace("#", ""))}
                            onchange={let state = staff_color.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })}
                        />
                    </div>
                    <div class="flex flex-col gap-2">
                        <label class="text-sm text-gray-300">{ i18n.t("wizard.steps.step3.system_message_color") }</label>
                        <input type="color" class="w-8 h-8 rounded cursor-pointer bg-transparent border-0" value={format!("#{}", (*system_color).clone().replace("#", ""))}
                            onchange={let state = system_color.clone(); Callback::from(move |e: Event| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                state.set(input.value());
                            })}
                        />
                    </div>
                </div>
            </div>

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
