use serde::{Deserialize, Serialize};

#[derive(Clone, Default, PartialEq)]
pub struct WizardData {
    pub token: String,
    // Add other fields later for next steps
}

#[derive(Serialize)]
pub struct ValidateTokenRequest {
    pub token: String,
}

#[derive(Deserialize, Clone)]
pub struct BotInfo {
    pub id: String,
    pub username: String,
    pub avatar: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct ValidateTokenResponse {
    pub valid: bool,
    pub bot: Option<BotInfo>,
    pub error: Option<String>,
}
