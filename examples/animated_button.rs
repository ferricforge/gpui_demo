use gpui::*;
use gpui_component::{button::*, *};

// 1. Define your App/Window struct
pub struct AnimatedButtonDemo;

impl Render for AnimatedButtonDemo {
    fn render(
        &mut self,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .justify_center()
            .items_center()
            .size_full()
            .bg(rgb(0x1e1e1e)) // Dark background
            .child(
                // 2. Create the Button Component
                Button::new("animate_btn")
                    .primary() // Use primary theme color
                    .label("Hover & Click Me")
                    .on_click(|_, _, _| println!("Button clicked!")),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(|cx: &mut App| {
        // Initialize components and theme
        gpui_component::init(cx);

        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_| AnimatedButtonDemo)
        })
        .unwrap();
    });
}
