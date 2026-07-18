use rustmail_types::{CommandConfig, LanguageConfig, ServerMode, ThreadConfig};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Sender;

#[derive(Debug, Clone, PartialEq)]
pub enum SetupStep {
    Token,
    ServerMode,
    ThreadConfig,
    PanelConfig,
    Language,
    Review,
}

#[derive(Debug, Clone, Default)]
pub struct PartialConfig {
    pub token: Option<String>,
    pub bot_id: Option<u64>,
    pub bot_username: Option<String>,
    pub bot_avatar: Option<String>,
    pub server_mode: Option<ServerMode>,
    pub bot_status: Option<String>,
    pub welcome_message: Option<String>,
    pub close_message: Option<String>,
    pub typing_proxy_from_user: Option<bool>,
    pub typing_proxy_from_staff: Option<bool>,
    pub thread: Option<ThreadConfig>,
    pub command: Option<CommandConfig>,
    pub enable_panel: Option<bool>,
    pub client_id: Option<u64>,
    pub client_secret: Option<String>,
    pub redirect_url: Option<String>,
    pub panel_super_admin_users: Option<Vec<u64>>,
    pub language: Option<LanguageConfig>,
    pub enable_rustmail_logs: Option<bool>,
    pub enable_discord_logs: Option<bool>,
    pub logs_channel_id: Option<u64>,
    pub enable_features: Option<bool>,
    pub features_channel_id: Option<u64>,
}

#[derive(Debug)]
pub struct SetupState {
    pub step: SetupStep,
    pub config: PartialConfig,
    pub shutdown_tx: Option<Sender<()>>,
    pub token: String,
    pub panel_url: Option<String>,
    pub api_port: Option<u16>,
}

impl SetupState {
    pub fn new() -> Self {
        Self {
            step: SetupStep::Token,
            config: PartialConfig::default(),
            shutdown_tx: None,
            token: uuid::Uuid::new_v4().simple().to_string(),
            panel_url: None,
            api_port: None,
        }
    }
}

pub type SharedSetupState = Arc<Mutex<SetupState>>;

pub fn new_setup_state() -> SharedSetupState {
    Arc::new(Mutex::new(SetupState::new()))
}
