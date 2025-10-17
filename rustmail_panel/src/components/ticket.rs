use gloo_net::http::Request;
use js_sys::Date;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Clone, PartialEq, Deserialize)]
pub struct Author {
    pub name: String,
    pub avatar: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Message {
    pub id: String,
    pub author: Author,
    pub content: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub author: Author,
    pub category: String,
    pub status: String,
    pub created_at: String,
    pub closed_at: Option<String>,
    pub messages: Vec<Message>,
}

#[function_component(TicketsPage)]
pub fn tickets_page() -> Html {
    let tickets = use_state(|| Vec::<Ticket>::new());
    let selected_ticket = use_state(|| None::<Ticket>);
    let loading = use_state(|| true);
    let selected_category = use_state(|| "all".to_string());
    let show_user_messages = use_state(|| true);
    let show_staff_messages = use_state(|| true);
    let show_system_messages = use_state(|| true);

    {
        let tickets = tickets.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let tickets_clone = tickets.clone();
            let loading_clone = loading.clone();

            spawn_local(async move {
                loading_clone.set(true);

                if let Ok(resp) = Request::get("/api/bot/tickets").send().await {
                    if let Ok(data) = resp.json::<Vec<Ticket>>().await {
                        tickets_clone.set(data);
                    } else {
                        tickets_clone.set(Vec::new());
                    }
                } else {
                    tickets_clone.set(Vec::new());
                }

                loading_clone.set(false);
            });

            || ()
        });
    }

    let categories = {
        let mut cats: Vec<String> = tickets.clone()
            .iter()
            .map(|t| t.category.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        cats.sort();
        cats.insert(0, "all".into());
        cats
    };

    let filtered_tickets: Vec<Ticket> = {
        if *selected_category == "all" {
            (*tickets).clone()
        } else {
            tickets.iter()
                .filter(|t| t.category == *selected_category)
                .cloned()
                .collect()
        }
    };

    let get_filtered_messages = {
        let show_user_messages = show_user_messages.clone();
        let show_staff_messages = show_staff_messages.clone();
        let show_system_messages = show_system_messages.clone();

        move |ticket: &Ticket| -> Vec<Message> {
            ticket.messages.iter().filter(|m| {
                match m.message_type.as_str() {
                    "user" if !*show_user_messages => false,
                    "staff" if !*show_staff_messages => false,
                    "system" if !*show_system_messages => false,
                    _ => true,
                }
            }).cloned().collect()
        }
    };

    let format_date = |date: &str| -> String {
        let d = Date::new(&JsValue::from_str(date));
        let s = d.to_locale_string("fr-FR", &JsValue::UNDEFINED);
        s.as_string().unwrap_or_else(|| date.to_string())
    };

    let status_color = |status: &str| match status {
        "open" => "bg-green-500",
        "closed" => "bg-gray-500",
        "pending" => "bg-yellow-500",
        _ => "bg-gray-500",
    };

    html! {
        <div class="min-h-screen px-4 sm:px-6 lg:px-8 py-8 text-white">
            <div class="max-w-7xl mx-auto">

                <div class="mb-8">
                    <h1 class="text-3xl text-white mb-2">{"Tickets Fermés"}</h1>
                    <p class="text-gray-400">{"Consultez l'historique des tickets fermés"}</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">

                    <div class="lg:col-span-1">
                        <div class="bg-slate-800/50 border border-slate-700 rounded-lg overflow-hidden">

                            <div class="p-4 border-b border-slate-700">
                                <label class="block text-sm text-gray-300 mb-2">{"Filtrer par catégorie"}</label>
                                <select
                                    value={(*selected_category).clone()}
                                    onchange={{
                                        let selected_category = selected_category.clone();
                                        move |e: Event| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                            selected_category.set(value);
                                        }
                                    }}
                                    class="w-full px-3 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                >
                                    { for categories.iter().map(|c| html! {
                                        <option value={c.clone()}>{ if c == "all" { "Toutes les catégories" } else { c } }</option>
                                    }) }
                                </select>
                            </div>

                            <div class="overflow-y-auto max-h-[calc(100vh-300px)]">
                                {
                                    if *loading {
                                        html! {
                                            <div class="p-8 text-center text-gray-400">
                                                <svg class="animate-spin h-8 w-8 mx-auto mb-2" fill="none" viewBox="0 0 24 24">
                                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                                                </svg>
                                                {"Chargement..."}
                                            </div>
                                        }
                                    } else if filtered_tickets.is_empty() {
                                        html! {
                                            <div class="p-8 text-center text-gray-400">{"Aucun ticket trouvé"}</div>
                                        }
                                    } else {
                                        html! {
                                            <div class="divide-y divide-slate-700">
                                                { for filtered_tickets.iter().map(|ticket| {
                                                    let is_selected = selected_ticket.as_ref().map(|t| t.id == ticket.id).unwrap_or(false);
                                                    let ticket_clone = ticket.clone();
                                                    let selected_ticket = selected_ticket.clone();

                                                    html! {
                                                        <button
                                                            class={classes!(
                                                                "w-full", "p-4", "text-left", "hover:bg-slate-700/30", "transition",
                                                                if is_selected { Some("bg-slate-700/50") } else { None }
                                                            )}
                                                            onclick={Callback::from(move |_| selected_ticket.set(Some(ticket_clone.clone())))}
                                                        >
                                                            <div class="flex items-start gap-3">
                                                                <img src={ticket.author.avatar.clone()} class="w-10 h-10 rounded-full" />
                                                                <div class="flex-1 min-w-0">
                                                                    <div class="flex items-center justify-between mb-1">
                                                                        <h3 class="text-white text-sm truncate">{ &ticket.title }</h3>
                                                                        <div class={classes!("w-2","h-2","rounded-full",status_color(&ticket.status))}></div>
                                                                    </div>
                                                                    <p class="text-xs text-gray-400 mb-1">{ &ticket.author.name }</p>
                                                                    <div class="flex items-center gap-2 text-xs text-gray-500">
                                                                        <span>{ &ticket.category }</span>
                                                                        <span>{"•"}</span>
                                                                        <span>{ format_date(&ticket.created_at) }</span>
                                                                    </div>
                                                                </div>
                                                            </div>
                                                        </button>
                                                    }
                                                }) }
                                            </div>
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </div>

                    <div class="lg:col-span-2">
                        <div class="bg-slate-800/50 border border-slate-700 rounded-lg overflow-hidden">
                            {
                                if let Some(ticket) = &*selected_ticket {
                                    let messages = get_filtered_messages(ticket);
                                    html! {
                                        <>
                                            <div class="p-6 border-b border-slate-700">
                                                <div class="flex items-start justify-between mb-4">
                                                    <div>
                                                        <h2 class="text-2xl text-white mb-2">{ &ticket.title }</h2>
                                                        <div class="flex items-center gap-3 text-sm text-gray-400">
                                                            <span>{ &ticket.author.name }</span>
                                                            <span>{"•"}</span>
                                                            <span>{ &ticket.category }</span>
                                                            <span>{"•"}</span>
                                                            <span>{ format!("Ouvert: {}", format_date(&ticket.created_at)) }</span>
                                                            {
                                                                if let Some(closed) = &ticket.closed_at {
                                                                    html! {
                                                                        <>
                                                                            <span>{"•"}</span>
                                                                            <span>{ format!("Fermé: {}", format_date(closed)) }</span>
                                                                        </>
                                                                    }
                                                                } else {
                                                                    html! {}
                                                                }
                                                            }
                                                        </div>
                                                    </div>

                                                    <div class="flex items-center gap-2">
                                                        <div class={classes!("w-3","h-3","rounded-full",status_color(&ticket.status))}></div>
                                                        <span class="text-sm text-gray-300">{ &ticket.status }</span>
                                                    </div>
                                                </div>

                                                <div class="flex flex-wrap gap-2">
                                                    <button onclick={{
                                                        let show_user_messages = show_user_messages.clone();
                                                        move |_| show_user_messages.set(!*show_user_messages)
                                                    }} class={classes!(
                                                        "px-3","py-1.5","text-sm","rounded-md","transition","flex","items-center","gap-2",
                                                        if *show_user_messages { "bg-blue-600 hover:bg-blue-700 text-white" } else { "bg-slate-700 hover:bg-slate-600 text-gray-300" }
                                                    )}>
                                                        <div class="w-2 h-2 rounded-full bg-blue-400"></div>
                                                        {"Messages Utilisateur"}
                                                    </button>

                                                    <button onclick={{
                                                        let show_staff_messages = show_staff_messages.clone();
                                                        move |_| show_staff_messages.set(!*show_staff_messages)
                                                    }} class={classes!(
                                                        "px-3","py-1.5","text-sm","rounded-md","transition","flex","items-center","gap-2",
                                                        if *show_staff_messages { "bg-green-600 hover:bg-green-700 text-white" } else { "bg-slate-700 hover:bg-slate-600 text-gray-300" }
                                                    )}>
                                                        <div class="w-2 h-2 rounded-full bg-green-400"></div>
                                                        {"Messages Staff"}
                                                    </button>

                                                    <button onclick={{
                                                        let show_system_messages = show_system_messages.clone();
                                                        move |_| show_system_messages.set(!*show_system_messages)
                                                    }} class={classes!(
                                                        "px-3","py-1.5","text-sm","rounded-md","transition","flex","items-center","gap-2",
                                                        if *show_system_messages { "bg-gray-600 hover:bg-gray-700 text-white" } else { "bg-slate-700 hover:bg-slate-600 text-gray-300" }
                                                    )}>
                                                        <div class="w-2 h-2 rounded-full bg-gray-400"></div>
                                                        {"Messages Système"}
                                                    </button>
                                                </div>
                                            </div>

                                            <div class="p-6 space-y-4 overflow-y-auto max-h-[calc(100vh-350px)]">
                                                {
                                                    if messages.is_empty() {
                                                        html! {
                                                            <div class="text-center py-8 text-gray-400">
                                                                {"Aucun message à afficher avec les filtres actuels"}
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {
                                                            { for messages.iter().map(|m| html! {
                                                                <div class="flex gap-3">
                                                                    <img src={m.author.avatar.clone()} class="w-10 h-10 rounded-full" />
                                                                    <div class="flex-1">
                                                                        <div class="flex items-baseline gap-2 mb-1">
                                                                            <span class="text-white text-sm">{ &m.author.name }</span>
                                                                            <span class={classes!(
                                                                                "text-xs","px-2","py-0.5","rounded",
                                                                                match m.message_type.as_str() {
                                                                                    "user" => "bg-blue-500/20 text-blue-300",
                                                                                    "staff" => "bg-green-500/20 text-green-300",
                                                                                    _ => "bg-gray-500/20 text-gray-300"
                                                                                }
                                                                            )}>
                                                                                {
                                                                                    match m.message_type.as_str() {
                                                                                        "user" => "Utilisateur",
                                                                                        "staff" => "Staff",
                                                                                        _ => "Système"
                                                                                    }
                                                                                }
                                                                            </span>
                                                                            <span class="text-xs text-gray-500">{ format_date(&m.timestamp) }</span>
                                                                        </div>
                                                                        <div class="bg-slate-900/50 border border-slate-700 rounded-lg p-3">
                                                                            <p class="text-gray-300 text-sm">{ &m.content }</p>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            }) }
                                                        }
                                                    }
                                                }
                                            </div>
                                        </>
                                    }
                                } else {
                                    html! {
                                        <div class="p-12 text-center text-gray-400">
                                            <svg class="w-16 h-16 mx-auto mb-4 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                                            </svg>
                                            {"Sélectionnez un ticket pour voir les détails"}
                                        </div>
                                    }
                                }
                            }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
