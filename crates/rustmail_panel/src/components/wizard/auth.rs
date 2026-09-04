use crate::components::wizard::types::ValidationResponse;
use gloo_net::Error;
use gloo_net::http::{Request, RequestBuilder, Response};
use rustmail_types::SETUP_TOKEN_HEADER;
use serde::de::DeserializeOwned;
use std::cell::OnceCell;
use yew::Callback;

const TOKEN_PARAM: &str = "token";
const STORAGE_KEY: &str = "rustmail_setup_token";

thread_local! {
    static SETUP_TOKEN: OnceCell<Option<String>> = const { OnceCell::new() };
}

pub fn setup_token() -> Option<String> {
    SETUP_TOKEN.with(|cell| cell.get_or_init(resolve_token).clone())
}

fn session_storage() -> Option<web_sys::Storage> {
    web_sys::window()?.session_storage().ok()?
}

fn resolve_token() -> Option<String> {
    match take_token_from_url() {
        Some(token) => {
            if let Some(storage) = session_storage() {
                let _ = storage.set_item(STORAGE_KEY, &token);
            }
            Some(token)
        }
        None => session_storage()?.get_item(STORAGE_KEY).ok()?,
    }
}

fn take_token_from_url() -> Option<String> {
    let window = web_sys::window()?;
    let location = window.location();
    let search = location.search().ok()?;
    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    let token = params.get(TOKEN_PARAM)?;

    params.delete(TOKEN_PARAM);
    let remaining = params.to_string().as_string().unwrap_or_default();
    let path = location.pathname().unwrap_or_default();
    let hash = location.hash().unwrap_or_default();
    let new_url = if remaining.is_empty() {
        format!("{path}{hash}")
    } else {
        format!("{path}?{remaining}{hash}")
    };

    if let Ok(history) = window.history() {
        let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
    }

    Some(token)
}

fn with_token(mut builder: RequestBuilder) -> RequestBuilder {
    if let Some(token) = setup_token() {
        builder = builder.header(SETUP_TOKEN_HEADER, &token);
    }
    builder
}

pub fn authed_post(url: &str) -> RequestBuilder {
    with_token(Request::post(url))
}

pub fn authed_get(url: &str) -> RequestBuilder {
    with_token(Request::get(url))
}

pub async fn handle_validation_response<T>(
    res: Result<Response, Error>,
    on_unauthorized: &Callback<()>,
) -> Option<T>
where
    T: DeserializeOwned + ValidationResponse,
{
    match res {
        Ok(resp) if resp.status() == 401 => {
            on_unauthorized.emit(());
            None
        }
        Ok(resp) => match resp.json::<T>().await {
            Ok(data) => Some(data),
            Err(_) => Some(T::from_error("Invalid response from server")),
        },
        Err(_) => Some(T::from_error("Network error")),
    }
}
