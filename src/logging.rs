use anyhow::Result;
use chrono::Local;
use tracing::{error, Event, Subscriber};
use tracing_subscriber::{
    fmt::{
        format::{FormatEvent, FormatFields, Writer},
        FmtContext,
    },
    registry::LookupSpan,
    EnvFilter,
};

struct LocalFmt;

impl<S, N> FormatEvent<S, N> for LocalFmt
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        let meta = event.metadata();

        // Timestamp in local timezone with UTC offset
        write!(writer, "{} ", Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))?;

        // Log level, right-aligned to 5 chars to match tracing's default width
        write!(writer, "{:>5} ", meta.level())?;

        // File path with src/ prefix stripped, and line number
        let file = meta.file().map(|f| {
            f.strip_prefix("src/")
                .or_else(|| f.strip_prefix("src\\"))
                .unwrap_or(f)
        });
        if let (Some(file), Some(line)) = (file, meta.line()) {
            write!(writer, "{file}:{line} ")?;
        }

        // Message and any structured fields (e.g. task = "...", ?error)
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

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
        .event_format(LocalFmt)
        .try_init();
}

/// Logs a background task failure with context.
pub fn log_task_error(task_name: &'static str, result: Result<()>) {
    if let Err(error) = result {
        error!(task = task_name, ?error, "background task failed");
    }
}
