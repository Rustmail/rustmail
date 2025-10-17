use crate::components::navbar::RustmailNavbar;
use gloo_net::http::Request;
use gloo_utils::window;
use wasm_bindgen_futures::spawn_local;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;

#[derive(serde::Deserialize, Debug, serde::Serialize)]
pub struct UserAvatar {
    pub avatar_url: Option<String>,
}

fn get_cookie(name: &str) -> Option<String> {
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .dyn_into::<HtmlDocument>()
        .unwrap();

    let cookies = document.cookie().unwrap_or_default();

    cookies
        .split(';')
        .map(|s| s.trim())
        .find(|s| s.starts_with(&format!("{}=", name)))
        .map(|s| s.split_at(name.len() + 1).1.to_string())
}

#[function_component(Panel)]
pub fn panel() -> Html {
    let (i18n, _set_language) = i18nrs::yew::use_translation();

    let authorized = use_state(|| None::<bool>);
    {
        let authorized = authorized.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let ok = Request::get("/api/panel/check")
                    .send()
                    .await
                    .map(|r| r.status() == 200)
                    .unwrap_or(false);
                authorized.set(Some(ok));
            });
            || ()
        });
    }

    {
        let authorized = authorized.clone();
        use_effect_with((*authorized).clone(), move |auth| {
            if matches!(auth, Some(false)) {
                let _ = window().location().set_href("/");
            }
            || ()
        });
    }

    let user_id: u64 = get_cookie("user_id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let avatar = use_state(|| None::<String>);
    {
        let avatar = avatar.clone();
        use_effect_with(user_id, move |&user_id| {
            let avatar = avatar.clone();
            spawn_local(async move {
                if user_id == 0 {
                    avatar.set(None);
                    return;
                }

                let url = format!("/api/user/avatar?id={}", user_id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(user_avatar) = resp.json::<UserAvatar>().await {
                        avatar.set(user_avatar.avatar_url);
                    } else {
                        avatar.set(None);
                    }
                } else {
                    avatar.set(None);
                }
            });
            || ()
        });
    }
    let avatar_url = (*avatar).clone().unwrap_or_default();

    html! {
        <>
            {
                match *authorized {
                    None => html! {
                        <section class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                            <p class="text-gray-400 animate-pulse">{"Vérification de l'accès..."}</p>
                        </section>
                    },
                    Some(true) => html! {
                        <>
                            <RustmailNavbar avatar_url={avatar_url.clone()} />

                            <section class="pt-24 flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-slate-900 to-black text-white">
                                <img src="logo.png" alt="Rustmail logo" class="w-40 h-40 mb-6" />
                                <h1 class="text-3xl font-bold mb-2">{"Rustmail Panel"}</h1>
                                <p class="max-w-xl text-center text-gray-400 mb-8">{ i18n.t("panel.welcome") }</p>
                            </section>
                        </>
                    },
                    Some(false) => html! {},
                }
            }
        </>
    }
}
