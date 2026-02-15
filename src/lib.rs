pub mod components;
pub mod models;
pub mod platform;
pub mod preferences;

use gpui::{
    AnyElement, App, AppContext, Context, InteractiveElement, IntoElement, KeyBinding, Menu,
    MenuItem, ParentElement, Styled, Window, actions,
};
use gpui_component::{TitleBar, h_flex, v_flex};

use crate::components::{FileSelectionForm, make_button};
#[cfg(target_os = "macos")]
use crate::platform::apply_macos_system_theme;

actions!(gpui_demo, [Quit]);

// Takes a reference to the action (often unused) and mutable app context
pub fn quit(
    _: &Quit,
    cx: &mut App,
) {
    println!("Executing the Quit handler");
    cx.quit();
}

pub fn setup_app(app_cx: &mut App) {
    // This must be called before using any GPUI Component features.
    gpui_component::init(app_cx);

    #[cfg(target_os = "macos")]
    apply_macos_system_theme(app_cx);

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
            .child(TitleBar::new().child(h_flex().w_full().child("TimeKeeper Loader")))
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
                                    println!("Form data is:\n{form_model}");
                                    // Next step: pass validated model to the processing crate.
                                }
                                Err(errors) => {
                                    println!("Cannot submit form due to validation errors:");
                                    for error in errors {
                                        println!("- {error}");
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
                                println!("Form data is:\n{form_model}");
                                let sheets = form_handle.read(cx).load_sheet_options(cx);
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
