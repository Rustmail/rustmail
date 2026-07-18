use crate::components::wizard::types::WizardData;
use serde::{Deserialize, Serialize};

const STORAGE_KEY: &str = "rustmail_setup_progress";

#[derive(Serialize, Deserialize)]
struct StoredProgress {
    step: usize,
    data: WizardData,
}

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok()?
}

pub fn save_progress(step: usize, data: &WizardData) {
    let Some(storage) = storage() else {
        return;
    };
    let mut redacted = data.clone();
    redacted.token = String::new();
    redacted.client_secret = String::new();
    if let Ok(json) = serde_json::to_string(&StoredProgress {
        step,
        data: redacted,
    }) {
        let _ = storage.set_item(STORAGE_KEY, &json);
    }
}

pub fn load_progress() -> Option<(usize, WizardData)> {
    let storage = storage()?;
    let json = storage.get_item(STORAGE_KEY).ok()??;
    let stored: StoredProgress = serde_json::from_str(&json).ok()?;
    Some((stored.step, stored.data))
}

pub fn clear_progress() {
    if let Some(storage) = storage() {
        let _ = storage.remove_item(STORAGE_KEY);
    }
}
