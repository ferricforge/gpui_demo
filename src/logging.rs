use anyhow::Result;
use chrono::Local;
use std::{io::IsTerminal, path::Path, sync::Mutex};
use tracing::{error, Event, Level, Subscriber};
use tracing_subscriber::{
    EnvFilter, Layer, fmt::{
        FmtContext, format::{FormatEvent, FormatFields, Writer}
    }, layer::SubscriberExt, registry::LookupSpan, util::SubscriberInitExt
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
        // has_ansi_escapes() is now reliable because we set with_ansi()
        // explicitly on every layer below — it never falls back to a guess.
        let ansi = writer.has_ansi_escapes();

        // Timestamp — dimmed
        if ansi { write!(writer, "\x1b[2m")? }
        write!(writer, "{} ", Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))?;
        if ansi { write!(writer, "\x1b[0m")? }

        // Level — bold + color by severity
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

        // File and line — cyan
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

        // Message and any structured fields
        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn make_filter() -> EnvFilter {
    EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,gpui_demo=debug"))
}

/// Initializes logging to stdout only.
///
/// ANSI colors are enabled only when stdout is a real terminal, so piping
/// or redirecting (`> file.log`) automatically produces plain text.
pub fn init_default_logging() {
    let layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(std::io::stdout().is_terminal()) // explicit — no guessing
        .with_filter(make_filter());

    let _ = tracing_subscriber::registry()
        .with(layer)
        .try_init();
}

/// Initializes logging to stdout **and** a file simultaneously.
///
/// Stdout receives ANSI colors when it is a terminal. The file always
/// receives plain text — no escape codes, readable in any editor.
pub fn init_logging_with_file(log_path: &Path) -> Result<()> {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let stdout_layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(std::io::stdout().is_terminal())
        .with_filter(make_filter());

    // Mutex<File> satisfies MakeWriter; tracing-subscriber has a built-in
    // impl for it. File alone is Send but not Sync, so the Mutex is required.
    let file_layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(false)
        .with_writer(Mutex::new(log_file))
        .with_filter(make_filter());

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(file_layer)
        .try_init()?;

    Ok(())
}

/// Logs a background task failure with context.
pub fn log_task_error(task_name: &'static str, result: Result<()>) {
    if let Err(error) = result {
        error!(task = task_name, ?error, "background task failed");
    }
}
