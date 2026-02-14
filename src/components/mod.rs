// components

pub mod registration;
pub mod window;

use gpui_component::Sizable;
pub use registration::RegistrationForm;
pub use window::AppWindow;

use gpui::{App, SharedString, Window};
use gpui::{ClickEvent, Styled, px};
use gpui_component::button::{Button, ButtonVariants};

/// Creates a primary-styled button with a custom click handler.
pub fn make_button(
    id: impl Into<SharedString>,
    label: impl Into<SharedString>,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Button {
    Button::new(id.into())
        .primary()
        .large()
        .w(px(140.)) // ← fixed width
        .label(label.into())
        .on_click(on_click)
}
