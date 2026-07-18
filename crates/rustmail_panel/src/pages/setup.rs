use crate::components::wizard::auth::authed_get;
use crate::components::wizard::layout::WizardLayout;
use crate::components::wizard::persistence::{clear_progress, load_progress, save_progress};
use crate::components::wizard::step1_token::Step1Token;
use crate::components::wizard::step2_guilds::Step2Guilds;
use crate::components::wizard::step3_thread::Step3Thread;
use crate::components::wizard::step4_panel::Step4Panel;
use crate::components::wizard::step5_language::Step5Language;
use crate::components::wizard::step6_review::Step6Review;
use crate::components::wizard::success::SuccessScreen;
use crate::components::wizard::types::WizardData;
use crate::components::wizard::welcome::Welcome;
use crate::i18n::yew::use_translation;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[allow(dead_code)]
#[derive(Deserialize)]
struct StatusResponse {
    setup_required: bool,
    step: String,
    completed: bool,
    token_prefill: Option<String>,
    panel_url: Option<String>,
    api_port: Option<u16>,
}

#[function_component(Setup)]
pub fn setup() -> Html {
    let (i18n, _) = use_translation();
    let saved_progress = load_progress();
    let current_step = use_state(|| saved_progress.as_ref().map(|(s, _)| *s).unwrap_or(0));
    let wizard_data = use_state(|| {
        saved_progress
            .as_ref()
            .map(|(_, d)| d.clone())
            .unwrap_or_default()
    });
    let is_loading = use_state(|| true);
    let is_unauthorized = use_state(|| false);
    let is_completed = use_state(|| false);
    let completed_target = use_state(|| None::<(String, u16)>);
    let has_started = use_state(|| saved_progress.is_some());

    let on_unauthorized = {
        let is_unauthorized = is_unauthorized.clone();
        Callback::from(move |_| is_unauthorized.set(true))
    };

    let on_restart = {
        let current_step = current_step.clone();
        let wizard_data = wizard_data.clone();
        let has_started = has_started.clone();
        Callback::from(move |_| {
            clear_progress();
            current_step.set(0);
            wizard_data.set(WizardData::default());
            has_started.set(false);
        })
    };

    {
        let wizard_data = wizard_data.clone();
        let is_loading = is_loading.clone();
        let is_completed = is_completed.clone();
        let completed_target = completed_target.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = authed_get("/api/setup/status").send().await {
                    if let Ok(status) = resp.json::<StatusResponse>().await {
                        if status.completed {
                            completed_target.set(Some((
                                status.panel_url.unwrap_or_default(),
                                status.api_port.unwrap_or(3002),
                            )));
                            is_completed.set(true);
                        } else if let Some(token) = status.token_prefill {
                            let mut data = (*wizard_data).clone();
                            if data.token.is_empty() {
                                data.token = token;
                                wizard_data.set(data);
                            }
                        }
                    }
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    {
        let data_dep = (*wizard_data).clone();
        let step_dep = *current_step;
        let started_dep = *has_started;
        use_effect_with(
            (data_dep, step_dep, started_dep),
            move |(data, step, started)| {
                if *started {
                    save_progress(*step, data);
                }
                || ()
            },
        );
    }

    let step_names = vec![
        i18n.t("wizard.steps.step1.title").to_string(),
        i18n.t("wizard.steps.step2.title").to_string(),
        i18n.t("wizard.steps.step3.title").to_string(),
        i18n.t("wizard.steps.step4.title").to_string(),
        i18n.t("wizard.steps.step5.title").to_string(),
        i18n.t("wizard.steps.step6.title").to_string(),
    ];

    let on_update_data = {
        let wizard_data = wizard_data.clone();
        Callback::from(move |new_data: WizardData| {
            wizard_data.set(new_data);
        })
    };

    let on_next_step = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            current_step.set(*current_step + 1);
        })
    };

    let on_prev_step = {
        let current_step = current_step.clone();
        Callback::from(move |_| {
            if *current_step > 0 {
                current_step.set(*current_step - 1);
            }
        })
    };

    if *is_loading {
        return html! {
            <div class="min-h-screen bg-slate-950 flex flex-col items-center justify-center">
                <div class="text-gray-400 animate-pulse">{ i18n.t("wizard.loading") }</div>
            </div>
        };
    }

    if *is_unauthorized {
        return html! {
            <div class="min-h-screen bg-slate-950 flex flex-col items-center justify-center gap-4 px-4 text-center">
                <div class="w-16 h-16 bg-red-500/20 rounded-full flex items-center justify-center text-red-400 ring-4 ring-red-500/10">
                    <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path></svg>
                </div>
                <h1 class="text-2xl font-bold text-white">{ i18n.t("wizard.unauthorized_title") }</h1>
                <p class="text-gray-400 max-w-md">{ i18n.t("wizard.unauthorized_description") }</p>
            </div>
        };
    }

    if *is_completed {
        let (panel_url, api_port) = (*completed_target)
            .clone()
            .unwrap_or_else(|| ((*wizard_data).panel_url.clone(), (*wizard_data).api_port));
        return html! {
            <SuccessScreen panel_url={panel_url} api_port={api_port} on_unauthorized={on_unauthorized.clone()} />
        };
    }

    if !*has_started {
        let on_start = {
            let has_started = has_started.clone();
            Callback::from(move |_| has_started.set(true))
        };

        return html! {
            <Welcome on_start={on_start} on_unauthorized={on_unauthorized.clone()} />
        };
    }

    let title = match *current_step {
        0 => i18n.t("wizard.steps.step1.title"),
        1 => i18n.t("wizard.steps.step2.title"),
        2 => i18n.t("wizard.steps.step3.title"),
        3 => i18n.t("wizard.steps.step4.title"),
        4 => i18n.t("wizard.steps.step5.title"),
        _ => i18n.t("wizard.steps.step6.title"),
    };

    let description = match *current_step {
        0 => i18n.t("wizard.steps.step1.description"),
        1 => i18n.t("wizard.steps.step2.description"),
        2 => i18n.t("wizard.steps.step3.description"),
        3 => i18n.t("wizard.steps.step4.description"),
        4 => i18n.t("wizard.steps.step5.description"),
        _ => i18n.t("wizard.steps.step6.description"),
    };

    html! {
        <WizardLayout
            current_step={*current_step}
            total_steps={step_names.len()}
            step_names={step_names}
            title={title.to_string()}
            description={description.to_string()}
            on_restart={on_restart}
        >
            if *current_step == 0 {
                <Step1Token
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_unauthorized={on_unauthorized.clone()}
                />
            } else if *current_step == 1 {
                <Step2Guilds
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
                    on_unauthorized={on_unauthorized.clone()}
                />
            } else if *current_step == 2 {
                <Step3Thread
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
                    on_unauthorized={on_unauthorized.clone()}
                />
            } else if *current_step == 3 {
                <Step4Panel
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
                    on_unauthorized={on_unauthorized.clone()}
                />
            } else if *current_step == 4 {
                <Step5Language
                    data={(*wizard_data).clone()}
                    on_update={on_update_data}
                    on_next={on_next_step}
                    on_prev={on_prev_step.clone()}
                />
            } else {
                <Step6Review
                    data={(*wizard_data).clone()}
                    on_prev={on_prev_step}
                    on_unauthorized={on_unauthorized}
                />
            }
        </WizardLayout>
    }
}
