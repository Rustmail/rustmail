use crate::components::forbidden::{CheckingPermissions, Forbidden403};
use crate::components::ui::{
    Avatar, Badge, Button, ButtonSize, ButtonVariant, EmptyState, Icon, IconName, Loading, Notice,
    Select, StatusDot, TicketStub, Tone, button_classes, input_value,
};
use crate::hooks::use_user_permissions;
use crate::i18n::yew::use_translation;
use crate::pages::panel::PanelRoute;
use crate::types::PanelPermission;
use crate::utils::format::group_digits;
use crate::utils::markdown::markdown_to_html_safe;
use chrono::{DateTime, NaiveDateTime};
use gloo_net::http::Request;
use js_sys::{Date, Object, Reflect};
use serde::{Deserialize, Serialize};
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
    pub is_internal: bool,
}

impl ThreadMessage {
    fn message_type(&self) -> MessageType {
        if self.is_internal {
            MessageType::Internal
        } else if self.message_number.is_some() {
            MessageType::Staff
        } else if self.dm_message_id.is_some() && self.inbox_message_id.is_some() {
            MessageType::User
        } else {
            MessageType::System
        }
    }

    fn timestamp_ms(&self) -> Option<f64> {
        NaiveDateTime::parse_from_str(&self.created_at, "%Y-%m-%d %H:%M:%S")
            .map(|d| d.and_utc().timestamp_millis() as f64)
            .ok()
            .or_else(|| {
                DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.timestamp_millis() as f64)
                    .ok()
            })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MessageType {
    User,
    Staff,
    System,
    Internal,
}

impl MessageType {
    const ALL: [MessageType; 4] = [
        MessageType::User,
        MessageType::Staff,
        MessageType::System,
        MessageType::Internal,
    ];

    fn label_key(self) -> &'static str {
        match self {
            MessageType::User => "panel.tickets.filter_user",
            MessageType::Staff => "panel.tickets.filter_staff",
            MessageType::System => "panel.tickets.filter_system",
            MessageType::Internal => "panel.tickets.filter_internal",
        }
    }

    fn dot_class(self) -> &'static str {
        match self {
            MessageType::User => "bg-blurple",
            MessageType::Staff => "bg-accent",
            MessageType::System => "bg-fg-muted",
            MessageType::Internal => "bg-warning",
        }
    }

    fn transcript_prefix(self) -> &'static str {
        match self {
            MessageType::User => "[User]",
            MessageType::Staff => "[Staff]",
            MessageType::System => "[System]",
            MessageType::Internal => "[Internal]",
        }
    }
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

impl CompleteThread {
    fn is_open(&self) -> bool {
        self.status == 1
    }

    fn short_id(&self) -> String {
        self.id.split('-').next().unwrap_or(&self.id).to_string()
    }

