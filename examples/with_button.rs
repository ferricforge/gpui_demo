use gpui::*;
use gpui_component::{button::*, Root, StyledExt};
use gpui_demo::{components::Window, preferences::WindowPreferences, setup_app};

fn click_handler(_event: &ClickEvent, _window: &mut gpui::Window, _cx: &mut App) {
    println!("Clicked!");
}

fn main() {
    let app = Application::new();

    app.run(move |app_cx: &mut App| {
        setup_app(app_cx);

        let prefs = WindowPreferences::default();

        app_cx.spawn(async move |async_cx| {
            let bounds = async_cx.update(|app_cx: &mut App| {
                Bounds::centered(None, prefs.size, app_cx)
            })?;

            let _window_handle = async_cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window: &mut gpui::Window, window_cx| {
                    let view = window_cx.new(|view_cx: &mut Context<Window>| {
                        let mut win = Window::new(view_cx);

                        // Add content to the window
                        win.set_content(
                            div()
                                .v_flex()
                                .gap_2()
                                .child("Hello, World!")
                                .child(
                                    Button::new("ok")
                                        .primary()
                                        .label("Let's Go!")
                                        .on_click(click_handler),
                                ),
                        );

                        win
                    });
                    window_cx.new(|root_cx| Root::new(view, window, root_cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
