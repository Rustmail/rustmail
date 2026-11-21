use crate::components::ui::{Field, ToggleRow, input_value};
use crate::components::wizard::auth::{authed_post, handle_validation_response};
use crate::components::wizard::parts::{StepNav, VerifiedLine, VerifyRow};
use crate::components::wizard::types::{
    ValidateChannelRequest, ValidateChannelResponse, WizardData,
};
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const CATEGORY_CHANNEL_KIND: u8 = 4;

#[derive(Properties, PartialEq)]
pub struct Step3Props {
    pub data: WizardData,
    pub on_update: Callback<WizardData>,
    pub on_next: Callback<()>,
    pub on_prev: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

fn normalized_hex(value: &str) -> String {
    value.trim().trim_start_matches('#').to_uppercase()
}

#[derive(Properties, PartialEq)]
struct ColorFieldProps {
    label: AttrValue,
    value: UseStateHandle<String>,
}

#[function_component(ColorField)]
fn color_field(props: &ColorFieldProps) -> Html {
    let hex = normalized_hex(&props.value);
    let on_pick = {
        let state = props.value.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            state.set(normalized_hex(&input.value()));
        })
    };
    let on_type = {
        let state = props.value.clone();
        Callback::from(move |e: InputEvent| state.set(normalized_hex(&input_value(&e))))
    };
    html! {
        <div class="flex min-w-0 flex-col gap-1.5">
            <span class="label">{props.label.clone()}</span>
            <div class="flex h-9 items-center gap-2 rounded-lg border border-line bg-surface-raised px-2.5 transition-colors duration-150 focus-within:border-accent focus-within:ring-[3px] focus-within:ring-accent/25">
                <label
                    class="relative size-4 shrink-0 cursor-pointer rounded border border-line-strong"
                    style={format!("background-color:#{hex}")}
                >
                    <input
                        type="color"
                        aria-label={props.label.clone()}
                        class="absolute inset-0 size-full cursor-pointer opacity-0"
                        value={format!("#{hex}")}
                        onchange={on_pick}
                    />
                </label>
                <span class="font-mono text-[13px] text-fg-muted">{"#"}</span>
                <input
                    type="text"
                    maxlength="7"
                    aria-label={props.label.clone()}
                    class="min-w-0 flex-1 bg-transparent font-mono text-[13px] text-fg focus:outline-none"
                    value={hex}
                    oninput={on_type}
                />
            </div>
        </div>
    }
}