    fn transcript_data_url(&self) -> String {
        let mut content = format!(
            "Ticket #{}\nUser: {}\nCategory: {}\n\n",
            self.id,
            self.user_name,
            self.category_name.as_deref().unwrap_or("Unknown")
        );
        for msg in &self.messages {
            content.push_str(&format!(
                "{} {} ({}):\n{}\n\n",
                msg.message_type().transcript_prefix(),
                msg.user_name,
                msg.created_at,
                msg.content
            ));
        }
        format!(
            "data:text/plain;charset=utf-8,{}",
            urlencoding::encode(&content)
        )
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PaginatedThreadsResponse {
    pub threads: Vec<CompleteThread>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
    pub total_pages: i64,
}

#[derive(Serialize)]
struct PageQuery {
    page: i64,
    page_size: i64,
}

const PAGE_SIZES: [i64; 4] = [25, 50, 100, 200];
const DEFAULT_PAGE_SIZE: i64 = 50;
const COLLAPSE_WINDOW_MS: f64 = 5.0 * 60.0 * 1000.0;

const LIST_COLUMNS: &str = "grid grid-cols-[112px_minmax(0,1.4fr)_minmax(0,1fr)_88px_168px_minmax(0,1.5fr)] items-center gap-4 px-5";

const MARKDOWN: &str = "min-w-0 break-words text-sm text-fg [&_p+p]:mt-2 [&_strong]:font-medium [&_a]:text-accent-fg [&_a]:underline [&_a]:underline-offset-2 [&_code]:rounded [&_code]:bg-surface-raised [&_code]:px-1 [&_code]:font-mono [&_code]:text-[13px] [&_pre]:mt-2 [&_pre]:overflow-x-auto [&_pre]:rounded-lg [&_pre]:border [&_pre]:border-line [&_pre]:bg-surface-sunken [&_pre]:p-3 [&_pre_code]:bg-transparent [&_pre_code]:p-0 [&_ul]:list-disc [&_ul]:pl-5 [&_ol]:list-decimal [&_ol]:pl-5 [&_blockquote]:border-l-2 [&_blockquote]:border-line-strong [&_blockquote]:pl-3 [&_blockquote]:text-fg-secondary [&_h1]:font-medium [&_h2]:font-medium [&_h3]:font-medium";

struct DateFormatter {
    locale: &'static str,
}

impl DateFormatter {
    fn new(language: &str) -> Self {
        let locale = match language {
            "fr" => "fr-FR",
            _ => "en-GB",
        };
        Self { locale }
    }

    fn format(&self, ms: f64, fields: &[(&str, &str)]) -> String {
        let options = Object::new();
        for (key, value) in fields {
            let _ = Reflect::set(&options, &JsValue::from_str(key), &JsValue::from_str(value));
        }
        let date = Date::new(&JsValue::from_f64(ms));
        date.to_locale_string(self.locale, &options).into()
    }

    fn date_time(&self, ms: f64) -> String {
        self.format(
            ms,
            &[
                ("day", "numeric"),
                ("month", "short"),
                ("year", "numeric"),
                ("hour", "2-digit"),
                ("minute", "2-digit"),
            ],
        )
    }

    fn date(&self, ms: f64) -> String {
        self.format(
            ms,
            &[("day", "numeric"), ("month", "short"), ("year", "numeric")],
        )
    }

    fn long_date(&self, ms: f64) -> String {
        self.format(
            ms,
            &[("day", "numeric"), ("month", "long"), ("year", "numeric")],
        )
    }

    fn time(&self, ms: f64) -> String {
        self.format(ms, &[("hour", "2-digit"), ("minute", "2-digit")])
    }

    fn seconds_date_time(&self, seconds: i64) -> String {
        self.date_time(seconds as f64 * 1000.0)
    }

    fn message_time(&self, message: &ThreadMessage) -> String {
        message
            .timestamp_ms()
            .map(|ms| self.time(ms))
            .unwrap_or_else(|| message.created_at.clone())
    }
}

fn read_page_query(query_str: &str) -> (i64, i64) {
    let params =
        UrlSearchParams::new_with_str(query_str.strip_prefix('?').unwrap_or(query_str)).ok();
    let read = |name: &str| {
        params
            .as_ref()
            .and_then(|p| p.get(name))
            .and_then(|s| s.parse::<i64>().ok())
            .filter(|v| *v > 0)
    };
    (
        read("page").unwrap_or(1),
        read("page_size").unwrap_or(DEFAULT_PAGE_SIZE),
    )
}

#[derive(Clone, PartialEq)]
enum LoadState<T> {
    Loading,
    Loaded(T),
    Failed,
}

#[function_component(TicketsList)]
pub fn tickets_list() -> Html {
    let (i18n, _set_language) = use_translation();
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ManageTickets) => {
            html! { <Forbidden403 required_permission={i18n.t("navbar.tickets")} /> }
        }
        Some(_) => html! { <TicketsListContent /> },
    }
}

