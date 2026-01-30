use gpui::{px, Pixels, Point, Size};

#[derive(Debug, Clone, Copy)]
pub struct WindowPreferences {
    pub size: Size<Pixels>,
    pub center_on_open: bool,
}

impl Default for WindowPreferences {
    fn default() -> Self {
        Self {
            size: Size {
                width: px(1024.0),
                height: px(768.0),
            },
            center_on_open: true,
        }
    }
}

impl WindowPreferences {
    pub fn new(width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        Self {
            size: Size {
                width: width.into(),
                height: height.into(),
            },
            center_on_open: true,
        }
    }

    pub fn with_centered(mut self, center: bool) -> Self {
        self.center_on_open = center;
        self
    }

    /// Calculate the centered position for the window on the given display
    pub fn calculate_centered_origin(&self, display: &gpui::Display) -> Point<Pixels> {
        let display_bounds = display.bounds();
        Point {
            x: display_bounds.center().x - self.size.width / 2.0,
            y: display_bounds.center().y - self.size.height / 2.0,
        }
    }
}
