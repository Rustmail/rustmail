use crate::types::PanelPermission;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[hook]
pub fn use_user_permissions() -> Option<Vec<PanelPermission>> {
    let permissions = use_state(|| None::<Vec<PanelPermission>>);
    {
        let permissions = permissions.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let loaded = match Request::get("/api/user/permissions").send().await {
                    Ok(resp) => resp
                        .json::<Vec<PanelPermission>>()
                        .await
                        .unwrap_or_default(),
                    Err(_) => Vec::new(),
                };
                permissions.set(Some(loaded));
            });
            || ()
        });
    }
    (*permissions).clone()
}

#[hook]
pub fn use_user_avatar() -> Option<AttrValue> {
    #[derive(serde::Deserialize)]
    struct UserAvatar {
        avatar_url: Option<String>,
    }
    let avatar = use_state(|| None::<AttrValue>);
    {
        let avatar = avatar.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                if let Ok(resp) = Request::get("/api/user/avatar").send().await
                    && let Ok(data) = resp.json::<UserAvatar>().await
                {
                    avatar.set(data.avatar_url.map(AttrValue::from));
                }
            });
            || ()
        });
    }
    (*avatar).clone()
}
