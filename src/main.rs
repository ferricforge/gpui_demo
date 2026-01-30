use gpui::*;
use gpui_component::*;
use gpui_demo::{components::Window, preferences::WindowPreferences, setup_app};

fn main() {
    let app = Application::new();

    app.run(move |cx: &mut App| {
        setup_app(cx);

        let prefs = WindowPreferences::default();

        cx.spawn(async move |cx| {
            let _window_handle = cx.open_window(
                WindowOptions {
                    bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Default::default(),
                        size: Size {
                            width: prefs.width.into(),
                            height: prefs.height.into(),
                        },
                    })),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Window::new(cx));
                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
