use crate::router::Route;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SetupStatusResponse {
    pub setup_required: bool,
    pub step: String,
}

#[function_component(SetupDetector)]
pub fn setup_detector() -> Html {
    let navigator = use_navigator();
    let route = use_route::<Route>();
    let is_checking = use_state(|| true);

    {
        let navigator = navigator.clone();
        let is_checking = is_checking.clone();
        let already_on_setup = matches!(route, Some(Route::Setup));

        use_effect_with((), move |_| {
            if already_on_setup {
                is_checking.set(false);
            } else {
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
            }
            || ()
        });
    }

    let _ = is_checking;
    html! {}
}
