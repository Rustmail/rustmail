use std::sync::Arc;
use crate::config::Config;

pub fn get_thread_lock(config: &Config, key: u64) -> Arc<tokio::sync::Mutex<()>> {
    let mut map = config.thread_locks.lock().expect("lock poisoned");
    map.entry(key)
        .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
        .clone()
}