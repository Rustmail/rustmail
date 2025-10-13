#[macro_export]
macro_rules! wrap_command {
    ($map:expr, [$($name:expr),+], $func:expr) => {{
        let command: std::sync::Arc<dyn Fn(serenity::prelude::Context, serenity::model::prelude::Message, $crate::config::Config, Receiver<bool>) -> std::pin::Pin<Box<dyn std::future::Future<Output = $crate::errors::ModmailResult<()>> + Send>> + Send + Sync + 'static> =
            std::sync::Arc::new(|ctx, msg, config, shutdown| {
                Box::pin(async move {
                    $func(&ctx, &msg, &config, Arc::new(shutdown)).await
                })
            });
        $(
            $map.insert($name.to_string(), std::sync::Arc::clone(&command));
        )+
    }};
    ($map:expr, $name:expr, $func:expr) => {{
        let command: std::sync::Arc<dyn Fn(serenity::prelude::Context, serenity::model::prelude::Message, $crate::config::Config, Receiver<bool>) -> std::pin::Pin<Box<dyn std::future::Future<Output = $crate::errors::ModmailResult<()>> + Send>> + Send + Sync + 'static> =
            std::sync::Arc::new(|ctx, msg, config, shutdown| {
                Box::pin(async move {
                    $func(&ctx, &msg, &config, Arc::new(shutdown)).await
                })
            });
        $map.insert($name.to_string(), command);
    }};
}
