use anyhow::Result;
use chrono::Local;
use tracing::{error, Event, Level, Subscriber};
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
        let ansi = writer.has_ansi_escapes();

        // Timestamp in local timezone, dimmed
        if ansi { write!(writer, "\x1b[2m")? }
        write!(writer, "{} ", Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))?;
        if ansi { write!(writer, "\x1b[0m")? }

        // Level, colored by severity
        let (pre, post) = if ansi {
            match *meta.level() {
                Level::ERROR => ("\x1b[1;31m", "\x1b[0m"), // bold red
                Level::WARN  => ("\x1b[1;33m", "\x1b[0m"), // bold yellow
                Level::INFO  => ("\x1b[1;32m", "\x1b[0m"), // bold green
                Level::DEBUG => ("\x1b[1;34m", "\x1b[0m"), // bold blue
                Level::TRACE => ("\x1b[1;35m", "\x1b[0m"), // bold magenta
            }
        } else {
            ("", "")
        };
        write!(writer, "{}{:>5}{} ", pre, meta.level(), post)?;

        // File path with src/ stripped, and line number, in cyan
        let file = meta.file().map(|f| {
            f.strip_prefix("src/")
                .or_else(|| f.strip_prefix("src\\"))
                .unwrap_or(f)
        });
        if let (Some(file), Some(line)) = (file, meta.line()) {
            if ansi {
                write!(writer, "\x1b[36m{file}:{line}\x1b[0m ")?;
            } else {
                write!(writer, "{file}:{line} ")?;
            }
        }

        // Message and structured fields (e.g. task = "...", ?error)
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
