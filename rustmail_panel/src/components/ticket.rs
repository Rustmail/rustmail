use crate::components::forbidden::Forbidden403;
use crate::types::PanelPermission;
use crate::utils::markdown::markdown_to_html_safe;
use gloo_net::http::Request;
use i18nrs::yew::use_translation;
use js_sys::Date;
use serde::Deserialize;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::UrlSearchParams;
use yew::prelude::*;
use yew_router::prelude::*;

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

impl ThreadMessage {
    fn message_type(&self) -> MessageType {
        if self.user_name.starts_with("System") || self.user_name == "System" {
            MessageType::System
        } else if self.user_id == 0 || self.is_anonymous {
            MessageType::User
        } else {
            MessageType::Staff
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MessageType {
    User,
    Staff,
    System,
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

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PaginatedThreadsResponse {
    pub threads: Vec<CompleteThread>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

impl CompleteThread {
    fn last_message_time(&self) -> String {
        self.messages
            .last()
            .map(|m| m.created_at.clone())
            .unwrap_or_default()
    }
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
    let (i18n, _set_language) = use_translation();

    let tickets = use_state(|| Vec::<CompleteThread>::new());
    let loading = use_state(|| true);
    let selected_category = use_state(|| "all".to_string());
    let search_query = use_state(|| String::new());
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();

    let current_page = use_state(|| 1i64);
    let page_size = use_state(|| 50i64);
    let total_pages = use_state(|| 1i64);
    let total_tickets = use_state(|| 0i64);

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
        if !perms.contains(&PanelPermission::ManageTickets) {
            return html! {
                <Forbidden403 required_permission={i18n.t("navbar.tickets")} />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
            </div>
        };
    }

    {
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let query_string = location.query_str().to_string();

        use_effect_with(query_string.clone(), move |query_str| {
            let url_params = {
                if let Some(query) = query_str.strip_prefix('?') {
                    UrlSearchParams::new_with_str(query).ok()
                } else {
                    None
                }
            };

            let page_from_url = url_params
                .as_ref()
                .and_then(|p| p.get("page"))
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(1);

            let page_size_from_url = url_params
                .as_ref()
                .and_then(|p| p.get("page_size"))
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(50);

            if *current_page != page_from_url {
                current_page.set(page_from_url);
            }
            if *page_size != page_size_from_url {
                page_size.set(page_size_from_url);
            }

            || ()
        });
    }

    let update_url = {
        let navigator = navigator.clone();
        let current_page = current_page.clone();
        let page_size = page_size.clone();

        Callback::from(move |_| {
            let url = format!(
                "/panel/tickets?page={}&page_size={}",
                *current_page, *page_size
            );
            navigator.replace(&TicketsRoute::TicketsList);
            if let Some(window) = web_sys::window() {
                if let Some(history) = window.history().ok() {
                    let _ = history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
                }
            }
        })
    };

    {
        let tickets = tickets.clone();
        let loading = loading.clone();
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let total_pages = total_pages.clone();
        let total_tickets = total_tickets.clone();
        let selected_category = selected_category.clone();
        let update_url = update_url.clone();

        use_effect_with(
            (*current_page, *page_size, (*selected_category).clone()),
            move |_| {
                let tickets_clone = tickets.clone();
                let loading_clone = loading.clone();
                let total_pages_clone = total_pages.clone();
                let total_tickets_clone = total_tickets.clone();
                let page = *current_page;
                let size = *page_size;
                let category = (*selected_category).clone();

                update_url.emit(());

                spawn_local(async move {
                    loading_clone.set(true);

                    let mut url = format!("/api/bot/tickets?page={}&page_size={}", page, size);
                    if category != "all" {
                        url.push_str(&format!("&category_id={}", urlencoding::encode(&category)));
                    }

                    if let Ok(resp) = Request::get(&url).send().await {
                        if let Ok(data) = resp.json::<PaginatedThreadsResponse>().await {
                            tickets_clone.set(data.threads);
                            total_pages_clone.set(data.total_pages);
                            total_tickets_clone.set(data.total);
                        } else {
                            tickets_clone.set(Vec::new());
                        }
                    } else {
                        tickets_clone.set(Vec::new());
                    }
                    loading_clone.set(false);
                });
                || ()
            },
        );
    }

    let filtered_tickets: Vec<CompleteThread> = tickets
        .iter()
        .filter(|t| {
            if search_query.is_empty() {
                true
            } else {
                let query = search_query.to_lowercase();
                t.id.to_lowercase().contains(&query)
                    || t.user_name.to_lowercase().contains(&query)
                    || t.messages
                        .iter()
                        .any(|m| m.content.to_lowercase().contains(&query))
            }
        })
        .cloned()
        .collect();

    let format_date = |timestamp: i64| -> String {
        let d = Date::new_0();
        d.set_time(timestamp as f64 * 1000.0);
        d.to_locale_string("fr-FR", &JsValue::UNDEFINED)
            .as_string()
            .unwrap_or_else(|| i18n.t("panel.tickets.unknown_date").into())
    };

    html! {
        <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8 text-white">
            <div class="mb-8">
                <h1 class="text-3xl text-white mb-2 font-bold">{i18n.t("panel.tickets.title")}</h1>
                <p class="text-gray-400">{i18n.t("panel.tickets.description")}</p>
            </div>

            <div class="bg-slate-800/50 border border-slate-700 rounded-lg overflow-hidden">
                <div class="p-4 border-b border-slate-700 space-y-4">
                    <div>
                        <label class="block text-sm text-gray-300 mb-2">
                            <i class="bi bi-search mr-2"></i>
                            {i18n.t("panel.tickets.search")}
                        </label>
                        <input
                            type="text"
                            value={(*search_query).clone()}
                            oninput={{
                                let search_query = search_query.clone();
                                move |e: InputEvent| {
                                    let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                    if let Some(input) = input {
                                        search_query.set(input.value());
                                    }
                                }
                            }}
                            placeholder={i18n.t("panel.tickets.search_placeholder")}
                            class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>

                    <div>
                        <label class="block text-sm text-gray-300 mb-2">
                            <i class="bi bi-sliders mr-2"></i>
                            {i18n.t("panel.tickets.tickets_per_page")}
                        </label>
                        <select
                            value={(*page_size).to_string()}
                            onchange={{
                                let page_size = page_size.clone();
                                let current_page = current_page.clone();
                                move |e: Event| {
                                    let value = e.target_unchecked_into::<web_sys::HtmlSelectElement>().value();
                                    if let Ok(size) = value.parse::<i64>() {
                                        page_size.set(size);
                                        current_page.set(1);
                                    }
                                }
                            }}
                            class="w-full px-3 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        >
                            <option value="25">{"25"}</option>
                            <option value="50">{"50"}</option>
                            <option value="100">{"100"}</option>
                            <option value="200">{"200"}</option>
                        </select>
                    </div>

                    <div class="flex items-center justify-between text-sm">
                        <div class="flex items-center gap-2 text-gray-400">
                            <i class="bi bi-info-circle"></i>
                            <span>
                                {format!("{} total {} | Page {} / {}",
                                    *total_tickets,
                                    if *total_tickets <= 1 { i18n.t("panel.tickets.ticket_singular") } else { i18n.t("panel.tickets.ticket_plural") },
                                    *current_page,
                                    *total_pages
                                )}
                            </span>
                        </div>
                        <div class="flex items-center gap-2">
                            <button
                                disabled={*current_page <= 1}
                                onclick={{
                                    let current_page = current_page.clone();
                                    move |_| {
                                        let page = *current_page;
                                        if page > 1 {
                                            current_page.set(page - 1);
                                        }
                                    }
                                }}
                                class="px-3 py-1 bg-slate-700 text-white rounded disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-600 transition"
                            >
                                <i class="bi bi-chevron-left"></i>
                            </button>
                            <span class="text-gray-300">
                                {format!("{} / {}", *current_page, *total_pages)}
                            </span>
                            <button
                                disabled={*current_page >= *total_pages}
                                onclick={{
                                    let current_page = current_page.clone();
                                    let total_pages_val = *total_pages;
                                    move |_| {
                                        let page = *current_page;
                                        if page < total_pages_val {
                                            current_page.set(page + 1);
                                        }
                                    }
                                }}
                                class="px-3 py-1 bg-slate-700 text-white rounded disabled:opacity-50 disabled:cursor-not-allowed hover:bg-slate-600 transition"
                            >
                                <i class="bi bi-chevron-right"></i>
                            </button>
                        </div>
                    </div>
                </div>

                {
                    if *loading {
                        html! {
                            <div class="p-8 text-center">
                                <div class="inline-block animate-spin rounded-full h-8 w-8 border-4 border-blue-500 border-t-transparent"></div>
                                <p class="mt-4 text-gray-400">{ i18n.t("panel.tickets.loading") }</p>
                            </div>
                        }
                    } else if filtered_tickets.is_empty() {
                        html! {
                            <div class="p-8 text-center text-gray-400">
                                <i class="bi bi-inbox text-4xl mb-2"></i>
                                <p>{ i18n.t("panel.tickets.none") }</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="divide-y divide-slate-700">
                                { for filtered_tickets.iter().map(|ticket| {
                                    let id = ticket.id.clone();
                                    let message_count = ticket.messages.len();
                                    html! {
                                        <button
                                            class="w-full p-4 text-left hover:bg-slate-700/30 transition-colors"
                                            onclick={{
                                                let navigator = navigator.clone();
                                                move |_| navigator.push(&TicketsRoute::TicketDetails { id: id.clone() })
                                            }}
                                        >
                                            <div class="flex-1 min-w-0">
                                                <div class="flex items-center justify-between mb-2">
                                                    <h3 class="text-white font-medium">{ format!("Ticket #{}", &ticket.id) }</h3>
                                                    <span class="text-xs text-gray-500">
                                                        { format_date(ticket.created_at) }
                                                    </span>
                                                </div>

                                                    <div class="flex items-center gap-2 mb-2">
                                                        <i class="bi bi-person text-gray-400 text-sm"></i>
                                                        <span class="text-sm text-gray-300">{ &ticket.user_name }</span>
                                                        {
                                                            if let Some(cat) = &ticket.category_name {
                                                                html! {
                                                                    <>
                                                                        <span class="text-gray-600">{"•"}</span>
                                                                        <i class="bi bi-folder text-gray-400 text-sm"></i>
                                                                        <span class="text-sm text-gray-300">{ cat }</span>
                                                                    </>
                                                                }
                                                            } else {
                                                                html! {}
                                                            }
                                                        }
                                                    </div>

                                                    <div class="flex items-center gap-4 text-xs text-gray-500">
                                                        <span class="flex items-center gap-1">
                                                            <i class="bi bi-chat-dots"></i>
                                                            {format!("{} {}", message_count,
                                                                if message_count <= 1 { i18n.t("panel.tickets.message_singular") }
                                                                else { i18n.t("panel.tickets.message_plural") }
                                                            )}
                                                        </span>
                                                        {
                                                            if let Some(closed) = ticket.closed_at {
                                                                html! {
                                                                    <span class="flex items-center gap-1">
                                                                        <i class="bi bi-lock"></i>
                                                                        {format!("{} {}", i18n.t("panel.tickets.closed_at"), format_date(closed))}
                                                                    </span>
                                                                }
                                                            } else {
                                                                html! {
                                                                    <span class="flex items-center gap-1">
                                                                        <i class="bi bi-clock-history"></i>
                                                                        {format!("{} {}", i18n.t("panel.tickets.last_message"), &ticket.last_message_time())}
                                                                    </span>
                                                                }
                                                            }
                                                        }
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
    let (i18n, _set_language) = use_translation();

    let ticket = use_state(|| None::<CompleteThread>);
    let loading = use_state(|| true);
    let show_user = use_state(|| true);
    let show_staff = use_state(|| true);
    let show_system = use_state(|| true);
    let search_query = use_state(|| String::new());

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
        if !perms.contains(&PanelPermission::ManageTickets) {
            return html! {
                <Forbidden403 required_permission={i18n.t("navbar.tickets")} />
            };
        }
    } else {
        return html! {
            <div class="flex items-center justify-center min-h-[70vh]">
                <div class="text-gray-400 animate-pulse">{i18n.t("panel.forbidden.checking_permissions")}</div>
            </div>
        };
    }

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
            .unwrap_or_else(|| i18n.t("panel.tickets.unknown_date").into())
    };

    html! {
        <div>
            <button
                onclick={{
                    let on_back = props.on_back.clone();
                    move |_| on_back.emit(())
                }}
                class="flex items-center gap-2 text-blue-400 hover:text-blue-300 transition mb-6"
            >
                <i class="bi bi-arrow-left"></i>
                {i18n.t("panel.tickets.back_to_tickets")}
            </button>

            {
                if *loading {
                    html! {
                        <div class="text-center py-12">
                            <div class="inline-block animate-spin rounded-full h-12 w-12 border-4 border-blue-500 border-t-transparent"></div>
                            <p class="mt-4 text-gray-400">{i18n.t("panel.tickets.loading_ticket")}</p>
                        </div>
                    }
                } else if let Some(ticket) = &*ticket {
                    let filtered_messages: Vec<&ThreadMessage> = ticket.messages.iter()
                        .filter(|m| {
                            let type_match = match m.message_type() {
                                MessageType::User => *show_user,
                                MessageType::Staff => *show_staff,
                                MessageType::System => *show_system,
                            };

                            let search_match = if search_query.is_empty() {
                                true
                            } else {
                                let query = search_query.to_lowercase();
                                m.content.to_lowercase().contains(&query)
                                    || m.user_name.to_lowercase().contains(&query)
                            };

                            type_match && search_match
                        })
                        .collect();

                    html! {
                        <div>
                            <div class="bg-gradient-to-r from-slate-800/80 to-slate-900/80 border border-slate-700 rounded-lg p-6 mb-6">
                                <div class="flex items-start justify-between mb-4">
                                    <div>
                                        <h1 class="text-3xl font-bold text-white mb-2">
                                            { format!("Ticket #{}", ticket.id) }
                                        </h1>
                                        {
                                            if let Some(cat) = &ticket.category_name {
                                                html! {
                                                    <span class="px-3 py-1 text-sm bg-blue-500/20 text-blue-400 border border-blue-500/30 rounded-full">
                                                        <i class="bi bi-folder mr-1"></i>
                                                        { cat }
                                                    </span>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
                                    <div class="flex items-center gap-2 text-gray-300">
                                        <i class="bi bi-person-circle text-blue-400"></i>
                                        <span class="text-gray-400">{i18n.t("panel.tickets.user")}{" :"}</span>
                                        <span class="font-medium">{ &ticket.user_name }</span>
                                    </div>
                                    <div class="flex items-center gap-2 text-gray-300">
                                        <i class="bi bi-calendar-event text-blue-400"></i>
                                        <span class="text-gray-400">{i18n.t("panel.tickets.opened")}{" :"}</span>
                                        <span class="font-medium">{ format_date(ticket.created_at) }</span>
                                    </div>
                                    {
                                        if let Some(closed) = ticket.closed_at {
                                            html! {
                                                <div class="flex items-center gap-2 text-gray-300">
                                                    <i class="bi bi-calendar-x text-blue-400"></i>
                                                    <span class="text-gray-400">{i18n.t("panel.tickets.closed")}{" :"}</span>
                                                    <span class="font-medium">{ format_date(closed) }</span>
                                                </div>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </div>
                            </div>

                            <div class="bg-slate-800/50 border border-slate-700 rounded-lg p-4 mb-4">
                                <div class="flex items-center justify-between mb-3">
                                    <h2 class="text-lg font-semibold text-white flex items-center gap-2">
                                        <i class="bi bi-funnel"></i>
                                        {i18n.t("panel.tickets.filters")}
                                    </h2>
                                    <span class="text-sm text-gray-400">
                                        {format!("{} / {} {}", filtered_messages.len(), ticket.messages.len(),
                                            i18n.t("panel.tickets.message_plural")
                                        )}
                                    </span>
                                </div>

                                <div class="mb-3">
                                    <input
                                        type="text"
                                        value={(*search_query).clone()}
                                        oninput={{
                                            let search_query = search_query.clone();
                                            move |e: InputEvent| {
                                                let input = e.target_dyn_into::<web_sys::HtmlInputElement>();
                                                if let Some(input) = input {
                                                    search_query.set(input.value());
                                                }
                                            }
                                        }}
                                        placeholder={i18n.t("panel.tickets.search_placeholder")}
                                        class="w-full px-4 py-2 bg-slate-900/50 border border-slate-600 rounded-md text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    />
                                </div>

                                <div class="flex flex-wrap gap-2">
                                    <button
                                        onclick={{
                                            let show_user = show_user.clone();
                                            move |_| show_user.set(!*show_user)
                                        }}
                                        class={classes!(
                                            "px-4", "py-2", "rounded-lg", "border", "transition-all", "text-sm", "font-medium",
                                            "flex", "items-center", "gap-2",
                                            if *show_user {
                                                "bg-blue-500/20 text-blue-400 border-blue-500/50"
                                            } else {
                                                "bg-slate-700/30 text-gray-400 border-slate-600"
                                            }
                                        )}
                                    >
                                        <i class="bi bi-person"></i>
                                        {i18n.t("panel.tickets.filter_user")}
                                    </button>

                                    <button
                                        onclick={{
                                            let show_staff = show_staff.clone();
                                            move |_| show_staff.set(!*show_staff)
                                        }}
                                        class={classes!(
                                            "px-4", "py-2", "rounded-lg", "border", "transition-all", "text-sm", "font-medium",
                                            "flex", "items-center", "gap-2",
                                            if *show_staff {
                                                "bg-green-500/20 text-green-400 border-green-500/50"
                                            } else {
                                                "bg-slate-700/30 text-gray-400 border-slate-600"
                                            }
                                        )}
                                    >
                                        <i class="bi bi-shield-check"></i>
                                        {i18n.t("panel.tickets.filter_staff")}
                                    </button>

                                    <button
                                        onclick={{
                                            let show_system = show_system.clone();
                                            move |_| show_system.set(!*show_system)
                                        }}
                                        class={classes!(
                                            "px-4", "py-2", "rounded-lg", "border", "transition-all", "text-sm", "font-medium",
                                            "flex", "items-center", "gap-2",
                                            if *show_system {
                                                "bg-yellow-500/20 text-yellow-400 border-yellow-500/50"
                                            } else {
                                                "bg-slate-700/30 text-gray-400 border-slate-600"
                                            }
                                        )}
                                    >
                                        <i class="bi bi-gear"></i>
                                        {i18n.t("panel.tickets.filter_system")}
                                    </button>
                                </div>
                            </div>

                            <div class="bg-slate-800/50 border border-slate-700 rounded-lg p-6 max-h-[calc(100vh-450px)] overflow-y-auto">
                                {
                                    if filtered_messages.is_empty() {
                                        html! {
                                            <div class="text-center py-8 text-gray-400">
                                                <i class="bi bi-chat-dots text-4xl mb-2"></i>
                                                <p>{i18n.t("panel.tickets.no_messages")}</p>
                                            </div>
                                        }
                                    } else {
                                        html! {
                                            <div class="space-y-4">
                                                { for filtered_messages.iter().map(|m| {
                                                    let msg_type = m.message_type();
                                                    let (bg_class, border_class, text_class) = match msg_type {
                                                        MessageType::User => (
                                                            "bg-blue-500/10",
                                                            "border-blue-500/30",
                                                            "text-blue-400"
                                                        ),
                                                        MessageType::Staff => (
                                                            "bg-green-500/10",
                                                            "border-green-500/30",
                                                            "text-green-400"
                                                        ),
                                                        MessageType::System => (
                                                            "bg-yellow-500/10",
                                                            "border-yellow-500/30",
                                                            "text-yellow-400"
                                                        ),
                                                    };

                                                    html! {
                                                        <div class={classes!("border", "rounded-lg", "p-4", bg_class, border_class)}>
                                                            <div class="flex items-baseline gap-2 mb-2">
                                                                <span class={classes!("font-medium", "text-sm", text_class)}>
                                                                    { &m.user_name }
                                                                </span>
                                                                <span class="text-xs text-gray-500">
                                                                    { &m.created_at }
                                                                </span>
                                                            </div>
                                                            <div class="prose prose-sm prose-invert max-w-none">
                                                                <div class="text-gray-200 break-words">
                                                                    { markdown_to_html_safe(&m.content) }
                                                                </div>
                                                            </div>
                                                        </div>
                                                    }
                                                }) }
                                            </div>
                                        }
                                    }
                                }
                            </div>
                        </div>
                    }
                } else {
                    html! {
                        <div class="text-center py-12">
                            <i class="bi bi-exclamation-triangle text-4xl text-gray-400 mb-4"></i>
                            <p class="text-gray-400">{i18n.t("panel.tickets.ticket_not_found")}</p>
                        </div>
                    }
                }
            }
        </div>
    }
}
