use gpui::*;
use gpui_component::Root;

use crate::quit;
use crate::Quit;

pub struct Window {
    _window_close_subscription: Option<Subscription>,
    content: Option<AnyElement>,
}

impl Window {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.on_window_closed(|cx: &mut App| {
            println!("Window closed callback!");
            quit(&Quit, cx);
        });

        Self {
            _window_close_subscription: Some(subscription),
            content: None,
        }
    }

    /// Set the content to be rendered in the window
    pub fn set_content(&mut self, content: impl IntoElement + 'static) {
        self.content = Some(content.into_any_element());
    }
}

impl Render for Window {
    fn render(&mut self, _: &mut gpui::Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .children(self.content.take())
    }
}
