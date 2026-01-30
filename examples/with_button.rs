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
            let _window_handle = cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Point::default(),
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
