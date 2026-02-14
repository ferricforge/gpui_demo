// preferences

use gpui::{Pixels, Size, px};

#[derive(Debug, Clone, Copy)]
pub struct WindowPreferences {
    pub size: Size<Pixels>,
    // TODO: Implement window centering once we determine the correct
    // gpui API for getting display bounds
}

impl Default for WindowPreferences {
    fn default() -> Self {
        Self {
            size: Size {
                width: px(800.0),
                height: px(600.0),
            },
        }
    }
}

impl WindowPreferences {
    pub fn new(
        width: impl Into<Pixels>,
        height: impl Into<Pixels>,
    ) -> Self {
        Self {
            size: Size {
                width: width.into(),
                height: height.into(),
            },
        }
    }
}