#[function_component(TicketsListContent)]
fn tickets_list_content() -> Html {
    let (i18n, _set_language) = use_translation();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let (page, page_size) = read_page_query(location.query_str());

    let response = use_state(|| LoadState::<PaginatedThreadsResponse>::Loading);
    let search_query = use_state(String::new);

    {
        let response = response.clone();
        use_effect_with((page, page_size), move |&(page, page_size)| {
            response.set(LoadState::Loading);
            spawn_local(async move {
                let url = format!("/api/bot/tickets?page={}&page_size={}", page, page_size);
                let loaded = match Request::get(&url).send().await {
                    Ok(resp) => resp.json::<PaginatedThreadsResponse>().await.ok(),
                    Err(_) => None,
                };
                response.set(loaded.map(LoadState::Loaded).unwrap_or(LoadState::Failed));
            });
            || ()
        });
    }

    let go_to = {
        let navigator = navigator.clone();
        Callback::from(move |(page, page_size): (i64, i64)| {
            let _ = navigator
                .replace_with_query(&PanelRoute::TicketsList, &PageQuery { page, page_size });
        })
    };

    let total = match &*response {
        LoadState::Loaded(data) => data.total,
        _ => 0,
    };
    let count_label = format!(
        "{} {}",
        group_digits(total),
        if total == 1 {
            i18n.t("panel.tickets.ticket_singular")
        } else {
            i18n.t("panel.tickets.ticket_plural")
        }
    );

    let page_size_options: Vec<(AttrValue, AttrValue)> = PAGE_SIZES
        .iter()
        .map(|size| {
            (
                AttrValue::from(size.to_string()),
                AttrValue::from(
                    i18n.t("panel.tickets.per_page")
                        .replace("{count}", &size.to_string()),
                ),
            )
        })
        .collect();

    let on_page_size = {
        let go_to = go_to.clone();
        Callback::from(move |value: String| {
            if let Ok(size) = value.parse::<i64>() {
                go_to.emit((1, size));
            }
        })
    };

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| search_query.set(input_value(&e)))
    };

    let body = match &*response {
        LoadState::Loading => html! {
            <div class="card"><Loading label={i18n.t("panel.tickets.loading")} /></div>
        },
        LoadState::Failed => html! {
            <Notice>{i18n.t("panel.tickets.load_error")}</Notice>
        },
        LoadState::Loaded(data) if data.threads.is_empty() => html! {
            <div class="card">
                <EmptyState
                    icon={IconName::Inbox}
                    title={i18n.t("panel.tickets.none")}
                    description={i18n.t("panel.tickets.none_description")}
                />
            </div>
        },
        LoadState::Loaded(data) => {
            let query = search_query.to_lowercase();
            let filtered: Vec<&CompleteThread> = data
                .threads
                .iter()
                .filter(|t| {
                    query.is_empty()
                        || t.id.to_lowercase().contains(&query)
                        || t.user_name.to_lowercase().contains(&query)
                        || t.messages
                            .iter()
                            .any(|m| m.content.to_lowercase().contains(&query))
                })
                .collect();

            let first = (data.page - 1) * data.page_size + 1;
            let last = first + data.threads.len() as i64 - 1;
            let range = i18n
                .t("panel.tickets.range")
                .replace("{start}", &group_digits(first))
                .replace("{end}", &group_digits(last))
                .replace("{total}", &group_digits(data.total));
            let page_of = i18n
                .t("panel.tickets.page_of")
                .replace("{page}", &data.page.to_string())
                .replace("{pages}", &data.total_pages.max(1).to_string());

            let on_previous = {
                let go_to = go_to.clone();
                Callback::from(move |_| go_to.emit(((page - 1).max(1), page_size)))
            };
            let on_next = {
                let go_to = go_to.clone();
                Callback::from(move |_| go_to.emit((page + 1, page_size)))
            };

            html! {
                <div class="card overflow-hidden p-0">
                    <div class="overflow-x-auto">
                        <div class="min-w-[920px]">
                            <div class={classes!(LIST_COLUMNS, "h-9", "text-xs", "text-fg-muted")}>
                                <span>{i18n.t("panel.tickets.column_ticket")}</span>
                                <span>{i18n.t("panel.tickets.column_member")}</span>
                                <span>{i18n.t("panel.tickets.category")}</span>
                                <span class="text-right">{i18n.t("panel.tickets.column_messages")}</span>
                                <span>{i18n.t("panel.tickets.opened")}</span>
                                <span>{i18n.t("panel.tickets.column_status")}</span>
                            </div>
                            if filtered.is_empty() {
                                <div class="border-t border-line">
                                    <EmptyState icon={IconName::Search} title={i18n.t("panel.tickets.no_results")} />
                                </div>
                            }
                            { for filtered.iter().map(|ticket| {
                                let onclick = {
                                    let navigator = navigator.clone();
                                    let id = ticket.id.clone();
                                    Callback::from(move |_| navigator.push(&PanelRoute::TicketDetails { id: id.clone() }))
                                };
                                html! {
                                    <TicketRow ticket={(*ticket).clone()} {onclick} dates_language={i18n.get_current_language().to_string()} />
                                }
                            }) }
                        </div>
                    </div>
                    <div class="flex h-[52px] items-center gap-4 border-t border-line px-5">
                        <span class="tnum flex-1 font-mono text-xs text-fg-muted">{range}</span>
                        <span class="text-[13px] text-fg-secondary">{page_of}</span>
                        <Button variant={ButtonVariant::Secondary} size={ButtonSize::Sm} disabled={data.page <= 1} onclick={on_previous}>
                            {i18n.t("panel.tickets.previous")}
                        </Button>
                        <Button variant={ButtonVariant::Secondary} size={ButtonSize::Sm} disabled={data.page >= data.total_pages} onclick={on_next}>
                            {i18n.t("panel.tickets.next")}
                        </Button>
                    </div>
                </div>
            }
        }
    };

    html! {
        <>
            <header class="flex flex-col gap-1">
                <h1 class="text-title font-semibold text-fg">{i18n.t("panel.tickets.title")}</h1>
                <p class="tnum text-sm text-fg-secondary">{count_label}</p>
            </header>
            <div class="flex flex-wrap items-center gap-3">
                <div class="relative w-[360px] max-w-full">
                    <span class="pointer-events-none absolute inset-y-0 left-3 flex items-center text-fg-muted">
                        <Icon name={IconName::Search} />
                    </span>
                    <input
                        type="search"
                        class="input pl-9"
                        value={(*search_query).clone()}
                        placeholder={i18n.t("panel.tickets.search_placeholder")}
                        oninput={on_search}
                    />
                </div>
                <div class="flex-1" />
                <Select
                    class="w-40"
                    value={AttrValue::from(page_size.to_string())}
                    options={page_size_options}
                    onchange={on_page_size}
                />
            </div>
            {body}
        </>
    }
}

