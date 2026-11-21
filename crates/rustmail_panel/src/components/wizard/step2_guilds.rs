use crate::components::ui::{Field, input_value};
use crate::components::wizard::auth::{authed_post, handle_validation_response};
use crate::components::wizard::parts::{StepNav, VerifiedLine, VerifyRow};
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

#[derive(Properties, PartialEq)]
struct ModeCardProps {
    value: AttrValue,
    selected: bool,
    title: AttrValue,
    caption: AttrValue,
    on_select: Callback<String>,
}

#[function_component(ModeCard)]
fn mode_card(props: &ModeCardProps) -> Html {
    let onclick = {
        let on_select = props.on_select.clone();
        let value = props.value.to_string();
        Callback::from(move |_: MouseEvent| on_select.emit(value.clone()))
    };
    let frame = if props.selected {
        "border-accent bg-surface-raised"
    } else {
        "border-line bg-surface hover:bg-surface-raised"
    };
    let ring = if props.selected {
        "border-accent bg-accent"
    } else {
        "border-line-strong bg-surface-raised"
    };
    html! {
        <button
            type="button"
            role="radio"
            aria-checked={props.selected.to_string()}
            class={classes!("flex", "items-start", "gap-3", "rounded-xl", "border", "p-4", "text-left", "transition-colors", "duration-150", frame)}
            {onclick}
        >
            <span class={classes!("mt-0.5", "inline-flex", "size-4", "shrink-0", "items-center", "justify-center", "rounded-full", "border", ring)}>
                if props.selected {
                    <span class="size-1.5 rounded-full bg-white" />
                }
            </span>
            <span class="flex min-w-0 flex-col gap-0.5">
                <span class="text-sm font-medium text-fg">{props.title.clone()}</span>
                <span class="text-xs text-fg-muted">{props.caption.clone()}</span>
            </span>
        </button>
    }
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

    let on_mode_select = {
        let server_mode = server_mode.clone();
        Callback::from(move |mode: String| server_mode.set(mode))
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

            if let Some(data) =
                handle_validation_response::<ValidateGuildResponse>(res, &on_unauthorized).await
            {
                result_state.set(Some(data));
            }

            is_validating.set(false);
        });
    };

    let validator =
        |id: &UseStateHandle<String>,
         is_validating: &UseStateHandle<bool>,
         result_state: &UseStateHandle<Option<ValidateGuildResponse>>| {
            let id = id.clone();
            let token = props.data.token.clone();
            let is_validating = is_validating.clone();
            let result_state = result_state.clone();
            let on_unauthorized = props.on_unauthorized.clone();
            Callback::from(move |_: MouseEvent| {
                validate_guild(
                    (*id).clone(),
                    token.clone(),
                    is_validating.clone(),
                    result_state.clone(),
                    on_unauthorized.clone(),
                )
            })
        };

    let on_validate_single = validator(
        &single_guild_id,
        &is_validating_single,
        &single_guild_result,
    );
    let on_validate_community = validator(
        &community_guild_id,
        &is_validating_community,
        &community_guild_result,
    );
    let on_validate_staff = validator(&staff_guild_id, &is_validating_staff, &staff_guild_result);

    let is_verified = |result: &UseStateHandle<Option<ValidateGuildResponse>>| {
        result.as_ref().map(|r| r.valid).unwrap_or(false)
    };

    let is_dual = *server_mode == "dual";
    let is_valid = if is_dual {
        is_verified(&community_guild_result) && is_verified(&staff_guild_result)
    } else {
        is_verified(&single_guild_result)
    };

    let on_next = {
        let props_on_next = props.on_next.clone();
        let props_on_update = props.on_update.clone();
        let server_mode = server_mode.clone();
        let single_guild_id = single_guild_id.clone();
        let community_guild_id = community_guild_id.clone();
        let staff_guild_id = staff_guild_id.clone();
        let current_data = props.data.clone();

        Callback::from(move |_: MouseEvent| {
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
        Callback::from(move |_: MouseEvent| props_on_prev.emit(()))
    };

    let render_guild_input = |label: String,
                              state: UseStateHandle<String>,
                              result: UseStateHandle<Option<ValidateGuildResponse>>,
                              is_validating: UseStateHandle<bool>,
                              on_validate: Callback<MouseEvent>| {
        let on_input = {
            let state = state.clone();
            let result = result.clone();
            Callback::from(move |e: InputEvent| {
                state.set(input_value(&e));
                result.set(None);
            })
        };
        let current = (*result).clone();
        let error = current
            .as_ref()
            .filter(|r| !r.valid)
            .and_then(|r| r.error.clone())
            .map(AttrValue::from);
        let guild = current.filter(|r| r.valid).and_then(|r| r.guild);

        html! {
            <div class="flex flex-col gap-3">
                <Field {label} help={i18n.t("wizard.steps.step2.guild_help")} {error}>
                    <VerifyRow
                        on_verify={on_validate}
                        verifying={*is_validating}
                        disabled={state.trim().is_empty()}
                    >
                        <input
                            type="text"
                            inputmode="numeric"
                            class="input input-mono"
                            placeholder="123456789012345678"
                            value={(*state).clone()}
                            oninput={on_input}
                        />
                    </VerifyRow>
                </Field>
                if let Some(guild) = guild {
                    <VerifiedLine
                        text={i18n.t("wizard.steps.step2.found_server").replace("{name}", &guild.name)}
                        meta={guild.id.clone()}
                    />
                }
            </div>
        }
    };

    let hint = if is_dual {
        i18n.t("wizard.steps.step2.verify_both_hint")
    } else {
        i18n.t("wizard.steps.step2.verify_hint")
    };

    html! {
        <>
            <div class="flex flex-col gap-6">
                <div role="radiogroup" aria-label={i18n.t("wizard.steps.step2.mode_label")} class="grid grid-cols-1 gap-3 sm:grid-cols-2">
                    <ModeCard
                        value="single"
                        selected={!is_dual}
                        title={i18n.t("wizard.steps.step2.mode_single")}
                        caption={i18n.t("wizard.steps.step2.mode_single_help")}
                        on_select={on_mode_select.clone()}
                    />
                    <ModeCard
                        value="dual"
                        selected={is_dual}
                        title={i18n.t("wizard.steps.step2.mode_dual")}
                        caption={i18n.t("wizard.steps.step2.mode_dual_help")}
                        on_select={on_mode_select}
                    />
                </div>

                if is_dual {
                    { render_guild_input(
                        i18n.t("wizard.steps.step2.community_server"),
                        community_guild_id.clone(),
                        community_guild_result.clone(),
                        is_validating_community.clone(),
                        on_validate_community,
                    ) }
                    { render_guild_input(
                        i18n.t("wizard.steps.step2.staff_server"),
                        staff_guild_id.clone(),
                        staff_guild_result.clone(),
                        is_validating_staff.clone(),
                        on_validate_staff,
                    ) }
                } else {
                    { render_guild_input(
                        i18n.t("wizard.steps.step2.main_server"),
                        single_guild_id.clone(),
                        single_guild_result.clone(),
                        is_validating_single.clone(),
                        on_validate_single,
                    ) }
                }
            </div>
            <StepNav
                on_back={on_prev}
                {on_next}
                next_disabled={!is_valid}
                {hint}
            />
        </>
    }
}
