use anyhow::Result;
use tracing::error;
use tracing_subscriber::EnvFilter;

/// Initializes a default tracing subscriber for local development.
///
/// This setup is intentionally lightweight and no-op safe: if the host
/// application has already installed a global subscriber, this function does
/// nothing so external logging configuration can take precedence.
pub fn init_default_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,gpui_demo=debug"));

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();
}

/// Logs a background task failure with context.
pub fn log_task_error(
    task_name: &'static str,
    result: Result<()>,
) {
    if let Err(error) = result {
        error!(task = task_name, ?error, "background task failed");
    }
}
