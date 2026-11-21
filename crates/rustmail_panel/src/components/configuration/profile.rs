use crate::components::ui::{
    Avatar, Button, ButtonSize, ButtonVariant, CardHeader, EmptyState, Field, Icon, IconName,
    Loading, Notice, NoticeTone, button_classes, input_value,
};
use crate::i18n::yew::use_translation;
use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const IMAGE_TYPES: &str = "image/png,image/jpeg,image/gif,image/webp";

#[derive(Clone, PartialEq, serde::Deserialize)]
struct BotProfile {
    username: String,
    avatar_url: Option<String>,
    banner_url: Option<String>,
}

async fn fetch_profile() -> Result<Option<BotProfile>, ()> {
    let resp = Request::get("/api/bot/profile")
        .send()
        .await
        .map_err(|_| ())?;
    if !resp.ok() {
        return Ok(None);
    }
    Ok(resp.json::<BotProfile>().await.ok())
}

fn read_image(file: web_sys::File, on_loaded: Callback<String>) {
    let Ok(reader) = web_sys::FileReader::new() else {
        return;
    };
    let source = reader.clone();
    let onload = Closure::wrap(Box::new(move |_: web_sys::Event| {
        if let Some(data_url) = source.result().ok().and_then(|r| r.as_string()) {
            on_loaded.emit(data_url);
        }
    }) as Box<dyn FnMut(_)>);
    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    onload.forget();
    let _ = reader.read_as_data_url(&file);
}

fn picked_file(on_file: Callback<web_sys::File>) -> Callback<Event> {
    Callback::from(move |e: Event| {
        if let Some(file) = e
            .target_dyn_into::<web_sys::HtmlInputElement>()
            .and_then(|input| input.files())
            .and_then(|files| files.get(0))
        {
            on_file.emit(file);
        }
    })
}

fn dropped_file(on_file: Callback<web_sys::File>) -> Callback<DragEvent> {
    Callback::from(move |e: DragEvent| {
        e.prevent_default();
        if let Some(file) = e
            .data_transfer()
            .and_then(|transfer| transfer.files())
            .and_then(|files| files.get(0))
        {
            on_file.emit(file);
        }
    })
}

fn allow_drop() -> Callback<DragEvent> {
    Callback::from(|e: DragEvent| e.prevent_default())
}

#[derive(Properties, PartialEq)]
pub struct DiscordProfileCardProps {
    pub bot_running: bool,
}

