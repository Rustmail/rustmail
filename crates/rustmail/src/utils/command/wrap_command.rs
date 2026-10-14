/// Registers a text command function under one or more names: `wrap_command!(registry, ["close", "c"], close)`.
#[macro_export]
macro_rules! wrap_command {
    ($registry:expr, [$($name:expr),+], $func:expr) => {{
        let command: $crate::commands::TextCommandFunc =
            std::sync::Arc::new(|ctx, msg, config, handler| {
                Box::pin(async move { $func(ctx, msg, config, handler).await })
            });
        $registry.register_legacy_text_command(&[$($name),+], command);
    }};
    ($registry:expr, $name:expr, $func:expr) => {{
        wrap_command!($registry, [$name], $func);
    }};
}
