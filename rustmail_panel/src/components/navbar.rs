use crate::components::language_switcher::LanguageSwitcher;
use yew::{Callback, Html, Properties, function_component, html, use_state};

#[derive(Properties, PartialEq, Clone)]
pub struct RustmailNavbarProps {
    pub avatar_url: String,
}

#[function_component(RustmailNavbar)]
pub fn rustmail_navbar(props: &RustmailNavbarProps) -> Html {
    let mobile_menu_open = use_state(|| false);
    let profile_menu_open = use_state(|| false);

    let toggle_mobile_menu = {
        let mobile_menu_open = mobile_menu_open.clone();
        Callback::from(move |_| mobile_menu_open.set(!*mobile_menu_open))
    };

    let toggle_profile_menu = {
        let profile_menu_open = profile_menu_open.clone();
        Callback::from(move |_| profile_menu_open.set(!*profile_menu_open))
    };

    html! {
        <nav class="fixed top-0 left-0 w-full z-50 bg-gradient-to-r from-slate-900 to-black border-b border-slate-800">
            <div class="w-full px-4 sm:px-6 lg:px-8">
                <div class="flex h-16 items-center justify-between">

                    <div class="flex items-center space-x-4">
                        <a href="#">
                            <img
                                src="logo.png"
                                alt="Logo"
                                class="h-7 w-auto"
                            />
                        </a>

                        <div class="hidden sm:flex space-x-4">
                            <a href="/panel" class="rounded-md bg-white/10 px-3 py-2 text-sm font-medium text-white hover:bg-white/20 transition">{"Accueil"}</a>
                            <a href="#" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-white/10 hover:text-white">{"Configuration"}</a>
                            <a href="#" class="rounded-md px-3 py-2 text-sm font-medium text-gray-300 hover:bg-white/10 hover:text-white">{"Tickets"}</a>
                        </div>
                    </div>

                    <div class="flex items-center space-x-4">
                        <button onclick={toggle_mobile_menu.clone()} class="sm:hidden inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-800 focus:outline-none">
                            <svg class="h-6 w-6" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16M4 18h16" />
                            </svg>
                        </button>

                        <LanguageSwitcher placement="nav"/>

                        <button type="button" class="relative rounded-full p-1 text-gray-300 hover:text-white transition">
                            <span class="sr-only">{"View notifications"}</span>
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" class="h-5 w-5">
                                <path d="M14.857 17.082a23.848 23.848 0 0 0 5.454-1.31A8.967 8.967 0 0 1 18 9.75V9A6 6 0 0 0 6 9v.75a8.967 8.967 0 0 1-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 0 1-5.714 0m5.714 0a3 3 0 1 1-5.714 0" stroke-linecap="round" stroke-linejoin="round" />
                            </svg>
                        </button>

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
                                        <a href="/api/auth/logout" class="block px-4 py-2 text-sm text-gray-300 hover:bg-white/5">{"Sign out"}</a>
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </div>
                </div>
            </div>

            { if *mobile_menu_open {
                html! {
                    <div class="sm:hidden border-t border-gray-700 bg-slate-900/95 backdrop-blur-md">
                        <div class="px-2 pt-2 pb-3 space-y-1">
                            <a href="#" class="block rounded-md px-3 py-2 text-base font-medium text-white hover:bg-white/10">{"Accueil"}</a>
                            <a href="#" class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-white/10 hover:text-white">{"Configuration"}</a>
                            <a href="#" class="block rounded-md px-3 py-2 text-base font-medium text-gray-300 hover:bg-white/10 hover:text-white">{"Tickets"}</a>
                        </div>
                    </div>
                }
            } else {
                html! {}
            }}
        </nav>
    }
}
