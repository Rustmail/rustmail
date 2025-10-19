use gloo_net::http::Request;
use js_sys::Date;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen::JsValue;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ThreadMessage {
    pub id: i64,
    pub thread_id: String,
    pub user_id: i64,
    pub user_name: String,
    pub is_anonymous: bool,
    pub dm_message_id: Option<String>,
    pub inbox_message_id: Option<String>,
    pub message_number: Option<i64>,
    pub created_at: String,
    pub content: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct CompleteThread {
    pub id: String,
    pub user_id: i64,
    pub user_name: String,
    pub channel_id: String,
    pub created_at: i64,
    pub new_message_number: i64,
    pub status: i64,
    pub user_left: bool,
    pub closed_at: Option<i64>,
    pub closed_by: Option<String>,
    pub category_id: Option<String>,
    pub category_name: Option<String>,
    pub required_permissions: Option<String>,
    pub messages: Vec<ThreadMessage>,
}

#[derive(Clone, Routable, PartialEq)]
pub enum TicketsRoute {
    #[at("/panel/tickets")]
    TicketsList,
    #[at("/panel/tickets/:id")]
    TicketDetails { id: String },
}

#[function_component(TicketsPage)]
pub fn tickets_page() -> Html {
    let location = use_location().unwrap();
    let navigator = use_navigator().unwrap();

    let path = location.path();

    if let Some(id) = path.strip_prefix("/panel/tickets/") {
        html! {
            <section class="min-h-screen bg-gradient-to-b from-slate-900 to-black text-white pt-24">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
                    <TicketDetails
                        id={id.to_string()}
                        on_back={Callback::from({
                            let navigator = navigator.clone();
                            move |_| navigator.push(&TicketsRoute::TicketsList)
                        })}
                    />
                </div>
            </section>
        }
    } else {
        html! { <TicketsList /> }
    }
}

#[function_component(TicketsList)]
pub fn tickets_list() -> Html {
    let tickets = use_state(|| Vec::<CompleteThread>::new());
    let loading = use_state(|| true);
    let selected_category = use_state(|| "all".to_string());
    let navigator = use_navigator().unwrap();

    {
        let tickets = tickets.clone();
        let loading = loading.clone();

        use_effect_with((), move |_| {
            let tickets_clone = tickets.clone();
            let loading_clone = loading.clone();

            spawn_local(async move {
                loading_clone.set(true);
                if let Ok(resp) = Request::get("/api/bot/tickets").send().await {
                    if let Ok(data) = resp.json::<Vec<CompleteThread>>().await {
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
        let mut cats: Vec<String> = tickets
            .iter()
            .filter_map(|t| t.category_name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        cats.sort();
        cats.insert(0, "all".into());
        cats
    };

    let filtered_tickets: Vec<CompleteThread> = if *selected_category == "all" {
        (*tickets).clone()
    } else {
        tickets
            .iter()
            .filter(|t| t.category_name.as_deref() == Some(&*selected_category))
            .cloned()
            .collect()
    };

    let format_date = |timestamp: i64| -> String {
        let d = Date::new_0();
        d.set_time(timestamp as f64 * 1000.0);
        d.to_locale_string("fr-FR", &JsValue::UNDEFINED)
            .as_string()
            .unwrap_or_else(|| "date inconnue".into())
    };

    let status_color = |status: i64| match status {
        1 => "bg-green-500",
        0 => "bg-gray-500",
        _ => "bg-yellow-500",
    };

    html! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 text-white">
            <div class="mb-8">
                <h1 class="text-3xl text-white mb-2">{"Tickets Fermés"}</h1>
                <p class="text-gray-400">{"Consultez l'historique des tickets fermés"}</p>
            </div>

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

                {
                    if *loading {
                        html! {
                            <div class="p-8 text-center text-gray-400">
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
                                    let id = ticket.id.clone();
                                    html! {
                                        <button
                                            class="w-full p-4 text-left hover:bg-slate-700/30 transition"
                                            onclick={{
                                                let navigator = navigator.clone();
                                                move |_| navigator.push(&TicketsRoute::TicketDetails { id: id.clone() })
                                            }}
                                        >
                                            <div class="flex items-start gap-3">
                                                <div class="flex-1 min-w-0">
                                                    <div class="flex items-center justify-between mb-1">
                                                        <h3 class="text-white text-sm truncate">{ format!("Ticket #{}", &ticket.id) }</h3>
                                                        <div class={classes!("w-2","h-2","rounded-full",status_color(ticket.status))}></div>
                                                    </div>
                                                    <p class="text-xs text-gray-400 mb-1">{ &ticket.user_name }</p>
                                                    <div class="flex items-center gap-2 text-xs text-gray-500">
                                                        <span>{ ticket.category_name.clone().unwrap_or_else(|| "Inconnue".into()) }</span>
                                                        <span>{"•"}</span>
                                                        <span>{ format_date(ticket.created_at) }</span>
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
    }
}

#[derive(Properties, PartialEq)]
pub struct TicketDetailsProps {
    pub id: String,
    pub on_back: Callback<()>,
}

#[function_component(TicketDetails)]
pub fn ticket_details(props: &TicketDetailsProps) -> Html {
    let ticket = use_state(|| None::<CompleteThread>);
    let loading = use_state(|| true);

    {
        let id = props.id.clone();
        let ticket = ticket.clone();
        let loading = loading.clone();

        use_effect_with(id.clone(), move |_| {
            spawn_local(async move {
                loading.set(true);
                let url = format!("/api/bot/tickets?id={}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(data) = resp.json::<CompleteThread>().await {
                        ticket.set(Some(data));
                    }
                }
                loading.set(false);
            });
            || ()
        });
    }

    let format_date = |timestamp: i64| -> String {
        let d = Date::new_0();
        d.set_time(timestamp as f64 * 1000.0);
        d.to_locale_string("fr-FR", &JsValue::UNDEFINED)
            .as_string()
            .unwrap_or_else(|| "date inconnue".into())
    };

    html! {
        <div class="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-8 text-white">
            <button
                onclick={{
                    let on_back = props.on_back.clone();
                    move |_| on_back.emit(())
                }}
                class="text-blue-400 hover:underline mb-6"
            >
                {"← Retour à la liste"}
            </button>

            {
                if *loading {
                    html! { <p class="text-gray-400">{"Chargement du ticket..."}</p> }
                } else if let Some(ticket) = &*ticket {
                    html! {
                        <div>
                            <h2 class="text-2xl text-white mb-2">{ format!("Ticket #{}", ticket.id) }</h2>
                            <p class="text-gray-400 text-sm mb-4">
                                { format!("Utilisateur : {} • Catégorie : {} • Ouvert : {}", ticket.user_name, ticket.category_name.clone().unwrap_or_default(), format_date(ticket.created_at)) }
                                {
                                    if let Some(closed) = ticket.closed_at {
                                        html! {
                                            <span>{ format!(" • Fermé : {}", format_date(closed)) }</span>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </p>

                            <div class="bg-slate-800/50 border border-slate-700 rounded-lg p-4 space-y-4 max-h-[calc(100vh-350px)] overflow-y-auto">
                                { for ticket.messages.iter().map(|m| html! {
                                    <div class="flex flex-col gap-1 border-b border-slate-700 pb-3">
                                        <div class="flex items-baseline gap-2">
                                            <span class="text-white text-sm">{ &m.user_name }</span>
                                            <span class="text-xs text-gray-500">{ &m.created_at }</span>
                                        </div>
                                        <p class="text-gray-300 text-sm">{ &m.content }</p>
                                    </div>
                                }) }
                            </div>
                        </div>
                    }
                } else {
                    html! { <p class="text-gray-400">{"Ticket introuvable."}</p> }
                }
            }
        </div>
    }
}
