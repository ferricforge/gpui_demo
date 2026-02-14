// components

use gpui::*;
use gpui_component::StyledExt;

use crate::Quit;
use crate::quit;

pub struct Window {
    _window_close_subscription: Option<Subscription>,
    content: Option<Box<dyn Fn() -> AnyElement>>,
}

impl Window {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.on_window_closed(|cx: &mut App| {
            println!("Window closed callback!");
            quit(&Quit, cx);
        });

        println!("Window has been constructed");
        Self {
            _window_close_subscription: Some(subscription),
            content: None,
        }
    }

    /// Set a factory that produces the content to be rendered in the window.
    ///
    /// The factory is called on every render, ensuring stateless `RenderOnce`
    /// components like `Button` are reconstructed each frame.
    pub fn set_content(
        &mut self,
        content: impl Fn() -> AnyElement + 'static,
    ) {
        self.content = Some(Box::new(content));
    }
}

impl Render for Window {
    fn render(
        &mut self,
        _: &mut gpui::Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let content = self.content.as_ref().map(|f| f());

        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .children(content)
    }
}
