#[macro_export]
macro_rules! wrap_command {
    ($map:expr, [$($name:expr),+], $func:expr) => {{
        let command: std::sync::Arc<dyn Fn(serenity::prelude::Context, serenity::model::prelude::Message, $crate::config::Config) -> std::pin::Pin<Box<dyn std::future::Future<Output = $crate::errors::ModmailResult<()>> + Send>> + Send + Sync + 'static> =
            std::sync::Arc::new(|ctx, msg, config| {
                Box::pin(async move {
                    $func(&ctx, &msg, &config).await
                })
            });
        $(
            $map.insert($name.to_string(), std::sync::Arc::clone(&command));
        )+
    }};
    ($map:expr, $name:expr, $func:expr) => {{
        let command: std::sync::Arc<dyn Fn(serenity::prelude::Context, serenity::model::prelude::Message, $crate::config::Config) -> std::pin::Pin<Box<dyn std::future::Future<Output = $crate::errors::ModmailResult<()>> + Send>> + Send + Sync + 'static> =
            std::sync::Arc::new(|ctx, msg, config| {
                Box::pin(async move {
                    $func(&ctx, &msg, &config).await
                })
            });
        $map.insert($name.to_string(), command);
    }};
}