#[function_component(DiscordProfileCard)]
pub fn discord_profile_card(props: &DiscordProfileCardProps) -> Html {
    let (i18n, _) = use_translation();
    let t = |key: &str| i18n.t(&format!("panel.configuration.discord_profile.{key}"));

    let profile = use_state(|| None::<BotProfile>);
    let loading = use_state(|| true);
    let load_error = use_state(|| None::<String>);
    let username = use_state(String::new);
    let new_avatar = use_state(|| None::<String>);
    let new_banner = use_state(|| None::<String>);
    let saving = use_state(|| false);
    let feedback = use_state(|| None::<(bool, String)>);

    {
        let profile = profile.clone();
        let loading = loading.clone();
        let load_error = load_error.clone();
        let username = username.clone();
        let i18n = i18n.clone();
        use_effect_with(props.bot_running, move |running| {
            if *running {
                spawn_local(async move {
                    loading.set(true);
                    load_error.set(None);
                    match fetch_profile().await {
                        Ok(Some(loaded)) => {
                            username.set(loaded.username.clone());
                            profile.set(Some(loaded));
                        }
                        Ok(None) => load_error.set(Some(
                            i18n.t("panel.configuration.discord_profile.load_error"),
                        )),
                        Err(()) => {
                            load_error.set(Some(i18n.t("panel.configuration.network_error")))
                        }
                    }
                    loading.set(false);
                });
            }
            || ()
        });
    }

    let pick_avatar = {
        let new_avatar = new_avatar.clone();
        Callback::from(move |file: web_sys::File| {
            let new_avatar = new_avatar.clone();
            read_image(file, Callback::from(move |url| new_avatar.set(Some(url))));
        })
    };

    let pick_banner = {
        let new_banner = new_banner.clone();
        Callback::from(move |file: web_sys::File| {
            let new_banner = new_banner.clone();
            read_image(file, Callback::from(move |url| new_banner.set(Some(url))));
        })
    };

    let current_username = profile
        .as_ref()
        .map(|p| p.username.clone())
        .unwrap_or_default();
    let username_changed = !username.is_empty() && *username != current_username;
    let has_changes = username_changed || new_avatar.is_some() || new_banner.is_some();

    let save = {
        let username = username.clone();
        let new_avatar = new_avatar.clone();
        let new_banner = new_banner.clone();
        let saving = saving.clone();
        let feedback = feedback.clone();
        let profile = profile.clone();
        let i18n = i18n.clone();
        Callback::from(move |_: MouseEvent| {
            if !has_changes {
                return;
            }
            let mut payload = serde_json::Map::new();
            if username_changed {
                payload.insert(
                    "username".to_string(),
                    serde_json::Value::String((*username).clone()),
                );
            }
            if let Some(avatar) = (*new_avatar).clone() {
                payload.insert("avatar".to_string(), serde_json::Value::String(avatar));
            }
            if let Some(banner) = (*new_banner).clone() {
                payload.insert("banner".to_string(), serde_json::Value::String(banner));
            }
            let new_avatar = new_avatar.clone();
            let new_banner = new_banner.clone();
            let saving = saving.clone();
            let feedback = feedback.clone();
            let profile = profile.clone();
            let i18n = i18n.clone();
            spawn_local(async move {
                saving.set(true);
                let request =
                    Request::put("/api/bot/profile").json(&serde_json::Value::Object(payload));
                let outcome = match request {
                    Ok(req) => req.send().await.map_err(|_| ()),
                    Err(_) => Err(()),
                };
                match outcome {
                    Ok(resp) if resp.ok() => {
                        feedback.set(Some((
                            true,
                            i18n.t("panel.configuration.discord_profile.save_success"),
                        )));
                        new_avatar.set(None);
                        new_banner.set(None);
                        if let Ok(Some(loaded)) = fetch_profile().await {
                            profile.set(Some(loaded));
                        }
                    }
                    Ok(resp) => {
                        let detail = resp
                            .text()
                            .await
                            .unwrap_or_else(|_| i18n.t("panel.configuration.unknown_error"));
                        feedback.set(Some((
                            false,
                            format!(
                                "{}: {}",
                                i18n.t("panel.configuration.discord_profile.save_error"),
                                detail
                            ),
                        )));
                    }
                    Err(()) => {
                        feedback.set(Some((false, i18n.t("panel.configuration.network_error"))))
                    }
                }
                saving.set(false);
            });
        })
    };

    let body = if !props.bot_running {
        html! { <EmptyState icon={IconName::Power} title={t("bot_not_running")} /> }
    } else if *loading {
        html! { <Loading label={t("loading")} class="py-12" /> }
    } else if let Some(error) = (*load_error).clone() {
        html! { <Notice>{error}</Notice> }
    } else {
        let banner_url = (*new_banner)
            .clone()
            .or_else(|| profile.as_ref().and_then(|p| p.banner_url.clone()));
        let avatar_url = (*new_avatar)
            .clone()
            .or_else(|| profile.as_ref().and_then(|p| p.avatar_url.clone()));
        let upload_button = |size: ButtonSize| {
            classes!(
                button_classes(ButtonVariant::Secondary, size),
                "cursor-pointer"
            )
        };
        html! {
            <>
                <div
                    class="relative h-[88px] overflow-hidden rounded-xl border border-line bg-surface-raised"
                    ondragover={allow_drop()}
                    ondrop={dropped_file(pick_banner.clone())}
                >
                    if let Some(url) = banner_url {
                        <img src={url} alt="" class="absolute inset-0 size-full object-cover" />
                    }
                    <label class={classes!(upload_button(ButtonSize::Sm), "absolute", "right-3", "top-3")}>
                        <Icon name={IconName::Upload} />
                        {t("change_banner")}
                        <input type="file" accept={IMAGE_TYPES} class="hidden" onchange={picked_file(pick_banner)} />
                    </label>
                </div>
                <div class="flex flex-wrap items-start gap-4 sm:flex-nowrap">
                    <div ondragover={allow_drop()} ondrop={dropped_file(pick_avatar.clone())}>
                        <Avatar name={(*username).clone()} url={avatar_url.map(AttrValue::from)} size={64} />
                    </div>
                    <Field label={t("username")} help={t("username_help")} class="min-w-[160px] flex-1">
                        <input
                            type="text"
                            class="input"
                            value={(*username).clone()}
                            minlength="2"
                            maxlength="32"
                            oninput={{
                                let username = username.clone();
                                Callback::from(move |e: InputEvent| username.set(input_value(&e)))
                            }}
                        />
                    </Field>
                    <label class={classes!(upload_button(ButtonSize::Md), "sm:mt-[26px]")}>
                        <Icon name={IconName::Upload} />
                        {t("change_avatar")}
                        <input type="file" accept={IMAGE_TYPES} class="hidden" onchange={picked_file(pick_avatar)} />
                    </label>
                </div>
                <p class="help">{t("image_help")}</p>
                if let Some((success, message)) = (*feedback).clone() {
                    <Notice tone={if success { NoticeTone::Positive } else { NoticeTone::Negative }}>{message}</Notice>
                }
                if has_changes {
                    <div class="flex justify-end">
                        <Button variant={ButtonVariant::Primary} loading={*saving} onclick={save}>
                            {t("save_profile")}
                        </Button>
                    </div>
                }
            </>
        }
    };

    html! {
        <section class="card flex min-w-0 flex-1 flex-col gap-5 p-6">
            <CardHeader title={t("title")} description={t("description")} />
            {body}
        </section>
    }
}
