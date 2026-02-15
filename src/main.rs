use gpui::{
    App, AppContext, Application, Bounds, Context, TitlebarOptions, WindowBounds, WindowHandle,
    WindowOptions,
};

use gpui_component::Root;
use gpui_component_assets::Assets;
use gpui_demo::build_main_content;
use gpui_demo::{components::AppWindow as MainWindow, preferences::WindowPreferences, setup_app};

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(move |app_cx: &mut App| {
        setup_app(app_cx);

        let prefs = WindowPreferences::default();

        let titlebar = Some(TitlebarOptions {
            title: Some("TimeKeeper".into()),
            appears_transparent: false,
            ..Default::default()
        });

        app_cx
            .spawn(async move |async_cx| {
                let bounds = async_cx
                    .update(|app_cx: &mut App| Bounds::centered(None, prefs.size, app_cx))?;

                let _window_handle: WindowHandle<Root> = async_cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        titlebar,
                        ..Default::default()
                    },
                    |window: &mut gpui::Window, window_cx| {
                        let view = window_cx.new(|view_cx: &mut Context<MainWindow>| {
                            let content = build_main_content(window, view_cx);
                            let mut main_window = MainWindow::new(view_cx);
                            main_window.set_content(content);
                            main_window
                        });
                        window_cx.new(|root_cx| Root::new(view, window, root_cx))
                    },
                )?;

                Ok::<_, anyhow::Error>(())
            })
            .detach();
    });
}
