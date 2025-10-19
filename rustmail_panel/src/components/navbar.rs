use crate::components::language_switcher::LanguageSwitcher;
use crate::pages::panel::PanelRoute;
use yew::{function_component, html, use_state, Callback, Html, Properties, classes};
use yew_router::hooks::{use_location, use_navigator};

#[derive(Properties, PartialEq, Clone)]
pub struct RustmailNavbarProps {
    pub avatar_url: String,
}

#[function_component(RustmailNavbar)]
pub fn rustmail_navbar(props: &RustmailNavbarProps) -> Html {
    let mobile_menu_open = use_state(|| false);
    let profile_menu_open = use_state(|| false);
    let navigator = use_navigator();
    let current_path = use_location()
        .map(|loc| loc.path().to_string())
        .unwrap_or_default();

    let toggle_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| mobile_menu_open.set(!*mobile_menu_open))
    };

    let toggle_profile_menu = {
        let profile_menu_open = profile_menu_open.clone();
        Callback::from(move |_| profile_menu_open.set(!*profile_menu_open))
    };

    let home_active = current_path == "/panel";
    let config_active = current_path == "/panel/configuration";
    let tickets_active = current_path.starts_with("/panel/tickets");

    html! {
        <nav class="fixed top-0 left-0 w-full z-50 bg-gradient-to-r from-slate-900 to-black border-b border-slate-800">
            <div class="w-full px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between">

                    <div class="flex items-center space-x-4">
                        <button onclick={{
                            let navigator = navigator.clone();
                            move |_| if let Some(nav) = &navigator {
                                nav.push(&PanelRoute::Home);
                            }
                        }}>
                            <img src="logo.png" alt="Logo" class="h-7 w-auto" />
                        </button>

                        <div class="hidden sm:flex space-x-4">
                            <button
                                onclick={{
                                    let navigator = navigator.clone();
                                    move |_| if let Some(nav) = &navigator {
                                        nav.push(&PanelRoute::Home);
                                    }
                                }}
                                class={classes!(
                                    "rounded-md", "px-3", "py-2", "text-sm", "transition",
                                    if home_active {
                                        "bg-white/10 text-white"
                                    } else {
                                        "text-gray-300 hover:bg-white/10 hover:text-white"
                                    }
                                )}
                            >
                                {"Accueil"}
                            </button>

                            <button
                                onclick={{
                                    let navigator = navigator.clone();
                                    move |_| if let Some(nav) = &navigator {
                                        nav.push(&PanelRoute::Configuration);
                                    }
                                }}
                                class={classes!(
                                    "rounded-md", "px-3", "py-2", "text-sm", "transition",
                                    if config_active {
                                        "bg-white/10 text-white"
                                    } else {
                                        "text-gray-300 hover:bg-white/10 hover:text-white"
                                    }
                                )}
                            >
                                {"Configuration"}
                            </button>

                            <button
                                onclick={{
                                    let navigator = navigator.clone();
                                    move |_| if let Some(nav) = &navigator {
                                        nav.push(&PanelRoute::TicketsList);
                                    }
                                }}
                                class={classes!(
                                    "rounded-md", "px-3", "py-2", "text-sm", "transition",
                                    if tickets_active {
                                        "bg-white/10 text-white"
                                    } else {
                                        "text-gray-300 hover:bg-white/10 hover:text-white"
                                    }
                                )}
                            >
                                {"Tickets"}
                            </button>
                        </div>
                    </div>

                    <div class="flex items-center space-x-4">
                        <button onclick={toggle_mobile_menu.clone()} class="sm:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-800 focus:outline-none">
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>

                        <LanguageSwitcher placement="nav" />

                        <div class="relative">
                            <button onclick={toggle_profile_menu.clone()} class="flex rounded-full focus:outline-none">
                                <span class="sr-only">{"Open user menu"}</span>
                                <img
                                    src={props.avatar_url.clone()}
                                    alt="User"
                                    class="h-6 w-6 rounded-full bg-gray-800 outline outline-1 outline-white/10"
                                />
                            </button>

                            { if *profile_menu_open {
                                html! {
                                    <div class="absolute right-0 mt-2 w-48 origin-top-right rounded-md bg-gray-900/90 backdrop-blur-md py-1 shadow-lg ring-1 ring-black/5">
                                        <a href="/api/auth/logout" class="block px-4 py-2 text-sm text-gray-300 hover:bg-white/5">{"Déconnexion"}</a>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </div>
                </div>
            </div>

            <div class={classes!(
                "sm:hidden", "border-t", "border-gray-700", "bg-slate-900/95",
                "backdrop-blur-md", "transition-all", "duration-300", "overflow-hidden",
                if *mobile_menu_open { "max-h-64 opacity-100" } else { "max-h-0 opacity-0" }
            )}>
                <div class="px-2 pt-2 pb-3 space-y-1">
                    <button
                        onclick={{
                            let navigator = navigator.clone();
                            let mobile_menu_open = mobile_menu_open.clone();
                            move |_| {
                                if let Some(nav) = &navigator {
                                    nav.push(&PanelRoute::Home);
                                }
                                mobile_menu_open.set(false);
                            }
                        }}
                        class={classes!(
                            "block", "w-full", "text-left", "rounded-md", "px-3", "py-2", "text-sm", "transition",
                            if home_active {
                                "bg-white/10 text-white"
                            } else {
                                "text-gray-300 hover:bg-white/10 hover:text-white"
                            }
                        )}
                    >
                        {"Accueil"}
                    </button>

                    <button
                        onclick={{
                            let navigator = navigator.clone();
                            let mobile_menu_open = mobile_menu_open.clone();
                            move |_| {
                                if let Some(nav) = &navigator {
                                    nav.push(&PanelRoute::Configuration);
                                }
                                mobile_menu_open.set(false);
                            }
                        }}
                        class={classes!(
                            "block", "w-full", "text-left", "rounded-md", "px-3", "py-2", "text-sm", "transition",
                            if config_active {
                                "bg-white/10 text-white"
                            } else {
                                "text-gray-300 hover:bg-white/10 hover:text-white"
                            }
                        )}
                    >
                        {"Configuration"}
                    </button>

                    <button
                        onclick={{
                            let navigator = navigator.clone();
                            let mobile_menu_open = mobile_menu_open.clone();
                            move |_| {
                                if let Some(nav) = &navigator {
                                    nav.push(&PanelRoute::TicketsList);
                                }
                                mobile_menu_open.set(false);
                            }
                        }}
                        class={classes!(
                            "block", "w-full", "text-left", "rounded-md", "px-3", "py-2", "text-sm", "transition",
                            if tickets_active {
                                "bg-white/10 text-white"
                            } else {
                                "text-gray-300 hover:bg-white/10 hover:text-white"
                            }
                        )}
                    >
                        {"Tickets"}
                    </button>
                </div>
            </div>
        </nav>
    }
}
