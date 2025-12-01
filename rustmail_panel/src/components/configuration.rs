use crate::components::forbidden::Forbidden403;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use rustmail_types::*;

#[function_component(ConfigurationPage)]
pub fn configuration_page() -> Html {
    let (i18n, _set_language) = use_translation();

    let bot_status = use_state(|| "running".to_string());
    let is_loading = use_state(|| false);
    let config = use_state(|| None::<ConfigResponse>);
    let config_loading = use_state(|| true);
    let show_restart_modal = use_state(|| false);
    let save_message = use_state(|| None::<(bool, String)>);

    let expanded_sections = use_state(|| vec![true, false, false, false, false, false, false, false]);

    let permissions = use_state(|| None::<Vec<PanelPermission>>);
    {
        let permissions = permissions.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/permissions").send().await {
                    if let Ok(perms) = resp.json::<Vec<PanelPermission>>().await {
                        permissions.set(Some(perms));
                    }
                }
            });
            || ()
        });
    }

    if let Some(perms) = (*permissions).as_ref() {
        if !perms.contains(&PanelPermission::ManageConfig) {
            return html! {
                <Forbidden403 required_permission="Gérer la configuration" />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{"Vérification des permissions..."}</div>
            </div>
        };
    }

    {
        let bot_status = bot_status.clone();
        let is_loading = is_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                is_loading.set(true);
                if let Ok(resp) = Request::get("/api/bot/status").send().await {
                    if resp.ok() {
                        if let Ok(json) = resp.json::<serde_json::Value>().await {
                            if let Some(status) = json["status"].as_str() {
                                bot_status.set(status.to_string());
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
        let config = config.clone();
        let config_loading = config_loading.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                config_loading.set(true);
                if let Ok(resp) = Request::get("/api/bot/config").send().await {
                    if resp.ok() {
                        if let Ok(config_data) = resp.json::<ConfigResponse>().await {
                            config.set(Some(config_data));
                        }
                    }
                }
                config_loading.set(false);
            });
            || ()
        });
    }

    let handle_bot_action = {
        let bot_status = bot_status.clone();
        let is_loading = is_loading.clone();

        Callback::from(move |action: String| {
            let bot_status = bot_status.clone();
            let is_loading = is_loading.clone();

            spawn_local(async move {
                is_loading.set(true);
                let url = format!("/api/bot/{}", action);
                if let Ok(resp) = Request::post(&url).send().await {
                    if resp.ok() {
                        match action.as_str() {
                            "start" | "restart" => bot_status.set("running".to_string()),
                            "stop" => bot_status.set("stopped".to_string()),
                            _ => {}
                        }
                    }
                }
                is_loading.set(false);
            });
        })
    };

    let handle_save = {
        let show_restart_modal = show_restart_modal.clone();
        let save_message = save_message.clone();
        let i18n = i18n.clone();

        Callback::from(move |new_config: ConfigResponse| {
            let show_restart_modal = show_restart_modal.clone();
            let save_message = save_message.clone();
            let i18n = i18n.clone();

            spawn_local(async move {
                match Request::put("/api/bot/config").json(&new_config) {
                    Ok(req) => match req.send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                save_message.set(Some((true, i18n.t("panel.configuration.save_success"))));
                                show_restart_modal.set(true);
                            } else {
                                let error_msg = resp.text().await.unwrap_or_else(|_| "Erreur inconnue".to_string());
                                save_message.set(Some((false, error_msg)));
                            }
                        }
                        Err(e) => {
                            save_message.set(Some((false, format!("Erreur réseau : {:?}", e))));
                        }
                    },
                    Err(e) => {
                        save_message.set(Some((false, format!("Erreur : {:?}", e))));
                    }
                }
            });
        })
    };

    let handle_close_modal = {
        let show_restart_modal = show_restart_modal.clone();
        let handle_bot_action = handle_bot_action.clone();

        Callback::from(move |should_restart: bool| {
            show_restart_modal.set(false);
            if should_restart {
                handle_bot_action.emit("restart".to_string());
            }
        })
    };

    let toggle_section = {
        let expanded_sections = expanded_sections.clone();
        Callback::from(move |index: usize| {
            let mut sections = (*expanded_sections).clone();
            sections[index] = !sections[index];
            expanded_sections.set(sections);
        })
    };

    html! {
        <div class="min-h-screen px-4 sm:px-6 lg:px-8 py-8 text-white">
            <div class="max-w-4xl mx-auto">

                <div class="mb-8">
                    <h1 class="text-3xl text-white mb-2">{i18n.t("panel.configuration.title")}</h1>
                    <p class="text-gray-400">{i18n.t("panel.configuration.description")}</p>
                </div>

                <div class="mb-6 bg-slate-800/50 border border-slate-700 rounded-lg p-6">
                    <div class="flex items-center justify-between mb-4">
                        <div>
                            <h2 class="text-xl text-white mb-1">{i18n.t("panel.configuration.bot_status")}</h2>
                            <p class="text-gray-400 text-sm">{i18n.t("panel.configuration.bot_status_description")}</p>
                        </div>
                        <div class="flex items-center gap-2">
                            <div class={classes!(
                                "w-3", "h-3", "rounded-full",
                                if *bot_status == "running" { "bg-green-500" } else { "bg-red-500" }
                            )}></div>
                            <span class="text-sm text-gray-300">
                                { if *bot_status == "running" { i18n.t("panel.configuration.online") } else { i18n.t("panel.configuration.offline") } }
                            </span>
                        </div>
                    </div>

                    <div class="flex flex-wrap gap-3">
                        <button
                            onclick={{
                                let handle_bot_action = handle_bot_action.clone();
                                move |_| handle_bot_action.emit("start".to_string())
                            }}
                            disabled={*is_loading || *bot_status == "running"}
                            class="px-4 py-2 bg-green-600 hover:bg-green-700 disabled:bg-green-600/50 disabled:cursor-not-allowed text-white rounded-md transition flex items-center gap-2"
                        >
                            <i class="bi bi-play-fill"></i>
                            {i18n.t("panel.configuration.start_bot")}
                        </button>

                        <button
                            onclick={{
                                let handle_bot_action = handle_bot_action.clone();
                                move |_| handle_bot_action.emit("stop".to_string())
                            }}
                            disabled={*is_loading || *bot_status == "stopped"}
                            class="px-4 py-2 bg-red-600 hover:bg-red-700 disabled:bg-red-600/50 disabled:cursor-not-allowed text-white rounded-md transition flex items-center gap-2"
                        >
                            <i class="bi bi-stop-fill"></i>
                            {i18n.t("panel.configuration.stop_bot")}
                        </button>

                        <button
                            onclick={{
                                let handle_bot_action = handle_bot_action.clone();
                                move |_| handle_bot_action.emit("restart".to_string())
                            }}
                            disabled={*is_loading}
                            class="px-4 py-2 bg-blue-600 hover:bg-blue-700 disabled:bg-blue-600/50 disabled:cursor-not-allowed text-white rounded-md transition flex items-center gap-2"
                        >
                            <i class="bi bi-arrow-repeat"></i>
                            {i18n.t("panel.configuration.restart_bot")}
                        </button>
                    </div>
                </div>

                <div class="bg-slate-800/50 border border-slate-700 rounded-lg p-6">
                    <h2 class="text-xl text-white mb-4">{i18n.t("panel.configuration.config_file.title")}</h2>

                    {
                        if let Some((is_success, message)) = (*save_message).clone() {
                            html! {
                                <div class={classes!(
                                    "mb-4", "px-4", "py-3", "rounded-md", "border",
                                    if is_success { "bg-green-500/10 border-green-500 text-green-400" } else { "bg-red-500/10 border-red-500 text-red-400" }
                                )}>
                                    {message}
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }

                    {
                        if *config_loading {
                            html! {
                                <div class="flex justify-center items-center py-12">
                                    <div class="text-gray-400">{i18n.t("panel.configuration.loading")}</div>
                                </div>
                            }
                        } else if let Some(cfg) = (*config).clone() {
                            html! {
                                <ConfigForm
                                    config={cfg}
                                    on_save={handle_save}
                                    expanded_sections={(*expanded_sections).clone()}
                                    on_toggle_section={toggle_section}
                                />
                            }
                        } else {
                            html! {
                                <div class="text-center py-12 text-gray-400">
                                    {i18n.t("panel.configuration.load_error")}
                                </div>
                            }
                        }
                    }
                </div>

                {
                    if *show_restart_modal {
                        html! {
                            <div class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
                                <div class="bg-slate-800 border border-slate-700 rounded-lg p-6 max-w-md w-full">
                                    <h3 class="text-xl font-semibold text-white mb-4">{i18n.t("panel.configuration.restart_modal.title")}</h3>
                                    <p class="text-gray-300 mb-6">
                                        {i18n.t("panel.configuration.restart_modal.message")}
                                    </p>
                                    <div class="flex gap-3">
                                        <button
                                            onclick={{
                                                let handle_close_modal = handle_close_modal.clone();
                                                move |_| handle_close_modal.emit(true)
                                            }}
                                            class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition"
                                        >
                                            {i18n.t("panel.configuration.restart_modal.yes")}
                                        </button>
                                        <button
                                            onclick={{
                                                let handle_close_modal = handle_close_modal.clone();
                                                move |_| handle_close_modal.emit(false)
                                            }}
                                            class="flex-1 px-4 py-2 bg-slate-600 hover:bg-slate-500 text-white rounded-md transition"
                                        >
                                            {i18n.t("panel.configuration.restart_modal.later")}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ConfigFormProps {
    config: ConfigResponse,
    on_save: Callback<ConfigResponse>,
    expanded_sections: Vec<bool>,
    on_toggle_section: Callback<usize>,
}

#[function_component(ConfigForm)]
fn config_form(props: &ConfigFormProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = use_state(|| props.config.clone());

    html! {
        <div class="space-y-4">
            <AccordionSection
                title={i18n.t("panel.configuration.sections.bot")}
                is_expanded={props.expanded_sections[0]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(0))
                }}
            >
                <BotSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.server_mode")}
                is_expanded={props.expanded_sections[1]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(1))
                }}
            >
                <ServerModeSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.commands")}
                is_expanded={props.expanded_sections[2]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(2))
                }}
            >
                <CommandSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.threads")}
                is_expanded={props.expanded_sections[3]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(3))
                }}
            >
                <ThreadSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.languages")}
                is_expanded={props.expanded_sections[4]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(4))
                }}
            >
                <LanguageSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.notifications")}
                is_expanded={props.expanded_sections[5]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(5))
                }}
            >
                <NotificationsSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.error_handling")}
                is_expanded={props.expanded_sections[6]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(6))
                }}
            >
                <ErrorHandlingSection config={config.clone()} />
            </AccordionSection>

            <AccordionSection
                title={i18n.t("panel.configuration.sections.logs_reminders")}
                is_expanded={props.expanded_sections[7]}
                on_toggle={{
                    let cb = props.on_toggle_section.clone();
                    Callback::from(move |_| cb.emit(7))
                }}
            >
                <LogsReminderSection config={config.clone()} />
            </AccordionSection>

            <div class="pt-4 border-t border-slate-600">
                <button
                    onclick={{
                        let config = (*config).clone();
                        let on_save = props.on_save.clone();
                        move |_| on_save.emit(config.clone())
                    }}
                    class="w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition font-semibold flex items-center justify-center gap-2"
                >
                    <i class="bi bi-save"></i>
                    {i18n.t("panel.configuration.save")}
                </button>
                <p class="mt-2 text-xs text-gray-500 text-center">
                    {i18n.t("panel.configuration.save_help")}
                </p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct AccordionSectionProps {
    title: String,
    is_expanded: bool,
    on_toggle: Callback<()>,
    children: Children,
}

