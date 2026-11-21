use crate::components::language_switcher::LanguageSwitcher;
use crate::components::ui::{Avatar, Icon, IconName, StatusDot};
use crate::i18n::yew::use_translation;
use crate::pages::panel::PanelRoute;
use crate::types::PanelPermission;
use gloo_net::http::Request;
use rustmail_types::VersionInfo;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::hooks::{use_location, use_navigator};

#[derive(Deserialize, Clone, PartialEq)]
struct BotStatusResponse {
    status: String,
}

#[derive(Clone, PartialEq)]
enum Destination {
    Panel(PanelRoute),
    Page(&'static str),
}

struct NavEntry {
    label_key: &'static str,
    icon: IconName,
    destination: Destination,
    path: &'static str,
    permission: PanelPermission,
}

fn workspace_entries() -> Vec<NavEntry> {
    vec![
        NavEntry {
            label_key: "navbar.home",
            icon: IconName::LayoutDashboard,
            destination: Destination::Panel(PanelRoute::Home),
            path: "/panel",
            permission: PanelPermission::ViewPanel,
        },
        NavEntry {
            label_key: "navbar.tickets",
            icon: IconName::Inbox,
            destination: Destination::Panel(PanelRoute::TicketsList),
            path: "/panel/tickets",
            permission: PanelPermission::ManageTickets,
        },
        NavEntry {
            label_key: "navbar.categories",
            icon: IconName::Folder,
            destination: Destination::Panel(PanelRoute::Categories),
            path: "/panel/categories",
            permission: PanelPermission::ManageCategories,
        },
        NavEntry {
            label_key: "navbar.bans",
            icon: IconName::Ban,
            destination: Destination::Panel(PanelRoute::Bans),
            path: "/panel/bans",
            permission: PanelPermission::ViewBans,
        },
    ]
}

fn settings_entries() -> Vec<NavEntry> {
    vec![
        NavEntry {
            label_key: "navbar.config",
            icon: IconName::Settings,
            destination: Destination::Panel(PanelRoute::Configuration),
            path: "/panel/configuration",
            permission: PanelPermission::ManageConfig,
        },
        NavEntry {
            label_key: "navbar.apikeys",
            icon: IconName::KeyRound,
            destination: Destination::Panel(PanelRoute::ApiKeys),
            path: "/panel/apikeys",
            permission: PanelPermission::ManageApiKeys,
        },
        NavEntry {
            label_key: "navbar.administration",
            icon: IconName::Shield,
            destination: Destination::Page("/admin"),
            path: "/admin",
            permission: PanelPermission::ManagePermissions,
        },
    ]
}

fn is_active(current: &str, path: &str) -> bool {
    if path == "/panel" {
        current == "/panel" || current == "/panel/"
    } else {
        current.starts_with(path)
    }
}

#[derive(Properties, PartialEq)]
pub struct AppShellProps {
    pub permissions: Vec<PanelPermission>,
    #[prop_or_default]
    pub avatar_url: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(AppShell)]
pub fn app_shell(props: &AppShellProps) -> Html {
    let (i18n, _) = use_translation();
    let open = use_state(|| false);
    let navigator = use_navigator();
    let location = use_location();
    let current = location
        .as_ref()
        .map(|l| l.path().to_string())
        .unwrap_or_default();

    {
        let open = open.clone();
        use_effect_with(current.clone(), move |_| {
            open.set(false);
            || ()
        });
    }

    let bot_online = use_state(|| None::<bool>);
    let version = use_state(|| None::<String>);
    {
        let bot_online = bot_online.clone();
        let version = version.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/bot/status").send().await
                    && resp.ok()
                    && let Ok(status) = resp.json::<BotStatusResponse>().await
                {
                    bot_online.set(Some(status.status == "running"));
                }
                if let Ok(resp) = Request::get("/api/bot/version").send().await
                    && resp.ok()
                    && let Ok(info) = resp.json::<VersionInfo>().await
                {
                    version.set(Some(info.current));
                }
            });
            || ()
        });
    }

    let render_entry = |entry: &NavEntry| -> Html {
        if !props.permissions.contains(&entry.permission) {
            return html! {};
        }
        let active = is_active(&current, entry.path);
        let onclick = {
            let navigator = navigator.clone();
            let destination = entry.destination.clone();
            Callback::from(move |e: MouseEvent| {
                if let (Destination::Panel(route), Some(nav)) = (&destination, &navigator) {
                    e.prevent_default();
                    nav.push(route);
                }
            })
        };
        let state = if active {
            "bg-surface-raised text-fg"
        } else {
            "text-fg-secondary hover:bg-surface-raised/60 hover:text-fg"
        };
        let icon_class = if active { "text-accent-fg" } else { "" };
        html! {
            <a
                href={entry.path}
                {onclick}
                aria-current={active.then_some("page")}
                class={classes!("flex", "h-8", "items-center", "gap-2.5", "rounded-md", "px-2", "text-sm", "transition-colors", "duration-150", state)}
            >
                <Icon name={entry.icon} class={icon_class} />
                <span class="truncate">{i18n.t(entry.label_key)}</span>
            </a>
        }
    };

    let settings = settings_entries();
    let has_settings = settings
        .iter()
        .any(|e| props.permissions.contains(&e.permission));

    let status_card = match *bot_online {
        None => html! {},
        Some(online) => html! {
            <div class="flex items-center gap-2.5 rounded-xl border border-line bg-surface px-3 py-2.5">
                <StatusDot class={if online { "bg-positive" } else { "bg-negative" }} />
                <div class="flex min-w-0 flex-col">
                    <span class="text-sm font-medium text-fg">
                        {i18n.t(if online { "navbar.bot_online" } else { "navbar.bot_offline" })}
                    </span>
                    if let Some(v) = (*version).clone() {
                        <span class="font-mono text-xs text-fg-muted">{format!("Rustmail v{}", v)}</span>
                    }
                </div>
            </div>
        },
    };

    let sidebar = html! {
        <div class="flex h-full flex-col p-3">
            <a href="/panel" class="flex h-10 items-center gap-2.5 px-2">
                <img src="/logo.png" alt="" class="size-7" />
                <span class="text-base font-medium text-fg">{"Rustmail"}</span>
            </a>
            <nav class="mt-5 flex flex-col gap-0.5">
                { for workspace_entries().iter().map(render_entry) }
            </nav>
            if has_settings {
                <div class="mt-6 flex flex-col gap-0.5">
                    <span class="px-2 pb-1.5 text-xs text-fg-muted">{i18n.t("navbar.section_settings")}</span>
                    { for settings.iter().map(render_entry) }
                </div>
            }
            <div class="flex-1" />
            {status_card}
            <div class="mt-2 flex h-10 items-center gap-2.5 pl-2 pr-1">
                <Avatar name="" url={props.avatar_url.clone()} size={28} />
                <span class="flex-1" />
                <LanguageSwitcher compact=true />
                <a
                    href="/api/auth/logout"
                    class="inline-flex size-7 items-center justify-center rounded-md text-fg-secondary transition-colors hover:bg-surface-raised hover:text-fg"
                    aria-label={i18n.t("navbar.logout")}
                    title={i18n.t("navbar.logout")}
                >
                    <Icon name={IconName::LogOut} />
                </a>
            </div>
        </div>
    };

    let toggle = {
        let open = open.clone();
        Callback::from(move |_| open.set(!*open))
    };
    let close = {
        let open = open.clone();
        Callback::from(move |_| open.set(false))
    };
    let drawer = if *open {
        "translate-x-0"
    } else {
        "-translate-x-full lg:translate-x-0"
    };

    html! {
        <div class="min-h-screen bg-surface-page lg:pl-[248px]">
            <aside class={classes!("fixed", "inset-y-0", "left-0", "z-40", "w-[248px]", "border-r", "border-line", "bg-surface-sunken", "transition-transform", "duration-300", "ease-out", drawer)}>
                {sidebar}
            </aside>
            if *open {
                <div class="fixed inset-0 z-30 bg-black/50 lg:hidden" onclick={close} />
            }
            <div class="sticky top-0 z-20 flex h-14 items-center gap-3 border-b border-line bg-surface-sunken px-4 lg:hidden">
                <button type="button" class="inline-flex size-8 items-center justify-center rounded-md text-fg-secondary hover:bg-surface-raised" aria-label={i18n.t("navbar.menu")} onclick={toggle}>
                    <Icon name={IconName::Menu} />
                </button>
                <img src="/logo.png" alt="" class="size-6" />
                <span class="text-base font-medium">{"Rustmail"}</span>
            </div>
            <main class="mx-auto flex w-full max-w-[1440px] flex-col gap-6 px-4 pb-12 pt-7 sm:px-8">
                {props.children.clone()}
            </main>
        </div>
    }
}
