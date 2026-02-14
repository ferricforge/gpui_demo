pub mod components;
pub mod preferences;

#[cfg(target_os = "macos")]
use gpui::KeyBinding;
use gpui::{
    AnyElement, App, AppContext, Context, Entity, InteractiveElement, IntoElement, Menu, MenuItem,
    ParentElement, Render, Styled, TextAlign, Window, actions, div, px,
};
use gpui_component::input::{Input, InputState};
use gpui_component::{TitleBar, h_flex, v_flex};

use crate::components::{RegistrationForm, make_button};

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
            name: "App".into(),
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

    let register = app_cx
        .new(|form_cx: &mut Context<RegistrationForm>| RegistrationForm::new(window, form_cx));

    move || {
        v_flex()
            .size_full()
            .p_5()
            .gap_1()
            .child(
                TitleBar::new().child(
                    h_flex()
                        .w_full()
                        .pl_0()
                        .pr_2()
                        .justify_between()
                        .child("App with Custom title bar")
                        .child("Right Item"),
                ),
            )
            .child(
                h_flex()
                    .id("window-body")
                    .p_5()
                    .gap_4()
                    .size_full()
                    .items_center()
                    .justify_center()
                    .child("Hello, World!")
                    .child(make_button("ok-go", "Let's Go!", |_, _, _| {
                        println!("I've been CLICKED! 😫")
                    })),
            )
            .child(register.clone())
            .child(form.clone())
            .into_any_element()
    }
}

pub struct FileSelectionForm {
    source_file: Entity<InputState>,
    database_file: Entity<InputState>,
}

impl FileSelectionForm {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let source_file = cx.new(|closure_cx| {
            InputState::new(window, closure_cx).placeholder("Source file path...")
        });
        let database_file = cx.new(|closure_cx| {
            InputState::new(window, closure_cx).placeholder("Database file path...")
        });

        Self {
            source_file,
            database_file,
        }
    }
}

impl Render for FileSelectionForm {
    fn render(
        &mut self,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .gap(px(2.))
            .child(
                h_flex()
                    .items_center()
                    .gap_5()
                    .p(px(2.))
                    .rounded_md()
                    .border_1()
                    .child(
                        div()
                            .min_w(px(100.)) // keeps rows aligned
                            .text_align(TextAlign::Right)
                            .child("Source File:"),
                    )
                    .child(
                        Input::new(&self.source_file).flex_grow(), // input expands
                    )
                    .child(make_button("source-select", "Select File", |_, _, _| {
                        println!("I've been CLICKED! 😫")
                    })),
            )
            .child(
                h_flex()
                    .items_center()
                    .gap_5()
                    .p(px(2.))
                    .rounded_md()
                    .border_1()
                    .child(
                        div()
                            .min_w(px(100.))
                            .text_align(TextAlign::Right)
                            .child("Database:"),
                    )
                    .child(
                        Input::new(&self.database_file).flex_grow(), // Input expands
                    )
                    .child(make_button("db-select", "Select Database", |_, _, _| {
                        println!("I've been CLICKED! 😫")
                    })),
            )
    }
}