#[function_component(AccordionSection)]
fn accordion_section(props: &AccordionSectionProps) -> Html {
    html! {
        <div class="border border-slate-600 rounded-lg overflow-hidden">
            <button
                onclick={{
                    let on_toggle = props.on_toggle.clone();
                    move |_| on_toggle.emit(())
                }}
                class="w-full px-4 py-3 bg-slate-900/30 hover:bg-slate-900/50 transition flex items-center justify-between"
            >
                <h3 class="text-lg font-semibold text-white">{&props.title}</h3>
                <i class={classes!(
                    "bi",
                    if props.is_expanded { "bi-chevron-up" } else { "bi-chevron-down" },
                    "text-gray-400"
                )}></i>
            </button>

            {
                if props.is_expanded {
                    html! {
                        <div class="px-4 py-4 space-y-4">
                            {for props.children.iter()}
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TextInputProps {
    label: String,
    value: String,
    on_change: Callback<String>,
    #[prop_or_default]
    input_type: Option<String>,
    #[prop_or_default]
    placeholder: Option<String>,
    #[prop_or_default]
    help: Option<String>,
}

#[function_component(TextInput)]
fn text_input(props: &TextInputProps) -> Html {
    let input_type = props.input_type.clone().unwrap_or_else(|| "text".to_string());
    let placeholder = props.placeholder.clone().unwrap_or_default();

    html! {
        <div>
            <label class="block text-sm text-gray-300 mb-2">{&props.label}</label>
            <input
                type={input_type}
                value={props.value.clone()}
                placeholder={placeholder}
                oninput={{
                    let on_change = props.on_change.clone();
                    move |e: InputEvent| {
                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                            on_change.emit(input.value());
                        }
                    }
                }}
                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            {
                if let Some(help) = &props.help {
                    html! { <p class="mt-1 text-xs text-gray-500">{help}</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct TextAreaInputProps {
    label: String,
    value: String,
    on_change: Callback<String>,
    #[prop_or(3)]
    rows: u32,
}

#[function_component(TextAreaInput)]
fn textarea_input(props: &TextAreaInputProps) -> Html {
    html! {
        <div>
            <label class="block text-sm text-gray-300 mb-2">{&props.label}</label>
            <textarea
                value={props.value.clone()}
                rows={props.rows.to_string()}
                oninput={{
                    let on_change = props.on_change.clone();
                    move |e: InputEvent| {
                        if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                            on_change.emit(input.value());
                        }
                    }
                }}
                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CheckboxInputProps {
    label: String,
    checked: bool,
    on_change: Callback<bool>,
    #[prop_or_default]
    help: Option<String>,
}

#[function_component(CheckboxInput)]
fn checkbox_input(props: &CheckboxInputProps) -> Html {
    html! {
        <div class="flex items-center justify-between p-4 bg-slate-900/30 rounded-md border border-slate-600">
            <div>
                <label class="text-sm text-gray-300">{&props.label}</label>
                {
                    if let Some(help) = &props.help {
                        html! { <p class="text-xs text-gray-500">{help}</p> }
                    } else {
                        html! {}
                    }
                }
            </div>
            <label class="relative inline-flex items-center cursor-pointer">
                <input
                    type="checkbox"
                    checked={props.checked}
                    onchange={{
                        let on_change = props.on_change.clone();
                        move |e: Event| {
                            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                on_change.emit(input.checked());
                            }
                        }
                    }}
                    class="sr-only peer"
                />
                <div class="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
            </label>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ColorInputProps {
    label: String,
    value: String,
    on_change: Callback<String>,
}

#[function_component(ColorInput)]
fn color_input(props: &ColorInputProps) -> Html {
    html! {
        <div>
            <label class="block text-sm text-gray-300 mb-2">{&props.label}</label>
            <div class="flex gap-2 items-center">
                <div class="relative flex-1">
                    <span class="absolute left-3 top-1/2 -translate-y-1/2 text-white z-10">{"#"}</span>
                    <input
                        type="text"
                        value={props.value.clone()}
                        maxlength="6"
                        oninput={{
                            let on_change = props.on_change.clone();
                            move |e: InputEvent| {
                                if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                    on_change.emit(input.value().trim_start_matches('#').to_string());
                                }
                            }
                        }}
                        class="w-full pl-8 pr-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        placeholder="3d54ff"
                    />
                </div>
                <div
                    class="w-10 h-10 rounded border border-slate-600 flex-shrink-0"
                    style={format!("background-color: #{}", props.value)}
                ></div>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct BotSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(BotSection)]
fn bot_section(props: &BotSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-4">
            <TextInput
                label={i18n.t("panel.configuration.bot.token")}
                value={config.bot.token.clone()}
                help={Some(i18n.t("panel.configuration.bot.token_help"))}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.token = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.status")}
                value={config.bot.status.clone()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.status = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextAreaInput
                label={i18n.t("panel.configuration.bot.welcome_message")}
                value={config.bot.welcome_message.clone()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.welcome_message = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextAreaInput
                label={i18n.t("panel.configuration.bot.close_message")}
                value={config.bot.close_message.clone()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.close_message = val;
                        config.set(cfg);
                    })
                }}
            />

            <div class="grid grid-cols-2 gap-4">
                <CheckboxInput
                    label={i18n.t("panel.configuration.bot.typing_proxy_user")}
                    checked={config.bot.typing_proxy_from_user}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: bool| {
                            let mut cfg = (*config).clone();
                            cfg.bot.typing_proxy_from_user = val;
                            config.set(cfg);
                        })
                    }}
                />

                <CheckboxInput
                    label={i18n.t("panel.configuration.bot.typing_proxy_staff")}
                    checked={config.bot.typing_proxy_from_staff}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: bool| {
                            let mut cfg = (*config).clone();
                            cfg.bot.typing_proxy_from_staff = val;
                            config.set(cfg);
                        })
                    }}
                />
            </div>

            <div class="grid grid-cols-3 gap-4">
                <CheckboxInput
                    label={i18n.t("panel.configuration.bot.enable_logs")}
                    checked={config.bot.enable_logs}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: bool| {
                            let mut cfg = (*config).clone();
                            cfg.bot.enable_logs = val;
                            config.set(cfg);
                        })
                    }}
                />

                <CheckboxInput
                    label={i18n.t("panel.configuration.bot.enable_features")}
                    checked={config.bot.enable_features}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: bool| {
                            let mut cfg = (*config).clone();
                            cfg.bot.enable_features = val;
                            config.set(cfg);
                        })
                    }}
                />

                <CheckboxInput
                    label={i18n.t("panel.configuration.bot.enable_panel")}
                    checked={config.bot.enable_panel}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: bool| {
                            let mut cfg = (*config).clone();
                            cfg.bot.enable_panel = val;
                            config.set(cfg);
                        })
                    }}
                />
            </div>

            <TextInput
                label={i18n.t("panel.configuration.bot.client_id")}
                value={config.bot.client_id.to_string()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        if let Ok(id) = val.parse::<u64>() {
                            let mut cfg = (*config).clone();
                            cfg.bot.client_id = id;
                            config.set(cfg);
                        }
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.client_secret")}
                value={config.bot.client_secret.clone()}
                help={Some(i18n.t("panel.configuration.bot.client_secret_help"))}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.client_secret = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.redirect_url")}
                value={config.bot.redirect_url.clone()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.redirect_url = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.timezone")}
                value={config.bot.timezone.to_string()}
                placeholder={Some("UTC".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        if let Ok(tz) = val.parse::<chrono_tz::Tz>() {
                            let mut cfg = (*config).clone();
                            cfg.bot.timezone = tz;
                            config.set(cfg);
                        }
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.logs_channel_id")}
                value={config.bot.logs_channel_id.map(|id| id.to_string()).unwrap_or_default()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.logs_channel_id = val.parse::<u64>().ok();
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.features_channel_id")}
                value={config.bot.features_channel_id.map(|id| id.to_string()).unwrap_or_default()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.features_channel_id = val.parse::<u64>().ok();
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.bot.ip")}
                value={config.bot.ip.clone().unwrap_or_default()}
                help={Some(i18n.t("panel.configuration.bot.ip_help"))}
                placeholder={Some("0.0.0.0".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.bot.ip = if val.is_empty() { None } else { Some(val) };
                        config.set(cfg);
                    })
                }}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ServerModeSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(ServerModeSection)]
fn server_mode_section(props: &ServerModeSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();
    let is_dual = matches!(config.bot.mode, ServerMode::Dual { .. });

    html! {
        <div class="space-y-4">
            <div>
                <label class="block text-sm text-gray-300 mb-2">{i18n.t("panel.configuration.server_mode.type")}</label>
                <select
                    value={if is_dual { "dual" } else { "single" }}
                    onchange={{
                        let config = config.clone();
                        move |e: Event| {
                            if let Some(select) = e.target_dyn_into::<web_sys::HtmlSelectElement>() {
                                let mut cfg = (*config).clone();
                                cfg.bot.mode = if select.value() == "dual" {
                                    ServerMode::Dual {
                                        community_guild_id: 0,
                                        staff_guild_id: 0,
                                    }
                                } else {
                                    ServerMode::Single { guild_id: 0 }
                                };
                                config.set(cfg);
                            }
                        }
                    }}
                    class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                >
                    <option value="single">{i18n.t("panel.configuration.server_mode.single")}</option>
                    <option value="dual">{i18n.t("panel.configuration.server_mode.dual")}</option>
                </select>
            </div>

            {
                match &config.bot.mode {
                    ServerMode::Single { guild_id } => html! {
                        <TextInput
                            label={i18n.t("panel.configuration.server_mode.guild_id")}
                            value={guild_id.to_string()}
                            on_change={{
                                let config = config.clone();
                                Callback::from(move |val: String| {
                                    if let Ok(id) = val.parse::<u64>() {
                                        let mut cfg = (*config).clone();
                                        cfg.bot.mode = ServerMode::Single { guild_id: id };
                                        config.set(cfg);
                                    }
                                })
                            }}
                        />
                    },
                    ServerMode::Dual { community_guild_id, staff_guild_id } => html! {
                        <>
                            <TextInput
                                label={i18n.t("panel.configuration.server_mode.community_guild_id")}
                                value={community_guild_id.to_string()}
                                on_change={{
                                    let config = config.clone();
                                    let staff_id = *staff_guild_id;
                                    Callback::from(move |val: String| {
                                        if let Ok(id) = val.parse::<u64>() {
                                            let mut cfg = (*config).clone();
                                            cfg.bot.mode = ServerMode::Dual {
                                                community_guild_id: id,
                                                staff_guild_id: staff_id,
                                            };
                                            config.set(cfg);
                                        }
                                    })
                                }}
                            />

                            <TextInput
                                label={i18n.t("panel.configuration.server_mode.staff_guild_id")}
                                value={staff_guild_id.to_string()}
                                on_change={{
                                    let config = config.clone();
                                    let community_id = *community_guild_id;
                                    Callback::from(move |val: String| {
                                        if let Ok(id) = val.parse::<u64>() {
                                            let mut cfg = (*config).clone();
                                            cfg.bot.mode = ServerMode::Dual {
                                                community_guild_id: community_id,
                                                staff_guild_id: id,
                                            };
                                            config.set(cfg);
                                        }
                                    })
                                }}
                            />
                        </>
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CommandSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(CommandSection)]
fn command_section(props: &CommandSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <TextInput
            label={i18n.t("panel.configuration.commands.prefix")}
            value={config.command.prefix.clone()}
            on_change={{
                Callback::from(move |val: String| {
                    let mut cfg = (*config).clone();
                    cfg.command.prefix = val;
                    config.set(cfg);
                })
            }}
        />
    }
}

#[derive(Properties, PartialEq)]
struct ThreadSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(ThreadSection)]
fn thread_section(props: &ThreadSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-4">
            <TextInput
                label={i18n.t("panel.configuration.threads.inbox_category_id")}
                value={config.thread.inbox_category_id.to_string()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        if let Ok(id) = val.parse::<u64>() {
                            let mut cfg = (*config).clone();
                            cfg.thread.inbox_category_id = id;
                            config.set(cfg);
                        }
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.threads.embedded_message")}
                checked={config.thread.embedded_message}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.thread.embedded_message = val;
                        config.set(cfg);
                    })
                }}
            />

            <div class="grid grid-cols-3 gap-4">
                <ColorInput
                    label={i18n.t("panel.configuration.threads.user_message_color")}
                    value={config.thread.user_message_color.clone()}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: String| {
                            let mut cfg = (*config).clone();
                            cfg.thread.user_message_color = val;
                            config.set(cfg);
                        })
                    }}
                />

                <ColorInput
                    label={i18n.t("panel.configuration.threads.staff_message_color")}
                    value={config.thread.staff_message_color.clone()}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: String| {
                            let mut cfg = (*config).clone();
                            cfg.thread.staff_message_color = val;
                            config.set(cfg);
                        })
                    }}
                />

                <ColorInput
                    label={i18n.t("panel.configuration.threads.system_message_color")}
                    value={config.thread.system_message_color.clone()}
                    on_change={{
                        let config = config.clone();
                        Callback::from(move |val: String| {
                            let mut cfg = (*config).clone();
                            cfg.thread.system_message_color = val;
                            config.set(cfg);
                        })
                    }}
                />
            </div>

            <CheckboxInput
                label={i18n.t("panel.configuration.threads.block_quote")}
                checked={config.thread.block_quote}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.thread.block_quote = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.threads.time_to_close")}
                value={config.thread.time_to_close_thread.to_string()}
                input_type={Some("number".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        if let Ok(time) = val.parse::<u64>() {
                            let mut cfg = (*config).clone();
                            cfg.thread.time_to_close_thread = time;
                            config.set(cfg);
                        }
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.threads.create_by_channel")}
                checked={config.thread.create_ticket_by_create_channel}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.thread.create_ticket_by_create_channel = val;
                        config.set(cfg);
                    })
                }}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LanguageSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(LanguageSection)]
