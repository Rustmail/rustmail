use moka::future::Cache;
use std::time::Duration;

pub fn get_admin_cache() -> &'static Cache<String, bool> {
    use std::sync::OnceLock;
    static ADMIN_CACHE: OnceLock<Cache<String, bool>> = OnceLock::new();

    ADMIN_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(1000)
            .time_to_live(Duration::from_secs(300))
            .build()
    })
}

pub fn get_permissions_cache() -> &'static Cache<(String, String), u64> {
    use std::sync::OnceLock;
    static PERMISSIONS_CACHE: OnceLock<Cache<(String, String), u64>> = OnceLock::new();

    PERMISSIONS_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(10000)
            .time_to_live(Duration::from_secs(300))
            .build()
    })
}
