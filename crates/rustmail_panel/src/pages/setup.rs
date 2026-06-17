use crate::components::wizard::layout::WizardLayout;
use crate::components::wizard::step1_token::Step1Token;
use crate::components::wizard::step2_guilds::Step2Guilds;
use crate::components::wizard::step3_thread::Step3Thread;
use crate::components::wizard::step4_panel::Step4Panel;
use crate::components::wizard::step5_language::Step5Language;
use crate::components::wizard::step6_review::Step6Review;
use crate::components::wizard::types::WizardData;
use crate::i18n::yew::use_translation;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[allow(dead_code)]
#[derive(Deserialize)]
struct StatusResponse {
    setup_required: bool,
    step: String,
    token_prefill: Option<String>,
}

#[function_component(Setup)]
pub fn setup() -> Html {
    let (i18n, _) = use_translation();
    let current_step = use_state(|| 0);
    let wizard_data = use_state(|| WizardData::default());
    let is_loading = use_state(|| true);

    {
        let wizard_data = wizard_data.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/setup/status").send().await {
                    if let Ok(status) = resp.json::<StatusResponse>().await {
                        if let Some(token) = status.token_prefill {
                            let mut data = (*wizard_data).clone();
                            data.token = token;
                            wizard_data.set(data);
                        }
                    }
                }
                is_loading.set(false);
            });
            || ()
        });
    }

    let step_names = vec![
        "Bot Token",
        "Guilds",
        "Thread Config",
        "Web Panel",
        "Language",
        "Review",
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
        >
            if *current_step == 0 {
                <Step1Token
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                />
            } else if *current_step == 1 {
                <Step2Guilds
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
                />
            } else if *current_step == 2 {
                <Step3Thread
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
                />
            } else if *current_step == 3 {
                <Step4Panel
                    data={(*wizard_data).clone()}
                    on_update={on_update_data.clone()}
                    on_next={on_next_step.clone()}
                    on_prev={on_prev_step.clone()}
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
                />
            }
        </WizardLayout>
    }
}