fn language_section(props: &LanguageSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-4">
            <TextInput
                label={i18n.t("panel.configuration.languages.default")}
                value={config.language.default_language.clone()}
                placeholder={Some("fr".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.language.default_language = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.languages.fallback")}
                value={config.language.fallback_language.clone()}
                placeholder={Some("en".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.language.fallback_language = val;
                        config.set(cfg);
                    })
                }}
            />

            <div>
                <label class="block text-sm text-gray-300 mb-2">{i18n.t("panel.configuration.languages.supported")}</label>
                <input
                    type="text"
                    value={config.language.supported_languages.join(", ")}
                    placeholder={i18n.t("panel.configuration.languages.supported_placeholder")}
                    oninput={{
                        let config = config.clone();
                        move |e: InputEvent| {
                            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                let mut cfg = (*config).clone();
                                cfg.language.supported_languages = input.value()
                                    .split(',')
                                    .map(|s| s.trim().to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect();
                                config.set(cfg);
                            }
                        }
                    }}
                    class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct NotificationsSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(NotificationsSection)]
fn notifications_section(props: &NotificationsSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-2">
            <CheckboxInput
                label={i18n.t("panel.configuration.notifications.success_edit")}
                checked={config.notifications.show_success_on_edit}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.notifications.show_success_on_edit = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.notifications.partial_success_edit")}
                checked={config.notifications.show_partial_success_on_edit}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.notifications.show_partial_success_on_edit = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.notifications.failure_edit")}
                checked={config.notifications.show_failure_on_edit}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.notifications.show_failure_on_edit = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.notifications.success_reply")}
                checked={config.notifications.show_success_on_reply}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.notifications.show_success_on_reply = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.notifications.success_delete")}
                checked={config.notifications.show_success_on_delete}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.notifications.show_success_on_delete = val;
                        config.set(cfg);
                    })
                }}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ErrorHandlingSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(ErrorHandlingSection)]
