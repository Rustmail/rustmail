use gloo_net::http::{Request, RequestBuilder};
use rustmail_types::SETUP_TOKEN_HEADER;

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
