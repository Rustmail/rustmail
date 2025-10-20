use gloo_net::http::Request;
use i18nrs::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(ConfigurationPage)]
pub fn configuration_page() -> Html {
    let (i18n, _set_language) = use_translation();

    let bot_status = use_state(|| "running".to_string());
    let is_loading = use_state(|| false);

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

    let bot_token = use_state(|| "".to_string());
    let guild_id = use_state(|| "".to_string());
    let ticket_category = use_state(|| "".to_string());
    let log_channel = use_state(|| "".to_string());
    let support_role = use_state(|| "".to_string());
    let auto_close = use_state(|| "".to_string());
    let enable_transcripts = use_state(|| false);

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
        let bot_token = bot_token.clone();
        let guild_id = guild_id.clone();
        let ticket_category = ticket_category.clone();
        let log_channel = log_channel.clone();
        let support_role = support_role.clone();
        let auto_close = auto_close.clone();
        let enable_transcripts = enable_transcripts.clone();

        Callback::from(move |_| {
            let data = serde_json::json!({
                "bot_token": *bot_token,
                "guild_id": *guild_id,
                "ticket_category": *ticket_category,
                "log_channel": *log_channel,
                "support_role": *support_role,
                "auto_close": *auto_close,
                "enable_transcripts": *enable_transcripts,
            });

            spawn_local(async move {
                let _ = Request::post("/api/bot/config")
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&data).unwrap())
                    .expect("REASON")
                    .send()
                    .await;
            });
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

                    <div class="space-y-6">

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"Token du Bot"}</label>
                            <input
                                message_type="password"
                                placeholder="Votre token Discord"
                                value={(*bot_token).clone()}
                                oninput={{
                                    let bot_token = bot_token.clone();
                                    move |e: InputEvent| {
                                        bot_token.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"Le token de votre bot Discord"}</p>
                        </div>

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"ID du Serveur (Guild ID)"}</label>
                            <input
                                message_type="text"
                                placeholder="123456789012345678"
                                value={(*guild_id).clone()}
                                oninput={{
                                    let guild_id = guild_id.clone();
                                    move |e: InputEvent| {
                                        guild_id.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"L'ID de votre serveur Discord"}</p>
                        </div>

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"ID de la Catégorie Tickets"}</label>
                            <input
                                message_type="text"
                                placeholder="123456789012345678"
                                value={(*ticket_category).clone()}
                                oninput={{
                                    let ticket_category = ticket_category.clone();
                                    move |e: InputEvent| {
                                        ticket_category.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"La catégorie où les tickets seront créés"}</p>
                        </div>

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"ID du Canal de Logs"}</label>
                            <input
                                message_type="text"
                                placeholder="123456789012345678"
                                value={(*log_channel).clone()}
                                oninput={{
                                    let log_channel = log_channel.clone();
                                    move |e: InputEvent| {
                                        log_channel.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"Le canal où les logs seront envoyés"}</p>
                        </div>

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"ID du Rôle Support"}</label>
                            <input
                                message_type="text"
                                placeholder="123456789012345678"
                                value={(*support_role).clone()}
                                oninput={{
                                    let support_role = support_role.clone();
                                    move |e: InputEvent| {
                                        support_role.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"Le rôle qui peut gérer les tickets"}</p>
                        </div>

                        <div>
                            <label class="block text-sm text-gray-300 mb-2">{"Délai de Fermeture Automatique (heures)"}</label>
                            <input
                                message_type="number"
                                placeholder="24"
                                value={(*auto_close).clone()}
                                oninput={{
                                    let auto_close = auto_close.clone();
                                    move |e: InputEvent| {
                                        auto_close.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    }
                                }}
                                class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                            />
                            <p class="mt-1 text-xs text-gray-500">{"Temps d'inactivité avant fermeture automatique"}</p>
                        </div>

                        <div class="flex items-center justify-between p-4 bg-slate-900/30 rounded-md border border-slate-600">
                            <div>
                                <label class="text-sm text-gray-300">{"Activer les Transcriptions"}</label>
                                <p class="text-xs text-gray-500">{"Sauvegarde automatique des conversations"}</p>
                            </div>

                            <label class="relative inline-flex items-center cursor-pointer">
                                <input
                                    message_type="checkbox"
                                    checked={*enable_transcripts}
                                    onchange={{
                                        let enable_transcripts = enable_transcripts.clone();
                                        move |e: Event| {
                                            let checked = e.target_unchecked_into::<web_sys::HtmlInputElement>().checked();
                                            enable_transcripts.set(checked);
                                        }
                                    }}
                                    class="sr-only peer"
                                />
                                <div class="w-11 h-6 bg-gray-700 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-blue-500 rounded-full peer peer-checked:after:translate-x-full after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all peer-checked:bg-blue-600"></div>
                            </label>
                        </div>

                        <div class="pt-4">
                            <button onclick={handle_save} class="w-full px-4 py-3 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition">
                                {"Sauvegarder la Configuration"}
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
