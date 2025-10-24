#[macro_export]
macro_rules! wrap_command {
    ($map:expr, [$($name:expr),+], $func:expr) => {{
        let command: std::sync::Arc<
            dyn for<'a> Fn(
                serenity::prelude::Context,
                serenity::model::prelude::Message,
                &'a $crate::config::Config,
                Arc<GuildMessagesHandler>,
            ) -> std::pin::Pin<
                Box<
                    dyn std::future::Future<Output = $crate::errors::ModmailResult<()>> + Send + 'a
                >
            > + Send + Sync + 'static
        > = std::sync::Arc::new(|ctx, msg, config, handler| {
            Box::pin(async move {
                $func(ctx, msg, config, handler).await
            })
        });
        $(
            $map.insert($name.to_string(), std::sync::Arc::clone(&command));
        )+
    }};
    ($map:expr, $name:expr, $func:expr) => {{
        wrap_command!($map, [$name], $func);
    }};
}