#[derive(Properties, PartialEq)]
struct TicketRowProps {
    ticket: CompleteThread,
    onclick: Callback<MouseEvent>,
    dates_language: String,
}

#[function_component(TicketRow)]
fn ticket_row(props: &TicketRowProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let dates = DateFormatter::new(&props.dates_language);
    let ticket = &props.ticket;

    let status = if ticket.is_open() {
        let caption = ticket
            .messages
            .last()
            .map(|m| {
                let when = m
                    .timestamp_ms()
                    .map(|ms| dates.date_time(ms))
                    .unwrap_or_else(|| m.created_at.clone());
                format!("{} {}", i18n.t("panel.tickets.last_message"), when)
            })
            .unwrap_or_default();
        html! {
            <>
                <Badge tone={Tone::Accent}>{i18n.t("panel.tickets.status_open")}</Badge>
                <span class="truncate text-xs text-fg-muted">{caption}</span>
            </>
        }
    } else {
        let caption = ticket
            .closed_at
            .map(|closed| {
                format!(
                    "{} {}",
                    i18n.t("panel.tickets.closed_at"),
                    dates.date(closed as f64 * 1000.0)
                )
            })
            .unwrap_or_default();
        html! {
            <>
                <Badge tone={Tone::Neutral}>{i18n.t("panel.tickets.status_closed")}</Badge>
                <span class="truncate text-xs text-fg-muted">{caption}</span>
            </>
        }
    };

    html! {
        <button
            type="button"
            class={classes!(LIST_COLUMNS, "h-[52px]", "w-full", "cursor-pointer", "border-t", "border-line", "text-left", "transition-colors", "duration-150", "hover:bg-surface-raised/50")}
            onclick={props.onclick.clone()}
        >
            <span><TicketStub number={ticket.short_id()} /></span>
            <span class="flex min-w-0 items-center gap-2.5">
                <Avatar name={ticket.user_name.clone()} member={true} size={24} />
                <span class="truncate text-sm text-fg">{&ticket.user_name}</span>
            </span>
            <span class="truncate text-sm text-fg-secondary">
                {ticket.category_name.clone().unwrap_or_else(|| "\u{2014}".to_string())}
            </span>
            <span class="tnum text-right font-mono text-[13px] text-fg-secondary">
                {ticket.messages.len()}
            </span>
            <span class="tnum text-[13px] text-fg-secondary">{dates.seconds_date_time(ticket.created_at)}</span>
            <span class="flex min-w-0 items-center gap-2">{status}</span>
        </button>
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
    match use_user_permissions() {
        None => html! { <CheckingPermissions /> },
        Some(perms) if !perms.contains(&PanelPermission::ManageTickets) => {
            html! { <Forbidden403 required_permission={i18n.t("navbar.tickets")} /> }
        }
        Some(_) => {
            html! { <TicketDetailsContent id={props.id.clone()} on_back={props.on_back.clone()} /> }
        }
    }
}

#[function_component(TicketDetailsContent)]
fn ticket_details_content(props: &TicketDetailsProps) -> Html {
    let (i18n, _set_language) = use_translation();
    let ticket = use_state(|| LoadState::<CompleteThread>::Loading);
    let visible_types = use_state(|| MessageType::ALL.to_vec());
    let search_query = use_state(String::new);

    {
        let ticket = ticket.clone();
        use_effect_with(props.id.clone(), move |id| {
            let id = id.clone();
            ticket.set(LoadState::Loading);
            spawn_local(async move {
                let url = format!("/api/bot/tickets?id={}", id);
                let loaded = match Request::get(&url).send().await {
                    Ok(resp) => resp.json::<CompleteThread>().await.ok(),
                    Err(_) => None,
                };
                ticket.set(loaded.map(LoadState::Loaded).unwrap_or(LoadState::Failed));
            });
            || ()
        });
    }

    let back = {
        let on_back = props.on_back.clone();
        html! {
            <div>
                <Button variant={ButtonVariant::Ghost} size={ButtonSize::Sm} icon={IconName::ArrowLeft} onclick={Callback::from(move |_| on_back.emit(()))}>
                    {i18n.t("panel.tickets.back_to_tickets")}
                </Button>
            </div>
        }
    };

    let ticket = match &*ticket {
        LoadState::Loading => {
            return html! {
                <>
                    {back}
                    <div class="card"><Loading label={i18n.t("panel.tickets.loading_ticket")} /></div>
                </>
            };
        }
        LoadState::Failed => {
            return html! {
                <>
                    {back}
                    <div class="card">
                        <EmptyState
                            icon={IconName::Ticket}
                            title={i18n.t("panel.tickets.ticket_not_found")}
                            description={i18n.t("panel.tickets.ticket_not_found_description")}
                        />
                    </div>
                </>
            };
        }
        LoadState::Loaded(ticket) => ticket,
    };

    let dates = DateFormatter::new(i18n.get_current_language());
    let count_of = |kind: MessageType| {
        ticket
            .messages
            .iter()
            .filter(|m| m.message_type() == kind)
            .count()
    };

    let query = search_query.to_lowercase();
    let filtered: Vec<&ThreadMessage> = ticket
        .messages
        .iter()
        .filter(|m| visible_types.contains(&m.message_type()))
        .filter(|m| {
            query.is_empty()
                || m.content.to_lowercase().contains(&query)
                || m.user_name.to_lowercase().contains(&query)
        })
        .collect();

    let status_badge = if ticket.is_open() {
        html! { <Badge tone={Tone::Accent}>{i18n.t("panel.tickets.status_open")}</Badge> }
    } else {
        html! { <Badge tone={Tone::Neutral}>{i18n.t("panel.tickets.status_closed")}</Badge> }
    };

    let message_count_label = |count: usize| {
        format!(
            "{} {}",
            count,
            if count == 1 {
                i18n.t("panel.tickets.message_singular")
            } else {
                i18n.t("panel.tickets.message_plural")
            }
        )
    };

    let filter_chips = MessageType::ALL.iter().map(|&kind| {
        let on = visible_types.contains(&kind);
        let onclick = {
            let visible_types = visible_types.clone();
            Callback::from(move |_| {
                let mut next = (*visible_types).clone();
                if next.contains(&kind) {
                    next.retain(|k| *k != kind);
                } else {
                    next.push(kind);
                }
                visible_types.set(next);
            })
        };
        let state = if on {
            "border-line-strong bg-surface-raised text-fg"
        } else {
            "border-line text-fg-muted hover:text-fg-secondary"
        };
        html! {
            <button
                type="button"
                aria-pressed={on.to_string()}
                class={classes!("inline-flex", "h-7", "items-center", "gap-2", "rounded-md", "border", "px-2.5", "text-[13px]", "transition-colors", "duration-150", state)}
                {onclick}
            >
                <StatusDot class={kind.dot_class()} />
                {i18n.t(kind.label_key())}
                <span class="tnum font-mono text-xs text-fg-muted">{count_of(kind)}</span>
            </button>
        }
    });

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |e: InputEvent| search_query.set(input_value(&e)))
    };

    let transcript = if filtered.is_empty() {
        html! {
            <EmptyState icon={IconName::MessageSquare} title={i18n.t("panel.tickets.no_messages")} />
        }
    } else {
        html! {
            <div class="flex flex-col gap-1 px-5 py-4">
                { for transcript_rows(&filtered, &dates, &i18n.t("panel.tickets.filter_staff"), &i18n.t("panel.tickets.internal_note")) }
            </div>
        }
    };

    let detail_row = |label: String, value: Html| {
        html! {
            <div class="flex items-start justify-between gap-4 border-t border-line py-2.5 first:border-t-0">
                <span class="text-[13px] text-fg-secondary">{label}</span>
                <span class="min-w-0 text-right text-[13px] text-fg">{value}</span>
            </div>
        }
    };

    html! {
        <>
            {back}
            <header class="flex flex-wrap items-end gap-4">
                <div class="flex min-w-0 flex-1 flex-col gap-2">
                    <div class="flex items-center gap-3">
                        <h1 class="text-title font-semibold text-fg">{i18n.t("panel.tickets.column_ticket")}</h1>
                        <TicketStub number={ticket.short_id()} />
                    </div>
                    <div class="flex flex-wrap items-center gap-2 text-sm text-fg-secondary">
                        <span>{&ticket.user_name}</span>
                        if let Some(category) = &ticket.category_name {
                            <span class="text-fg-muted">{"\u{b7}"}</span>
                            <span>{category}</span>
                        }
                        <span class="text-fg-muted">{"\u{b7}"}</span>
                        {status_badge}
                    </div>
                </div>
                <a
                    href={ticket.transcript_data_url()}
                    download={format!("ticket-{}.txt", ticket.id)}
                    class={button_classes(ButtonVariant::Secondary, ButtonSize::Md)}
                >
                    <Icon name={IconName::FileText} />
                    {i18n.t("panel.tickets.download")}
                </a>
            </header>
            <div class="grid grid-cols-1 items-start gap-6 lg:grid-cols-[minmax(0,1fr)_320px]">
                <section class="card min-w-0 overflow-hidden p-0">
                    <div class="flex min-h-14 flex-wrap items-center gap-2 border-b border-line px-5 py-3">
                        { for filter_chips }
                        <div class="flex-1" />
                        <span class="tnum font-mono text-xs text-fg-muted">{message_count_label(filtered.len())}</span>
                        <div class="relative w-48">
                            <span class="pointer-events-none absolute inset-y-0 left-2.5 flex items-center text-fg-muted">
                                <Icon name={IconName::Search} size={14} />
                            </span>
                            <input
                                type="search"
                                class="input h-8 pl-8 text-[13px]"
                                value={(*search_query).clone()}
                                placeholder={i18n.t("panel.tickets.search_messages")}
                                oninput={on_search}
                            />
                        </div>
                    </div>
                    {transcript}
                </section>
                <aside class="flex flex-col gap-6 lg:sticky lg:top-6">
                    <section class="card flex flex-col gap-4 p-5">
                        <h2 class="text-base font-semibold text-fg">{i18n.t("panel.tickets.column_member")}</h2>
                        <div class="flex items-center gap-3">
                            <Avatar name={ticket.user_name.clone()} member={true} size={40} />
                            <div class="flex min-w-0 flex-col">
                                <span class="truncate text-sm font-medium text-fg">{&ticket.user_name}</span>
                                <span class="truncate font-mono text-xs text-fg-muted">{ticket.user_id}</span>
                            </div>
                        </div>
                        <div class="flex flex-col">
                            if let Some(category) = &ticket.category_name {
                                {detail_row(i18n.t("panel.tickets.category"), html! { category.clone() })}
                            }
                            {detail_row(i18n.t("panel.tickets.opened"), html! { <span class="tnum">{dates.seconds_date_time(ticket.created_at)}</span> })}
                            if let Some(closed) = ticket.closed_at {
                                {detail_row(i18n.t("panel.tickets.closed"), html! { <span class="tnum">{dates.seconds_date_time(closed)}</span> })}
                            }
                            if let Some(closed_by) = &ticket.closed_by {
                                {detail_row(i18n.t("panel.tickets.closed_by"), html! { <span class="font-mono text-xs">{closed_by.clone()}</span> })}
                            }
                            {detail_row(i18n.t("panel.tickets.column_messages"), html! { <span class="tnum font-mono">{ticket.messages.len()}</span> })}
                            if ticket.user_left {
                                {detail_row(i18n.t("panel.tickets.column_status"), html! { <Badge tone={Tone::Warning}>{i18n.t("panel.tickets.user_left")}</Badge> })}
                            }
                        </div>
                    </section>
                    <section class="card flex flex-col gap-3 p-5">
                        <h2 class="text-base font-semibold text-fg">{i18n.t("panel.tickets.column_messages")}</h2>
                        <div class="flex flex-col gap-2.5">
                            { for MessageType::ALL.iter().map(|&kind| html! {
                                <div class="flex items-center gap-2.5 text-[13px]">
                                    <StatusDot class={kind.dot_class()} />
                                    <span class="flex-1 text-fg-secondary">{i18n.t(kind.label_key())}</span>
                                    <span class="tnum font-mono text-fg">{count_of(kind)}</span>
                                </div>
                            }) }
                        </div>
                    </section>
                </aside>
            </div>
        </>
    }
}

