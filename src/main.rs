use gpui::*;
use gpui_component::Root;
use gpui_demo::{components::Window, preferences::WindowPreferences, setup_app};

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
                    let view = window_cx.new(|view_cx: &mut Context<Window>| Window::new(view_cx));
                    window_cx.new(|root_cx| Root::new(view, window, root_cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
