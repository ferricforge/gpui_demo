use gpui::*;
use gpui_component::{button::*, Root};
use gpui_demo::{components::Window, preferences::WindowPreferences, setup_app};

fn click_handler(_event: &ClickEvent, _window: &mut gpui::Window, _cx: &mut App) {
    println!("Clicked!");
}

fn main() {
    let app = Application::new();

    app.run(move |cx: &mut App| {
        setup_app(cx);

        let prefs = WindowPreferences::default();

        cx.spawn(async move |cx| {
            // Get the primary display to calculate centered position
            let displays = cx.update(|cx| cx.displays())?;
            let primary_display = displays
                .first()
                .ok_or_else(|| anyhow::anyhow!("No display found"))?;

            let origin = if prefs.center_on_open {
                prefs.calculate_centered_origin(primary_display)
            } else {
                Point::default()
            };

            let _window_handle = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin,
                        size: prefs.size,
                    })),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| {
                        let mut win = Window::new(cx);

                        // Add content to the window
                        win.set_content(
                            div()
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
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
