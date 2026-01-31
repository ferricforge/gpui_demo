use gpui::*;
use gpui_component::{
    button::*,
    input::{Input, InputState},
    Root,
    StyledExt,
};
use gpui_demo::{preferences::WindowPreferences, quit, setup_app, Quit};

struct ButtonExample {
    input_state: Entity<InputState>,
    _window_close_subscription: Option<Subscription>,
}

impl ButtonExample {
    fn new(view_cx: &mut Context<Self>) -> Self {
        let subscription = view_cx.on_window_closed(|app_cx: &mut App| {
            println!("Window closed callback!");
            quit(&Quit, app_cx);
        });

        let input_state = view_cx.new(|_| InputState::new());

        Self {
            input_state,
            _window_close_subscription: Some(subscription),
        }
    }

    fn clear_input(
        &mut self,
        _: &ClickEvent,
        window: &mut gpui::Window,
        view_cx: &mut Context<Self>,
    ) {
        println!("Clearing input!");
        self.input_state.update(view_cx, |input: &mut InputState, input_cx| {
            input.set_text("", window, input_cx);
        });
    }
}

impl Render for ButtonExample {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        view_cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .v_flex()
            .gap_4()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .items_center()
                    .child("Enter text:")
                    .child(Input::new(self.input_state.clone())),
            )
            .child(
                Button::new("clear")
                    .primary()
                    .label("Clear")
                    .on_click(view_cx.listener(Self::clear_input)),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |app_cx: &mut App| {
        setup_app(app_cx);

        let prefs = WindowPreferences::default();

        app_cx.spawn(async move |async_cx| {
            let bounds =
                async_cx.update(|app_cx: &mut App| Bounds::centered(None, prefs.size, app_cx))?;

            let _window_handle = async_cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window: &mut gpui::Window, window_cx| {
                    let view = window_cx
                        .new(|view_cx: &mut Context<ButtonExample>| ButtonExample::new(view_cx));
                    window_cx.new(|root_cx| Root::new(view, window, root_cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
