pub mod components;
pub mod logging;
pub mod models;
pub mod platform;
pub mod preferences;

use gpui::{
    AnyElement, App, AppContext, Context, InteractiveElement, IntoElement, KeyBinding, Menu,
    MenuItem, ParentElement, Styled, Window, actions,
};
use gpui_component::{h_flex, v_flex};
use tracing::{info, warn};

use crate::components::{FileSelectionForm, make_button};
#[cfg(target_os = "linux")]
use crate::platform::apply_linux_system_theme;
#[cfg(target_os = "macos")]
use crate::platform::apply_macos_system_theme;

actions!(gpui_demo, [Quit]);

// Takes a reference to the action (often unused) and mutable app context
pub fn quit(
    _: &Quit,
    cx: &mut App,
) {
    info!("Executing quit handler");
    cx.quit();
}

pub fn setup_app(app_cx: &mut App) {
    // This must be called before using any GPUI Component features.
    gpui_component::init(app_cx);

    #[cfg(target_os = "macos")]
    apply_macos_system_theme(app_cx);
    #[cfg(target_os = "linux")]
    apply_linux_system_theme(app_cx);

    app_cx.activate(true);

    // Bind platform-appropriate quit shortcut
    #[cfg(target_os = "macos")]
    app_cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

    #[cfg(not(target_os = "macos"))]
    app_cx.bind_keys([
        KeyBinding::new("ctrl-q", Quit, None),
        KeyBinding::new("alt-F4", Quit, None),
    ]);

    // Register the quit action handler
    app_cx.on_action(quit);

    // Set up the application menu with Quit
    app_cx.set_menus(vec![
        Menu {
            name: "TimeKeeper Loader".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        },
    ]);
}

/// Builds the primary window content.
///
/// Returns a closure suitable for passing to `Window::set_content`,
/// producing a styled "Click Me!" button on each render frame.
pub fn build_main_content(
    window: &mut Window,
    app_cx: &mut App,
) -> impl Fn() -> AnyElement + 'static {
    let form = app_cx
        .new(|form_cx: &mut Context<FileSelectionForm>| FileSelectionForm::new(window, form_cx));

    move || {
        v_flex()
            .size_full()
            .p_5()
            .gap_4()
            .child(form.clone())
            .child(
                h_flex()
                    .id("window-body")
                    .p_1()
                    .gap_4()
                    .items_center()
                    .justify_center()
                    .child({
                        let form_handle = form.clone();
                        make_button("ok-go", "Convert Files", move |_, _, cx: &mut App| {
                            let form_model = form_handle.read(cx).to_model(cx);
                            match form_model.validate_for_submit() {
                                Ok(()) => {
                                    // Apply level first so subsequent calls in this session use it.
                                    if let Err(e) = logging::set_log_level(&form_model.log_level.to_label()) {
                                        warn!("Could not apply log level: {e}");
                                    }

                                    // Wire up file logging if a directory was provided.
                                    if !form_model.log_directory.as_os_str().is_empty() {
                                        // Use a timestamped name so runs don't overwrite each other.
                                        let filename = format!(
                                            "conversion_{}.log",
                                            chrono::Local::now().format("%Y-%m-%d_%H-%M-%S")
                                        );
                                        let log_path = form_model.log_directory.join(filename);
                                        if let Err(e) = logging::enable_file_logging(&log_path) {
                                            warn!("Could not open log file: {e}");
                                        }
                                    }

                                    // Honor the user's stdout preference.
                                    if let Err(e) = logging::set_stdout_enabled(form_model.log_stdout) {
                                        warn!("Could not configure stdout logging: {e}");
                                    }

                                    info!(%form_model, "Form validated");
                                    // Next step: pass validated model to the processing crate.
                                }
                                Err(errors) => {
                                    warn!("Cannot submit form due to validation errors");
                                    for error in errors {
                                        warn!(%error, "validation error");
                                    }
                                }
                            }
                        })
                    })
                    .child({
                        let form_handle = form.clone();
                        make_button(
                            "load-sheets",
                            "Load Sheets",
                            move |_, window, cx: &mut App| {
                                let form_model = form_handle.read(cx).to_model(cx);
                                let sheets = form_handle.read(cx).load_sheet_options(cx);
                                info!(
                                    source_file = %form_model.source_file.display(),
                                    sheet_count = sheets.len(),
                                    "Loaded sheet options"
                                );
                                form_handle.update(cx, |form, form_cx| {
                                    form.set_sheet_options(sheets, window, form_cx);
                                });
                            },
                        )
                    }),
            )
            .into_any_element()
    }
}
