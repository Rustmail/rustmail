use reqwest::Client;

pub async fn ping_internal(
    endpoint: &str,
    token: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = Client::new();
    let url = format!("http://127.0.0.1:3002{}", endpoint);
    client
        .post(&url)
        .header("x-internal-call", token)
        .send()
        .await
}
