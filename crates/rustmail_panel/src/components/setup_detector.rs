use crate::router::Route;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize)]
pub struct SetupStatusResponse {
    pub setup_required: bool,
    pub step: String,
}

#[function_component(SetupDetector)]
pub fn setup_detector() -> Html {
    let navigator = use_navigator();
    let is_checking = use_state(|| true);

    {
        let navigator = navigator.clone();
        let is_checking = is_checking.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/setup/status").send().await {
                    if let Ok(status) = resp.json::<SetupStatusResponse>().await {
                        if status.setup_required {
                            if let Some(nav) = navigator {
                                nav.push(&Route::Setup);
                            }
                        }
                    }
                }
                is_checking.set(false);
            });
            || ()
        });
    }

    if *is_checking {
        html! {
            <div class="flex items-center justify-center min-h-screen bg-slate-900 text-white">
                <div class="text-gray-400 animate-pulse">{ "Checking setup mode..." }</div>
            </div>
        }
    } else {
        html! {}
    }
}
