use crate::pages::panel::PanelRoute;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Forbidden403Props {
    pub required_permission: String,
}

#[function_component(Forbidden403)]
pub fn forbidden_403(props: &Forbidden403Props) -> Html {
    let navigator = use_navigator().unwrap();

    let go_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| {
            navigator.push(&PanelRoute::Home);
        })
    };

    html! {
        <div class="flex flex-col items-center justify-center min-h-[calc(100vh-12rem)] px-4">
            <div class="max-w-md w-full bg-gradient-to-br from-slate-800 to-slate-900 rounded-xl shadow-2xl p-8 border border-slate-700 text-center">
                <div class="flex justify-center mb-6">
                    <div class="bg-red-500/10 p-4 rounded-full">
                        <svg class="w-16 h-16 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
                        </svg>
                    </div>
                </div>

                <h1 class="text-3xl font-bold text-white mb-3">{"Accès refusé"}</h1>

                <p class="text-gray-400 mb-2">
                    {"Vous n'avez pas la permission nécessaire pour accéder à cette page."}
                </p>

                <div class="bg-slate-900/50 border border-slate-700 rounded-lg p-4 mb-6">
                    <p class="text-sm text-gray-500 mb-1">{"Permission requise :"}</p>
                    <p class="text-lg font-semibold text-red-400">{&props.required_permission}</p>
                </div>

                <p class="text-sm text-gray-500 mb-6">
                    {"Contactez un administrateur si vous pensez que vous devriez avoir accès à cette section."}
                </p>

                <button
                    onclick={go_home}
                    class="w-full px-6 py-3 bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 text-white font-semibold rounded-lg transition-all duration-200 shadow-lg hover:shadow-xl flex items-center justify-center space-x-2"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                    </svg>
                    <span>{"Retour à l'accueil"}</span>
                </button>
            </div>
        </div>
    }
}
