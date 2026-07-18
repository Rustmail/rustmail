use crate::components::wizard::types::ValidationResponse;
use gloo_net::Error;
use gloo_net::http::{Request, RequestBuilder, Response};
use rustmail_types::SETUP_TOKEN_HEADER;
use serde::de::DeserializeOwned;
use yew::Callback;

const TOKEN_PARAM: &str = "token";

pub fn setup_token() -> Option<String> {
    let search = web_sys::window()?.location().search().ok()?;
    let params = web_sys::UrlSearchParams::new_with_str(&search).ok()?;
    params.get(TOKEN_PARAM)
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

/// Shared response handling for the wizard's validate-* endpoints: emits
/// `on_unauthorized` on a 401 and returns `None`, otherwise always returns
/// a `T` (parsed on success, or a `T::from_error(..)` placeholder on a
/// network/parse failure) for the caller to store as the validation result.
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