fn error_handling_section(props: &ErrorHandlingSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-2">
            <CheckboxInput
                label={i18n.t("panel.configuration.error_handling.show_detailed")}
                checked={config.error_handling.show_detailed_errors}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.error_handling.show_detailed_errors = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.error_handling.log_errors")}
                checked={config.error_handling.log_errors}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.error_handling.log_errors = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.error_handling.send_embeds")}
                checked={config.error_handling.send_error_embeds}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.error_handling.send_error_embeds = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.error_handling.auto_delete")}
                checked={config.error_handling.auto_delete_error_messages}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.error_handling.auto_delete_error_messages = val;
                        config.set(cfg);
                    })
                }}
            />

            <TextInput
                label={i18n.t("panel.configuration.error_handling.ttl")}
                value={config.error_handling.error_message_ttl.map(|v| v.to_string()).unwrap_or_default()}
                input_type={Some("number".to_string())}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.error_handling.error_message_ttl = val.parse::<u64>().ok();
                        config.set(cfg);
                    })
                }}
            />
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct LogsReminderSectionProps {
    config: UseStateHandle<ConfigResponse>,
}

#[function_component(LogsReminderSection)]
fn logs_reminder_section(props: &LogsReminderSectionProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let config = props.config.clone();

    html! {
        <div class="space-y-4">
            <h4 class="text-md font-semibold text-white border-b border-slate-600 pb-2">{i18n.t("panel.configuration.logs.title")}</h4>

            <CheckboxInput
                label={i18n.t("panel.configuration.logs.show_edit")}
                checked={config.logs.show_log_on_edit}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.logs.show_log_on_edit = val;
                        config.set(cfg);
                    })
                }}
            />

            <CheckboxInput
                label={i18n.t("panel.configuration.logs.show_delete")}
                checked={config.logs.show_log_on_delete}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: bool| {
                        let mut cfg = (*config).clone();
                        cfg.logs.show_log_on_delete = val;
                        config.set(cfg);
                    })
                }}
            />

            <h4 class="text-md font-semibold text-white border-b border-slate-600 pb-2 mt-6">{i18n.t("panel.configuration.reminders.title")}</h4>

            <ColorInput
                label={i18n.t("panel.configuration.reminders.embed_color")}
                value={config.reminders.embed_color.clone()}
                on_change={{
                    let config = config.clone();
                    Callback::from(move |val: String| {
                        let mut cfg = (*config).clone();
                        cfg.reminders.embed_color = val;
                        config.set(cfg);
                    })
                }}
            />
        </div>
    }
}