#[function_component(Step3Thread)]
pub fn step3_thread(props: &Step3Props) -> Html {
    let (i18n, _) = use_translation();
    let inbox_category_id = use_state(|| props.data.inbox_category_id.clone());
    let command_prefix = use_state(|| props.data.command_prefix.clone());
    let user_color = use_state(|| normalized_hex(&props.data.user_message_color));
    let staff_color = use_state(|| normalized_hex(&props.data.staff_message_color));
    let system_color = use_state(|| normalized_hex(&props.data.system_message_color));

    let embedded_message = use_state(|| props.data.embedded_message);
    let block_quote = use_state(|| props.data.block_quote);
    let create_ticket_by_create_channel = use_state(|| props.data.create_ticket_by_create_channel);
    let close_on_leave = use_state(|| props.data.close_on_leave);

    let is_validating = use_state(|| false);
    let validation_result = use_state(|| None::<ValidateChannelResponse>);

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
        let not_category_message = i18n.t("wizard.steps.step3.not_category");

        Callback::from(move |_: MouseEvent| {
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
            let not_category_message = not_category_message.clone();

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

                if let Some(data) =
                    handle_validation_response::<ValidateChannelResponse>(res, &on_unauthorized)
                        .await
                {
                    if data.valid {
                        if let Some(chan) = &data.channel {
                            if chan.kind != CATEGORY_CHANNEL_KIND {
                                validation_result.set(Some(ValidateChannelResponse {
                                    valid: false,
                                    channel: Some(chan.clone()),
                                    error: Some(not_category_message),
                                }));
                            } else {
                                validation_result.set(Some(data));
                            }
                        }
                    } else {
                        validation_result.set(Some(data));
                    }
                }

                is_validating.set(false);
            });
        })
    };

    let current = (*validation_result).clone();
    let is_valid = current.as_ref().map(|r| r.valid).unwrap_or(false);
    let category_error = current
        .as_ref()
        .filter(|r| !r.valid)
        .and_then(|r| r.error.clone())
        .map(AttrValue::from);
    let category = current.filter(|r| r.valid).and_then(|r| r.channel);

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

        Callback::from(move |_: MouseEvent| {
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
        Callback::from(move |_: MouseEvent| props_on_prev.emit(()))
    };

    let on_category_input = {
        let inbox_category_id = inbox_category_id.clone();
        let validation_result = validation_result.clone();
        Callback::from(move |e: InputEvent| {
            inbox_category_id.set(input_value(&e));
            validation_result.set(None);
        })
    };

    let on_prefix_input = {
        let state = command_prefix.clone();
        Callback::from(move |e: InputEvent| state.set(input_value(&e)))
    };

    let toggle = |state: &UseStateHandle<bool>| {
        let state = state.clone();
        Callback::from(move |checked: bool| state.set(checked))
    };

    html! {
        <>
            <div class="flex flex-col gap-6">
                <div class="flex flex-col gap-3">
                    <Field label={i18n.t("wizard.steps.step3.category_label")} error={category_error}>
                        <VerifyRow
                            on_verify={on_validate_category}
                            verifying={*is_validating}
                            disabled={inbox_category_id.trim().is_empty()}
                        >
                            <input
                                type="text"
                                inputmode="numeric"
                                class="input input-mono"
                                placeholder={i18n.t("wizard.steps.step3.category_placeholder")}
                                value={(*inbox_category_id).clone()}
                                oninput={on_category_input}
                            />
                        </VerifyRow>
                    </Field>
                    if let Some(category) = category {
                        <VerifiedLine
                            text={i18n.t("wizard.steps.step3.found_category").replace("{name}", &category.name)}
                            meta={category.id.clone()}
                        />
                    }
                </div>

                <Field
                    label={i18n.t("wizard.steps.step3.prefix_label")}
                    help={i18n.t("wizard.steps.step3.prefix_help")}
                    class="max-w-[220px]"
                >
                    <input
                        type="text"
                        class="input input-mono"
                        placeholder={i18n.t("wizard.steps.step3.prefix_placeholder")}
                        value={(*command_prefix).clone()}
                        oninput={on_prefix_input}
                    />
                </Field>

                <div class="flex flex-col">
                    <ToggleRow
                        label={i18n.t("wizard.steps.step3.embed_label")}
                        help={i18n.t("wizard.steps.step3.embed_help")}
                        checked={*embedded_message}
                        onchange={toggle(&embedded_message)}
                    />
                    <ToggleRow
                        label={i18n.t("wizard.steps.step3.use_block_quotes")}
                        help={i18n.t("wizard.steps.step3.block_quotes_help")}
                        checked={*block_quote}
                        onchange={toggle(&block_quote)}
                    />
                </div>

                <div class="grid grid-cols-1 gap-3 sm:grid-cols-3">
                    <ColorField label={i18n.t("wizard.steps.step3.user_message_color")} value={user_color.clone()} />
                    <ColorField label={i18n.t("wizard.steps.step3.staff_message_color")} value={staff_color.clone()} />
                    <ColorField label={i18n.t("wizard.steps.step3.system_message_color")} value={system_color.clone()} />
                </div>
            </div>
            <StepNav
                on_back={on_prev}
                {on_next}
                next_disabled={!is_valid}
                hint={i18n.t("wizard.steps.step3.verify_hint")}
            />
        </>
    }
}
