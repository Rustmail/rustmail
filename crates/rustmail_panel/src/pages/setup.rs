use crate::components::ui::Loading;
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
use crate::components::wizard::unauthorized::Unauthorized;
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

#[derive(Clone, PartialEq)]
enum PageMode {
    Loading,
    Unauthorized,
    Completed { panel_url: String, api_port: u16 },
    Welcome,
    Wizard,
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
    let page_mode = use_state(|| PageMode::Loading);

    let on_unauthorized = {
        let page_mode = page_mode.clone();
        Callback::from(move |_| page_mode.set(PageMode::Unauthorized))
    };

    let on_restart = {
        let current_step = current_step.clone();
        let wizard_data = wizard_data.clone();
        let page_mode = page_mode.clone();
        Callback::from(move |_| {
            clear_progress();
            current_step.set(0);
            wizard_data.set(WizardData::default());
            page_mode.set(PageMode::Welcome);
        })
    };

    {
        let wizard_data = wizard_data.clone();
        let page_mode = page_mode.clone();
        let has_saved_progress = saved_progress.is_some();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let mut completed_mode = None;
                if let Ok(resp) = authed_get("/api/setup/status").send().await
                    && let Ok(status) = resp.json::<StatusResponse>().await
                {
                    if status.completed {
                        completed_mode = Some(PageMode::Completed {
                            panel_url: status.panel_url.unwrap_or_default(),
                            api_port: status.api_port.unwrap_or(3002),
                        });
                    } else if let Some(token) = status.token_prefill {
                        let mut data = (*wizard_data).clone();
                        if data.token.is_empty() {
                            data.token = token;
                            wizard_data.set(data);
                        }
                    }
                }
                page_mode.set(completed_mode.unwrap_or(if has_saved_progress {
                    PageMode::Wizard
                } else {
                    PageMode::Welcome
                }));
            });
            || ()
        });
    }

    {
        let data_dep = (*wizard_data).clone();
        let step_dep = *current_step;
        let mode_dep = (*page_mode).clone();
        use_effect_with((data_dep, step_dep, mode_dep), move |(data, step, mode)| {
            if *mode == PageMode::Wizard {
                save_progress(*step, data);
            }
            || ()
        });
    }

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

    match (*page_mode).clone() {
        PageMode::Loading => html! {
            <main class="flex min-h-screen items-center justify-center bg-surface-page">
                <Loading label={i18n.t("wizard.loading")} />
            </main>
        },
        PageMode::Unauthorized => html! { <Unauthorized /> },
        PageMode::Completed {
            panel_url,
            api_port,
        } => html! {
            <SuccessScreen panel_url={panel_url} api_port={api_port} on_unauthorized={on_unauthorized.clone()} />
        },
        PageMode::Welcome => {
            let on_start = {
                let page_mode = page_mode.clone();
                Callback::from(move |_| page_mode.set(PageMode::Wizard))
            };

            html! {
                <Welcome on_start={on_start} on_unauthorized={on_unauthorized.clone()} />
            }
        }
        PageMode::Wizard => {
            let on_saved = {
                let page_mode = page_mode.clone();
                let wizard_data = wizard_data.clone();
                Callback::from(move |_| {
                    page_mode.set(PageMode::Completed {
                        panel_url: wizard_data.panel_url.clone(),
                        api_port: wizard_data.api_port,
                    })
                })
            };

            html! {
                <WizardLayout current_step={*current_step} on_restart={on_restart}>
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
                            {on_saved}
                            on_unauthorized={on_unauthorized}
                        />
                    }
                </WizardLayout>
            }
        }
    }
}
