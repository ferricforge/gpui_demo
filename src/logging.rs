use anyhow::Result;
use chrono::Local;
use std::{
    io::IsTerminal,
    path::Path,
    sync::{Mutex, OnceLock},
};
use tracing::{error, Event, Level, Subscriber};
use tracing_subscriber::{
    fmt::{
        format::{FormatEvent, FormatFields, Writer},
        FmtContext,
    },
    layer::SubscriberExt,
    registry::LookupSpan,
    reload,
    util::SubscriberInitExt,
    EnvFilter, Layer,  // <-- Layer added here; fixes the with_filter compile error
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

        if ansi { write!(writer, "\x1b[2m")? }
        write!(writer, "{} ", Local::now().format("%Y-%m-%dT%H:%M:%S%.6f%:z"))?;
        if ansi { write!(writer, "\x1b[0m")? }

        let (pre, post) = if ansi {
            match *meta.level() {
                Level::ERROR => ("\x1b[1;31m", "\x1b[0m"),
                Level::WARN  => ("\x1b[1;33m", "\x1b[0m"),
                Level::INFO  => ("\x1b[1;32m", "\x1b[0m"),
                Level::DEBUG => ("\x1b[1;34m", "\x1b[0m"),
                Level::TRACE => ("\x1b[1;35m", "\x1b[0m"),
            }
        } else {
            ("", "")
        };
        write!(writer, "{}{:>5}{} ", pre, meta.level(), post)?;

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

        ctx.field_format().format_fields(writer.by_ref(), event)?;
        writeln!(writer)
    }
}

fn make_filter() -> EnvFilter {
    EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,gpui_demo=debug"))
}

// Box the setter so the complex reload::Handle<EnvFilter, S> type
// never leaks out of this module — callers just use set_log_level().
type SetLevelFn = Box<dyn Fn(&str) -> Result<()> + Send + Sync>;
static SET_LOG_LEVEL: OnceLock<SetLevelFn> = OnceLock::new();

/// Changes the active log filter at runtime.
///
/// Accepts a bare level name ("error", "warn", "info", "debug", "trace")
/// or any full `EnvFilter` directive (e.g. `"info,gpui_demo=debug"`).
/// Level names are case-insensitive.
pub fn set_log_level(level: &str) -> Result<()> {
    match SET_LOG_LEVEL.get() {
        Some(f) => f(level),
        None => anyhow::bail!("logging not yet initialized"),
    }
}

// Captures the handle in a closure and stores it, erasing the S type.
// Called once after try_init() succeeds.
fn store_handle<S>(handle: reload::Handle<EnvFilter, S>)
where
    S: Subscriber + Send + Sync + 'static,
{
    let _ = SET_LOG_LEVEL.set(Box::new(move |level_str: &str| {
        let filter = EnvFilter::try_new(level_str)
            .map_err(|e| anyhow::anyhow!("invalid log level '{level_str}': {e}"))?;
        handle
            .reload(filter)
            .map_err(|e| anyhow::anyhow!("filter reload failed: {e}"))
    }));
}

/// Initializes logging to stdout only.
pub fn init_default_logging() {
    // The filter lives at the registry level so one handle covers all layers.
    let (filter_layer, handle) = reload::Layer::new(make_filter());

    let stdout_layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(std::io::stdout().is_terminal());

    if tracing_subscriber::registry()
        .with(filter_layer)
        .with(stdout_layer)
        .try_init()
        .is_ok()
    {
        store_handle(handle);
    }
}

/// Initializes logging to stdout and a file simultaneously.
///
/// Stdout receives ANSI colors when attached to a terminal.
/// The file always receives plain text.
pub fn init_logging_with_file(log_path: &Path) -> Result<()> {
    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;

    let (filter_layer, handle) = reload::Layer::new(make_filter());

    let stdout_layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(std::io::stdout().is_terminal());

    let file_layer = tracing_subscriber::fmt::layer()
        .event_format(LocalFmt)
        .with_ansi(false)
        .with_writer(Mutex::new(log_file));

    tracing_subscriber::registry()
        .with(filter_layer)  // one filter, controls both layers below
        .with(stdout_layer)
        .with(file_layer)
        .try_init()?;

    store_handle(handle);
    Ok(())
}

/// Logs a background task failure with context.
pub fn log_task_error(task_name: &'static str, result: Result<()>) {
    if let Err(error) = result {
        error!(task = task_name, ?error, "background task failed");
    }
}
