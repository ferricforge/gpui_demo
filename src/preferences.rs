#[derive(Debug, Clone, Copy)]
pub struct WindowPreferences {
    pub width: f64,
    pub height: f64,
}

impl Default for WindowPreferences {
    fn default() -> Self {
        Self {
            width: 1024.0,
            height: 768.0,
        }
    }
}

impl WindowPreferences {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}
