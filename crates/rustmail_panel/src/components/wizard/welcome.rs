use crate::components::wizard::auth::authed_get;
use crate::i18n::yew::use_translation;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct WelcomeProps {
    pub on_start: Callback<()>,
    pub on_unauthorized: Callback<()>,
}

#[function_component(Welcome)]
pub fn welcome(props: &WelcomeProps) -> Html {
    let (i18n, _) = use_translation();
    let is_checking = use_state(|| false);
    let error = use_state(|| None::<String>);

    let on_click = {
        let on_start = props.on_start.clone();
        let on_unauthorized = props.on_unauthorized.clone();
        let is_checking = is_checking.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let on_start = on_start.clone();
            let on_unauthorized = on_unauthorized.clone();
            let is_checking = is_checking.clone();
            let error = error.clone();

            is_checking.set(true);
            error.set(None);

            spawn_local(async move {
                let res = authed_get("/api/setup/verify").send().await;

                match res {
                    Ok(resp) if resp.status() == 401 => on_unauthorized.emit(()),
                    Ok(resp) if resp.ok() => on_start.emit(()),
                    Ok(_) => error.set(Some("Unexpected server response".to_string())),
                    Err(_) => error.set(Some("Network error".to_string())),
                }

                is_checking.set(false);
            });
        })
    };

    html! {
        <div class="min-h-screen bg-slate-950 flex flex-col items-center justify-center gap-6 px-4 text-center animate-fade-in">
            <div class="w-16 h-16 bg-indigo-500/20 rounded-full flex items-center justify-center text-indigo-400 ring-4 ring-indigo-500/10">
                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path></svg>
            </div>
            <h1 class="text-3xl font-bold text-white">{ i18n.t("wizard.welcome_title") }</h1>
            <p class="text-gray-400 max-w-md">{ i18n.t("wizard.welcome_description") }</p>
            <button
                class="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white font-medium rounded-lg transition-colors shadow-lg shadow-indigo-600/20 disabled:opacity-50 disabled:cursor-not-allowed min-w-[160px]"
                onclick={on_click}
                disabled={*is_checking}
            >
                if *is_checking {
                    <svg class="animate-spin h-5 w-5 text-white mx-auto" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"><circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle><path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path></svg>
                } else {
                    { i18n.t("wizard.welcome_start") }
                }
            </button>
            if let Some(err) = (*error).as_ref() {
                <p class="text-sm text-red-400">{ err }</p>
            }
        </div>
    }
}