fn transcript_rows(
    messages: &[&ThreadMessage],
    dates: &DateFormatter,
    staff_label: &str,
    internal_label: &str,
) -> Vec<Html> {
    let mut rows = Vec::new();
    let mut previous_day: Option<String> = None;
    let mut previous: Option<(&ThreadMessage, Option<f64>)> = None;

    for message in messages {
        let timestamp = message.timestamp_ms();
        let day = timestamp.map(|ms| dates.long_date(ms));
        let new_day = day.is_some() && day != previous_day;
        if new_day {
            rows.push(html! {
                <div class="flex items-center gap-3 py-3">
                    <span class="h-px flex-1 bg-line" />
                    <span class="text-xs text-fg-muted">{day.clone().unwrap_or_default()}</span>
                    <span class="h-px flex-1 bg-line" />
                </div>
            });
            previous_day = day;
        }

        let kind = message.message_type();
        let time = dates.message_time(message);
        let collapsed = !new_day
            && kind == MessageType::User
            && previous.is_some_and(|(prev, prev_ts)| {
                prev.message_type() == MessageType::User
                    && prev.user_id == message.user_id
                    && matches!((prev_ts, timestamp), (Some(a), Some(b)) if b - a <= COLLAPSE_WINDOW_MS)
            });

        let content =
            html! { <div class={MARKDOWN}>{markdown_to_html_safe(&message.content)}</div> };

        rows.push(match kind {
            MessageType::User if collapsed => html! {
                <div class="group flex gap-3 py-0.5">
                    <span class="tnum w-8 shrink-0 pt-0.5 text-right font-mono text-[11px] text-fg-muted opacity-0 group-hover:opacity-100">
                        {time}
                    </span>
                    {content}
                </div>
            },
            MessageType::User => html! {
                <div class="flex gap-3 pt-3">
                    <Avatar name={message.user_name.clone()} member={true} size={32} />
                    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                        <div class="flex items-baseline gap-2">
                            <span class="text-sm font-medium text-fg">{&message.user_name}</span>
                            <span class="tnum font-mono text-xs text-fg-muted">{time}</span>
                        </div>
                        {content}
                    </div>
                </div>
            },
            MessageType::Staff => html! {
                <div class="flex gap-3 pt-3">
                    <Avatar name={message.user_name.clone()} size={32} />
                    <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                        <div class="flex flex-wrap items-center gap-2">
                            <span class="text-sm font-medium text-fg">{&message.user_name}</span>
                            <Badge tone={Tone::Accent}>{staff_label.to_string()}</Badge>
                            if let Some(number) = message.message_number {
                                <span class="tnum font-mono text-xs text-fg-muted">{format!("#{}", number)}</span>
                            }
                            <span class="tnum font-mono text-xs text-fg-muted">{time}</span>
                        </div>
                        {content}
                    </div>
                </div>
            },
            MessageType::System => html! {
                <div class="flex items-start gap-2.5 py-1.5 pl-2 text-fg-muted">
                    <Icon name={system_icon(message)} class="mt-0.5 shrink-0" />
                    <div class="min-w-0 flex-1 break-words text-[13px] text-fg-secondary [&_p+p]:mt-1 [&_strong]:font-medium [&_a]:underline [&_code]:font-mono">
                        {markdown_to_html_safe(&message.content)}
                    </div>
                    <span class="tnum shrink-0 font-mono text-xs text-fg-muted">{time}</span>
                </div>
            },
            MessageType::Internal => html! {
                <div class="my-1.5 ml-11 flex flex-col gap-1.5 rounded-r-lg border-l-2 border-warning bg-warning-subtle px-4 py-3">
                    <div class="flex flex-wrap items-center gap-2 text-xs">
                        <span class="text-warning"><Icon name={IconName::StickyNote} size={14} /></span>
                        <span class="font-medium text-warning">{internal_label.to_string()}</span>
                        <span class="tnum font-mono text-fg-muted">{format!("{} \u{b7} {}", message.user_name, time)}</span>
                    </div>
                    {content}
                </div>
            },
        });

        previous = Some((message, timestamp));
    }
    rows
}

fn system_icon(message: &ThreadMessage) -> IconName {
    let content = message.content.to_lowercase();
    if content.contains("cdn.discordapp.com/attachments") {
        IconName::Paperclip
    } else if content.contains("<t:") {
        IconName::Clock
    } else if content.contains("ticket") || content.contains("thread") {
        IconName::Ticket
    } else {
        IconName::CircleCheck
    }
}
