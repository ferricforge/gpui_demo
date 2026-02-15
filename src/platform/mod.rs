#[cfg(target_os = "macos")]
pub mod macos_theme;

#[cfg(target_os = "macos")]
pub use macos_theme::apply_macos_system_theme;