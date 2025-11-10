// GPUI Biorhythms Application
// ===========================
// This application calculates and displays biorhythm cycles based on a user's birthdate.
//
// TUTORIAL: GPUI Fundamentals
// ---------------------------
// GPUI is a GPU-accelerated UI framework for Rust that combines immediate and retained mode rendering.
// Key concepts demonstrated in this application:
//
// 1. ENTITIES & VIEWS: Structs that implement `Render` trait become views
// 2. CONTEXT: `Context<T>` provides access to app state and entity-specific methods
// 3. ELEMENTS: Building blocks of UI that implement `IntoElement` trait
// 4. RENDERING: The `render()` method describes what the UI should look like each frame
// 5. WINDOW MANAGEMENT: Creating and managing multiple windows with WindowHandle
// 6. EVENT HANDLING: Mouse, keyboard, and action handlers for interactivity
// 7. ADAPTIVE THEMING: Platform detection and native styling
// 8. MACROS: Simplifying repetitive UI code

/* Cargo.toml:

[package]
name = "gpui"
version = "0.1.0"
edition = "2021"

[dependencies]
gpui = "0.2"
native-dialog = "0.7"

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
objc = "0.2"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.58", features = ["Win32_UI_WindowsAndMessaging", "Win32_Graphics_Dwm", "Win32_System_Registry", "Win32_Foundation"] }

[target.'cfg(target_os = "linux")'.dependencies]
gtk4 = "0.10"

*/

use gpui::prelude::*; // Import common GPUI traits like Render, IntoElement
use gpui::*; // Import GPUI types and functions
use std::time::Instant; // Used for tracking cursor blink timing

// =============================================================================
// PLATFORM DETECTION & THEMING
// =============================================================================
//
// TUTORIAL: Adaptive Theming
// --------------------------
// Real applications need to feel native on each platform. This means detecting
// the OS and applying appropriate colors, spacing, and visual metaphors.
//
// We use Rust's conditional compilation and runtime platform detection to
// adapt our UI automatically.

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
enum Platform {
    MacOS,
    Windows,
    Linux,
}

impl Platform {
    fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "windows")]
        return Platform::Windows;

        #[cfg(target_os = "linux")]
        return Platform::Linux;
    }
}

// Platform-specific theme colors
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Theme {
    // Window chrome
    titlebar_bg: Hsla,
    titlebar_border: Hsla,
    titlebar_height: f32,

    // Traffic lights (macOS) or window controls
    close_button_bg: Hsla,
    close_button_border: Hsla,
    minimize_button_bg: Hsla,
    minimize_button_border: Hsla,
    maximize_button_bg: Hsla,
    maximize_button_border: Hsla,

    // Content area
    background: Hsla,

    // Inputs
    input_bg: Hsla,
    input_border: Hsla,
    input_border_focused: Hsla,
    input_text: Hsla,

    // Buttons
    button_primary_bg: Hsla,
    button_primary_bg_hover: Hsla,
    button_primary_text: Hsla,
    button_secondary_bg: Hsla,
    button_secondary_bg_hover: Hsla,
    button_secondary_text: Hsla,
    button_secondary_border: Hsla,

    // Text
    text_primary: Hsla,
    text_secondary: Hsla,
    text_error: Hsla,
}

impl Theme {
    fn new(platform: Platform) -> Self {
        match platform {
            Platform::MacOS => Self::macos_system(),
            Platform::Windows => Self::windows_system(),
            Platform::Linux => Self::linux_system(),
        }
    }

    // macOS system theme detection
    #[cfg(target_os = "macos")]
    fn macos_system() -> Self {
        use objc2::msg_send;
        use objc2::rc::Retained;
        use objc2_app_kit::{NSAppearance, NSApplication};
        use objc2_foundation::{MainThreadMarker, NSString};

        unsafe {
            // Get main thread marker (we assume we're on the main thread during theme initialization)
            let mtm = MainThreadMarker::new_unchecked();

            // Get the current appearance
            let app = NSApplication::sharedApplication(mtm);
            let appearance: Option<Retained<NSAppearance>> =
                msg_send![&app, effectiveAppearance];

            // Check if we're in dark mode by checking the appearance name
            let is_dark = if let Some(appearance) = appearance {
                let name: Retained<NSString> = msg_send![&appearance, name];
                let name_str = name.to_string();
                // Check if the appearance name contains "Dark"
                name_str.contains("Dark") || name_str.contains("dark")
            } else {
                false
            };

            // Try to get system accent color
            let accent_color = Self::get_macos_accent_color();

            Self::macos_with_preferences(is_dark, accent_color)
        }
    }

    #[cfg(target_os = "macos")]
    fn get_macos_accent_color() -> Option<u32> {
        use objc2::rc::Retained;
        use objc2::{ClassType, msg_send,};
        use objc2_app_kit::{NSColor, NSColorSpace};

        unsafe {
            // Get the system accent color (controlAccentColor)
            let color: Option<Retained<NSColor>> =
                msg_send![NSColor::class(), controlAccentColor];
            let color = color?;

            // Convert to RGB color space
            let srgb_space = NSColorSpace::sRGBColorSpace();
            let rgb_color: Option<Retained<NSColor>> =
                msg_send![&color, colorUsingColorSpace: &*srgb_space];
            let rgb_color = rgb_color?;

            // Get RGB components
            let mut r: f64 = 0.0;
            let mut g: f64 = 0.0;
            let mut b: f64 = 0.0;
            let _: () = msg_send![
                &rgb_color,
                getRed: &mut r,
                green: &mut g,
                blue: &mut b,
                alpha: std::ptr::null_mut::<f64>()
            ];

            // Convert to hex
            let r_int = (r * 255.0) as u32;
            let g_int = (g * 255.0) as u32;
            let b_int = (b * 255.0) as u32;
            Some((r_int << 16) | (g_int << 8) | b_int)
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn macos_system() -> Self {
        Self::macos_with_preferences(false, None)
    }

    fn macos_with_preferences(is_dark: bool, accent_color: Option<u32>) -> Self {
        // Use system accent color if available, otherwise default to macOS blue
        let accent = accent_color.unwrap_or(0x007AFF);
        let accent_hover = Self::darken_color(accent, 0.9);

        if is_dark {
            // Dark mode colors
            Self {
                titlebar_bg: rgb(0x2D2D2D).into(),
                titlebar_border: rgb(0x1E1E1E).into(),
                titlebar_height: 22.0,

                close_button_bg: rgb(0xFF5F57).into(),
                close_button_border: rgb(0xE04943).into(),
                minimize_button_bg: rgb(0xFFBD2E).into(),
                minimize_button_border: rgb(0xDEA123).into(),
                maximize_button_bg: rgb(0x28C940).into(),
                maximize_button_border: rgb(0x1AAB29).into(),

                background: rgb(0x1E1E1E).into(),

                input_bg: rgb(0x2D2D2D).into(),
                input_border: rgb(0x404040).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0xFFFFFF).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0x2D2D2D).into(),
                button_secondary_bg_hover: rgb(0x383838).into(),
                button_secondary_text: rgb(0xFFFFFF).into(),
                button_secondary_border: rgb(0x505050).into(),

                text_primary: rgb(0xFFFFFF).into(),
                text_secondary: rgb(0xA0A0A0).into(),
                text_error: rgb(0xFF6B6B).into(),
            }
        } else {
            // Light mode colors
            Self {
                titlebar_bg: rgb(0xE8E8E8).into(),
                titlebar_border: rgb(0xD0D0D0).into(),
                titlebar_height: 22.0,

                close_button_bg: rgb(0xFF5F57).into(),
                close_button_border: rgb(0xE04943).into(),
                minimize_button_bg: rgb(0xFFBD2E).into(),
                minimize_button_border: rgb(0xDEA123).into(),
                maximize_button_bg: rgb(0x28C940).into(),
                maximize_button_border: rgb(0x1AAB29).into(),

                background: rgb(0xEFEFEF).into(),

                input_bg: rgb(0xFFFFFF).into(),
                input_border: rgb(0xCCCCCC).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0x000000).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0xFFFFFF).into(),
                button_secondary_bg_hover: rgb(0xF8F8F8).into(),
                button_secondary_text: rgb(0x000000).into(),
                button_secondary_border: rgb(0xB8B8B8).into(),

                text_primary: rgb(0x000000).into(),
                text_secondary: rgb(0x666666).into(),
                text_error: rgb(0xCC0000).into(),
            }
        }
    }

    // Helper function to darken a color
    fn darken_color(color: u32, factor: f32) -> u32 {
        let r = ((color >> 16) & 0xFF) as f32;
        let g = ((color >> 8) & 0xFF) as f32;
        let b = (color & 0xFF) as f32;

        let r_dark = (r * factor) as u32;
        let g_dark = (g * factor) as u32;
        let b_dark = (b * factor) as u32;

        (r_dark << 16) | (g_dark << 8) | b_dark
    }

    // Windows system theme detection
    #[cfg(target_os = "windows")]
    fn windows_system() -> Self {
        use windows::Win32::Foundation::BOOL;
        use windows::Win32::Graphics::Dwm::DwmGetColorizationColor;

        unsafe {
            // Try to get accent color from DWM
            let mut colorization: u32 = 0;
            let mut opaque_blend: BOOL = BOOL(0);
            let accent_color =
                if DwmGetColorizationColor(&mut colorization, &mut opaque_blend).is_ok() {
                    // Extract RGB from colorization color (format: 0xAARRGGBB)
                    Some(colorization & 0x00FFFFFF)
                } else {
                    None
                };

            // Detect dark mode (simplified - in reality would check registry)
            // For now, defaulting to light mode
            let is_dark = false;

            Self::windows_with_preferences(is_dark, accent_color)
        }
    }

    #[cfg(not(target_os = "windows"))]
    fn windows_system() -> Self {
        Self::windows_with_preferences(false, None)
    }

    fn windows_with_preferences(is_dark: bool, accent_color: Option<u32>) -> Self {
        let accent = accent_color.unwrap_or(0x0078D4);
        let accent_hover = Self::darken_color(accent, 0.9);

        if is_dark {
            // Windows dark mode
            Self {
                titlebar_bg: rgb(0x202020).into(),
                titlebar_border: rgb(0x1A1A1A).into(),
                titlebar_height: 32.0,

                close_button_bg: rgb(0xE81123).into(),
                close_button_border: rgb(0xC50F1F).into(),
                minimize_button_bg: rgb(0x202020).into(),
                minimize_button_border: rgb(0x1A1A1A).into(),
                maximize_button_bg: rgb(0x202020).into(),
                maximize_button_border: rgb(0x1A1A1A).into(),

                background: rgb(0x1E1E1E).into(),

                input_bg: rgb(0x2D2D2D).into(),
                input_border: rgb(0x404040).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0xFFFFFF).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0x2D2D2D).into(),
                button_secondary_bg_hover: rgb(0x383838).into(),
                button_secondary_text: rgb(0xFFFFFF).into(),
                button_secondary_border: rgb(0x505050).into(),

                text_primary: rgb(0xFFFFFF).into(),
                text_secondary: rgb(0xA0A0A0).into(),
                text_error: rgb(0xFF6B6B).into(),
            }
        } else {
            // Windows light mode
            Self {
                titlebar_bg: rgb(0xF0F0F0).into(),
                titlebar_border: rgb(0xDFDFDF).into(),
                titlebar_height: 32.0,

                close_button_bg: rgb(0xE81123).into(),
                close_button_border: rgb(0xC50F1F).into(),
                minimize_button_bg: rgb(0xF0F0F0).into(),
                minimize_button_border: rgb(0xDFDFDF).into(),
                maximize_button_bg: rgb(0xF0F0F0).into(),
                maximize_button_border: rgb(0xDFDFDF).into(),

                background: rgb(0xFFFFFF).into(),

                input_bg: rgb(0xFFFFFF).into(),
                input_border: rgb(0x8A8A8A).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0x000000).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0xFFFFFF).into(),
                button_secondary_bg_hover: rgb(0xF5F5F5).into(),
                button_secondary_text: rgb(0x000000).into(),
                button_secondary_border: rgb(0x8A8A8A).into(),

                text_primary: rgb(0x000000).into(),
                text_secondary: rgb(0x605E5C).into(),
                text_error: rgb(0xA80000).into(),
            }
        }
    }

    // Linux system theme detection
    #[cfg(target_os = "linux")]
    fn linux_system() -> Self {
        use gtk::prelude::*;
        use gtk::{Settings, StyleContext};

        // Try to read GTK theme colors
        let accent_color = Self::get_gtk_accent_color();
        let is_dark = Self::get_gtk_dark_mode();

        Self::linux_with_preferences(is_dark, accent_color)
    }

    #[cfg(target_os = "linux")]
    fn get_gtk_accent_color() -> Option<u32> {
        // Initialize GTK if not already done
        if gtk::init().is_err() {
            return None;
        }

        // Try to get the theme accent color
        // This is a simplified implementation - real GTK theming is more complex
        let settings = Settings::default()?;
        let theme_name = settings.gtk_theme_name()?;

        // Map known themes to their accent colors
        if theme_name.contains("Adwaita") {
            Some(0x3584E4) // GNOME blue
        } else if theme_name.contains("elementary") {
            Some(0x3689E6) // elementary OS blue
        } else {
            None // Use default
        }
    }

    #[cfg(target_os = "linux")]
    fn get_gtk_dark_mode() -> bool {
        if gtk::init().is_err() {
            return false;
        }

        Settings::default()
            .and_then(|s| s.gtk_application_prefer_dark_theme())
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "linux"))]
    fn linux_system() -> Self {
        Self::linux_with_preferences(false, None)
    }

    fn linux_with_preferences(is_dark: bool, accent_color: Option<u32>) -> Self {
        let accent = accent_color.unwrap_or(0x3584E4);
        let accent_hover = Self::darken_color(accent, 0.9);

        if is_dark {
            // GTK/Adwaita dark theme
            Self {
                titlebar_bg: rgb(0x2D2D2D).into(),
                titlebar_border: rgb(0x1E1E1E).into(),
                titlebar_height: 38.0,

                close_button_bg: rgb(0xED333B).into(),
                close_button_border: rgb(0xC01C28).into(),
                minimize_button_bg: rgb(0x2D2D2D).into(),
                minimize_button_border: rgb(0x1E1E1E).into(),
                maximize_button_bg: rgb(0x2D2D2D).into(),
                maximize_button_border: rgb(0x1E1E1E).into(),

                background: rgb(0x242424).into(),

                input_bg: rgb(0x303030).into(),
                input_border: rgb(0x454545).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0xFFFFFF).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0x303030).into(),
                button_secondary_bg_hover: rgb(0x383838).into(),
                button_secondary_text: rgb(0xFFFFFF).into(),
                button_secondary_border: rgb(0x454545).into(),

                text_primary: rgb(0xFFFFFF).into(),
                text_secondary: rgb(0xA0A0A0).into(),
                text_error: rgb(0xFF6B6B).into(),
            }
        } else {
            // GTK/Adwaita light theme
            Self {
                titlebar_bg: rgb(0xEBEBEB).into(),
                titlebar_border: rgb(0xCDCDCD).into(),
                titlebar_height: 38.0,

                close_button_bg: rgb(0xED333B).into(),
                close_button_border: rgb(0xC01C28).into(),
                minimize_button_bg: rgb(0xEBEBEB).into(),
                minimize_button_border: rgb(0xCDCDCD).into(),
                maximize_button_bg: rgb(0xEBEBEB).into(),
                maximize_button_border: rgb(0xCDCDCD).into(),

                background: rgb(0xFAFAFA).into(),

                input_bg: rgb(0xFFFFFF).into(),
                input_border: rgb(0xCDCDCD).into(),
                input_border_focused: rgb(accent).into(),
                input_text: rgb(0x2E3436).into(),

                button_primary_bg: rgb(accent).into(),
                button_primary_bg_hover: rgb(accent_hover).into(),
                button_primary_text: rgb(0xFFFFFF).into(),
                button_secondary_bg: rgb(0xFFFFFF).into(),
                button_secondary_bg_hover: rgb(0xF6F5F4).into(),
                button_secondary_text: rgb(0x2E3436).into(),
                button_secondary_border: rgb(0xCDCDCD).into(),

                text_primary: rgb(0x2E3436).into(),
                text_secondary: rgb(0x5E5C64).into(),
                text_error: rgb(0xC01C28).into(),
            }
        }
    }
}

// =============================================================================
// UI MACROS
// =============================================================================
//
// TUTORIAL: Simplifying UI Code with Macros
// ------------------------------------------
// Repetitive UI patterns are perfect candidates for macros. Instead of writing
// the same builder pattern chains repeatedly, we create macros that generate
// the boilerplate for us.
//
// Benefits:
// - Less code to write and maintain
// - Consistent styling automatically
// - Easy to update globally
// - Type-safe (unlike string templates)

/// Create a styled button with consistent appearance
macro_rules! styled_button {
    ($label:expr, $theme:expr, primary, $handler:expr, $cx:expr) => {
        div()
            .flex()
            .items_center()
            .justify_center()
            .px_6()
            .h(px(32.0))
            .min_w(px(90.0))
            .bg($theme.button_primary_bg)
            .text_color($theme.button_primary_text)
            .text_size(px(13.0))
            .font_weight(FontWeight::NORMAL)
            .rounded(px(6.0))
            .cursor_pointer()
            .shadow_sm()
            .hover(|style| style.bg($theme.button_primary_bg_hover))
            .on_mouse_up(MouseButton::Left, $cx.listener($handler))
            .child($label)
    };
    ($label:expr, $theme:expr, secondary, $handler:expr, $cx:expr) => {
        div()
            .flex()
            .items_center()
            .justify_center()
            .px_6()
            .h(px(32.0))
            .min_w(px(90.0))
            .bg($theme.button_secondary_bg)
            .text_color($theme.button_secondary_text)
            .text_size(px(13.0))
            .font_weight(FontWeight::NORMAL)
            .rounded(px(6.0))
            .border_1()
            .border_color($theme.button_secondary_border)
            .cursor_pointer()
            .shadow_sm()
            .hover(|style| style.bg($theme.button_secondary_bg_hover))
            .on_mouse_up(MouseButton::Left, $cx.listener($handler))
            .child($label)
    };
}

// =============================================================================
// ACTIONS
// =============================================================================
//
// TUTORIAL: Actions in GPUI
// -------------------------
// Actions are user-triggered commands that can be invoked via keyboard shortcuts or menus.
// They provide a type-safe way to handle user input that's decoupled from specific UI elements.
//
// Key concepts:
// - actions!() macro: Defines action types in a namespace
// - Action handlers: Functions that take the action and a context
// - Key bindings: Map keyboard shortcuts to actions
// - on_action(): Attach action handlers to views

// Define actions in the "biorhythm" namespace
// These create zero-sized types that can be used as actions
actions!(biorhythm, [Quit, ShowAbout]);

// Action handler for the Quit action
// Takes a reference to the action (often unused) and mutable app context
fn quit(_: &Quit, cx: &mut App) {
    cx.quit(); // Terminate the application gracefully
}

// Action handler for the ShowAbout action
// Displays a native dialog with application information
fn show_about(_: &ShowAbout, _cx: &mut App) {
    // TUTORIAL: Native Dialogs
    // ------------------------
    // While GPUI excels at custom UI, sometimes you want native OS dialogs for
    // standard interactions like About boxes, file pickers, or simple alerts.
    // The native-dialog crate provides cross-platform access to these native dialogs.
    //
    // Benefits of native dialogs:
    // - Familiar to users (uses OS-standard appearance)
    // - Respects system accessibility settings
    // - No custom UI code needed for simple cases
    // - Handles platform differences automatically
    //
    // Platform implementations:
    // - macOS: Uses NSAlert (Cocoa framework)
    // - Windows: Uses MessageBox (Win32 API)
    // - Linux: Uses GTK MessageDialog or zenity fallback

    use native_dialog::{MessageDialog, MessageType};

    MessageDialog::new()
        .set_type(MessageType::Info)
        .set_title("About GPUI Biorhythm Calculator")
        .set_text(
            "GPUI Biorhythm Calculator v0.1.0\n\n\
             A demonstration of cross-platform UI development with GPUI.\n\n\
             Features:\n\
             • Adaptive theming with OS color detection\n\
             • Native menu integration\n\
             • Platform-specific styling\n\
             • Dark mode support\n\n\
             Built with GPUI - GPU-accelerated UI for Rust\n\
             https://github.com/zed-industries/gpui",
        )
        .show_alert()
        .unwrap_or_else(|e| eprintln!("Failed to show about dialog: {}", e));
}

// =============================================================================
// BIORHYTHM CALCULATIONS
// =============================================================================

fn calculate_biorhythm(days_since_birth: i32, cycle_length: f64) -> f64 {
    let angle = 2.0 * std::f64::consts::PI * (days_since_birth as f64) / cycle_length;
    angle.sin()
}

fn days_between_dates(year: i32, month: u32, day: u32) -> i32 {
    // Simplified calculation
    let birth_days = year * 365 + (month as i32) * 30 + day as i32;
    let current_year = 2025;
    let current_month = 11;
    let current_day = 1;
    let current_days = current_year * 365 + current_month * 30 + current_day;
    current_days - birth_days
}

// =============================================================================
// DATE INPUT DIALOG
// =============================================================================
//
// TUTORIAL: Entity State and View Management
// ------------------------------------------
// This struct represents the state for our date input dialog. In GPUI:
// - Any struct can become a view by implementing the `Render` trait
// - The struct holds all the state needed to render and interact with the view
// - State changes trigger re-renders via `cx.notify()`
// - FocusHandle allows tracking which input field has keyboard focus

struct DateInputDialog {
    // Input field values - stored as Strings for easy editing
    year: String,
    month: String,
    day: String,

    // Application state
    is_initial: bool, // Track if this is the first dialog (affects Cancel button behavior)
    chart_window: Option<WindowHandle<BiorhythmChart>>, // Handle to update the chart window

    // TUTORIAL: Focus Management
    // FocusHandle is GPUI's way of tracking keyboard focus. Each input field gets its own handle.
    // Use track_focus() to associate a handle with an element and focus() to move focus.
    year_focus: FocusHandle,
    month_focus: FocusHandle,
    day_focus: FocusHandle,

    // UI state
    validation_error: Option<String>, // Holds error message if validation fails

    // Cursor positions for each field (character index where caret appears)
    year_cursor: usize,
    month_cursor: usize,
    day_cursor: usize,

    // TUTORIAL: Cursor Blinking Implementation
    // To create a blinking caret, we track when it started and use elapsed time
    // to toggle visibility every 500ms
    caret_visible: bool, // Current visibility state (unused but kept for clarity)
    last_blink: Instant, // Timestamp when blinking started - used to calculate visibility

    // TUTORIAL: Adaptive Theming
    // Store the theme so we can apply platform-specific styling throughout the component
    theme: Theme,
}

impl DateInputDialog {
    // TUTORIAL: Entity Constructor Pattern
    // ------------------------------------
    // The constructor takes a Context<Self> which provides access to GPUI functionality.
    // Common pattern: Create FocusHandles via cx.focus_handle() during initialization.
    fn new(
        is_initial: bool,
        chart_window: Option<WindowHandle<BiorhythmChart>>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            // Initialize with default values
            year: String::from("1990"),
            month: String::from("1"),
            day: String::from("1"),
            is_initial,
            chart_window,

            // Create focus handles - each input field needs its own handle
            // to track and manage keyboard focus independently
            year_focus: cx.focus_handle(),
            month_focus: cx.focus_handle(),
            day_focus: cx.focus_handle(),

            validation_error: None,

            // Position cursors at the end of each default value
            year_cursor: 4,  // Position at end of "1990"
            month_cursor: 1, // Position at end of "1"
            day_cursor: 1,   // Position at end of "1"

            caret_visible: true,
            last_blink: Instant::now(), // Start the blink timer

            // TUTORIAL: Platform Detection in Action
            // Detect the platform and load the appropriate theme automatically
            theme: Theme::new(Platform::detect()),
        }
    }

    fn validate_date(&mut self) -> bool {
        // Parse the date values
        let year = match self.year.parse::<i32>() {
            Ok(y) => y,
            Err(_) => {
                self.validation_error = Some("Year must be a valid number".to_string());
                return false;
            }
        };

        let month = match self.month.parse::<u32>() {
            Ok(m) => m,
            Err(_) => {
                self.validation_error = Some("Month must be a valid number".to_string());
                return false;
            }
        };

        let day = match self.day.parse::<u32>() {
            Ok(d) => d,
            Err(_) => {
                self.validation_error = Some("Day must be a valid number".to_string());
                return false;
            }
        };

        // Validate ranges
        if year < 1900 || year > 2100 {
            self.validation_error = Some("Year must be between 1900 and 2100".to_string());
            return false;
        }

        if month < 1 || month > 12 {
            self.validation_error = Some("Month must be between 1 and 12".to_string());
            return false;
        }

        if day < 1 || day > 31 {
            self.validation_error = Some("Day must be between 1 and 31".to_string());
            return false;
        }

        // Additional validation for days in month
        let max_days = match month {
            2 => {
                // Leap year check
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    29
                } else {
                    28
                }
            }
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        };

        if day > max_days {
            self.validation_error = Some(format!(
                "Invalid day for month {}. Maximum is {}",
                month, max_days
            ));
            return false;
        }

        self.validation_error = None;
        true
    }

    fn submit_date(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        // Validate the date
        if self.validate_date() {
            // Parse the validated date
            let year = self.year.parse::<i32>().unwrap();
            let month = self.month.parse::<u32>().unwrap();
            let day = self.day.parse::<u32>().unwrap();

            // Update the chart window with the new birthdate
            if let Some(chart_window) = &self.chart_window {
                chart_window
                    .update(cx, |chart, _window, cx| {
                        chart.update_birthdate(year, month, day, cx);
                    })
                    .ok();
            }

            // Close the dialog window - the chart window will automatically gain focus
            window.remove_window();
        } else {
            // Re-render to show validation error
            cx.notify();
        }
    }

    fn on_ok_clicked(&mut self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        self.submit_date(window, cx);
    }

    fn on_cancel_clicked(&mut self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_initial {
            // If this is the initial dialog and user cancels, quit the app
            cx.quit();
        } else {
            // Close the dialog window - the chart window will automatically gain focus
            window.remove_window();
        }
    }
}

// TUTORIAL: The Render Trait
// ---------------------------
// Implementing Render makes your struct a view that GPUI can display.
// The render() method is called every time the view needs to be redrawn:
// - When cx.notify() is called
// - When window events occur (resize, focus, etc.)
// - When parent views re-render
//
// Key principles:
// - Describe what the UI should look like RIGHT NOW based on current state
// - Don't mutate state in render() (except for derived calculations)
// - Return elements that implement IntoElement
// - GPUI compares old and new descriptions and updates only what changed
impl Render for DateInputDialog {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // TUTORIAL: Implementing Cursor Blink Animation
        // ---------------------------------------------
        // To create a blinking cursor that toggles every 500ms:
        //
        // 1. Calculate elapsed time since last_blink started
        // 2. Divide by blink interval (500ms) and check if result is even/odd
        //    - elapsed_ms / 500 gives number of completed 500ms periods
        //    - % 2 gives 0 (even) or 1 (odd)
        //    - When even (0), caret is visible; when odd (1), caret is hidden
        //
        // Example timeline:
        //   0-499ms:   elapsed/500 = 0, 0%2 = 0 → visible
        //   500-999ms: elapsed/500 = 1, 1%2 = 1 → hidden
        //   1000-1499ms: elapsed/500 = 2, 2%2 = 0 → visible
        let elapsed_ms = self.last_blink.elapsed().as_millis();
        let caret_visible = (elapsed_ms / 500) % 2 == 0;

        // TUTORIAL: Continuous Animation with on_next_frame()
        // ---------------------------------------------------
        // Problem: The render method only runs when something triggers it (events, notify, etc.)
        // Solution: Schedule a notification for the next frame to create a continuous render loop
        //
        // on_next_frame() schedules a callback to run on the next animation frame (similar to
        // requestAnimationFrame in web browsers). This ensures our caret continues blinking even
        // when the user isn't interacting with the dialog.
        //
        // Without this, the caret would only blink when the user types or moves focus!
        cx.on_next_frame(
            window,
            |_this: &mut DateInputDialog,
             _window: &mut Window,
             cx: &mut Context<DateInputDialog>| {
                cx.notify(); // Trigger a re-render on the next frame
            },
        );

        let year_value = self.year.clone();
        let month_value = self.month.clone();
        let day_value = self.day.clone();
        let year_focus = self.year_focus.clone();
        let month_focus = self.month_focus.clone();
        let day_focus = self.day_focus.clone();
        let year_cursor = self.year_cursor;
        let month_cursor = self.month_cursor;
        let day_cursor = self.day_cursor;

        // Create the input fields using our helper method
        let year_field = self.editable_input_field(
            "Year",
            &year_focus,
            &year_value,
            year_cursor,
            caret_visible,
            window,
            cx,
        );
        let month_field = self.editable_input_field(
            "Month",
            &month_focus,
            &month_value,
            month_cursor,
            caret_visible,
            window,
            cx,
        );
        let day_field = self.editable_input_field(
            "Day",
            &day_focus,
            &day_value,
            day_cursor,
            caret_visible,
            window,
            cx,
        );

        // TUTORIAL: Building UI with Elements
        // -----------------------------------
        // GPUI uses a builder pattern for constructing UI elements. Key concepts:
        //
        // 1. div() creates a basic container element (like HTML <div>)
        // 2. Methods can be chained to configure the element (flex, bg, p_6, etc.)
        // 3. .child() adds child elements (can be called multiple times)
        // 4. Style methods follow CSS-like naming (flex, flex_col, gap, padding, etc.)
        // 5. Event handlers are attached with .on_* methods
        //
        // Flexbox layout:
        // - .flex() enables flexbox layout
        // - .flex_col() sets direction to column (vertical stacking)
        // - .gap_4() adds spacing between children
        // - .size_full() makes element fill available space
        // TUTORIAL: Using Theme Colors
        // The theme is automatically loaded based on platform detection.
        // All colors now come from self.theme, making the UI adapt to the platform.
        let theme = &self.theme;

        div()
            .flex() // Enable flexbox layout
            .flex_col() // Stack children vertically
            .size_full() // Fill the window
            .bg(theme.background) // Platform-specific background color
            .p_6() // Padding on all sides (6 units)
            .gap_4() // Space between children (4 units)
            // TUTORIAL: Key Contexts and Actions
            // ----------------------------------
            // key_context() defines a scope for keyboard shortcuts. Actions bound within
            // this context will only work when this view or its children have focus.
            .key_context("DateInputDialog")
            // TUTORIAL: Action Handlers with cx.listener()
            // --------------------------------------------
            // cx.listener() creates a handler that has access to the view's state (this).
            // The handler signature is: |this: &mut Self, action: &Action, window, cx|
            // This is the primary way to respond to actions in GPUI.
            .on_action(cx.listener(|_this, _action: &Quit, _window, cx| {
                cx.quit(); // Handle CMD+Q to quit the application
            }))
            .child(
                div()
                    .text_size(px(16.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary) // Platform-specific text color
                    .child("Enter Your Birthdate"),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(year_field)
                    .child(month_field)
                    .child(day_field),
            )
            .when_some(self.validation_error.clone(), |el, error| {
                el.child(
                    div()
                        .text_size(px(12.0))
                        .text_color(theme.text_error) // Platform-specific error color
                        .child(error),
                )
            })
            .child(
                // TUTORIAL: Using Macros for Buttons
                // The styled_button! macro dramatically reduces boilerplate.
                // It automatically applies platform-specific colors from the theme.
                div()
                    .flex()
                    .gap_3()
                    .justify_end()
                    .w_full()
                    .child(styled_button!(
                        "Cancel",
                        theme,
                        secondary,
                        Self::on_cancel_clicked,
                        cx
                    ))
                    .child(styled_button!(
                        "OK",
                        theme,
                        primary,
                        Self::on_ok_clicked,
                        cx
                    )),
            )
    }
}

impl DateInputDialog {
    // TUTORIAL: Creating Reusable Input Components
    // --------------------------------------------
    // This method demonstrates building a custom input field with:
    // - Focus tracking and visual feedback
    // - Cursor positioning and blinking
    // - Keyboard event handling
    // - Mouse interaction
    //
    // Pattern: Helper methods like this keep render() clean and promote reusability
    fn editable_input_field(
        &mut self,
        label: &'static str,
        focus_handle: &FocusHandle,
        value: &str,
        cursor_pos: usize,
        caret_visible: bool,
        window: &Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + use<> {
        // Check if this field currently has keyboard focus
        let is_focused = focus_handle.is_focused(window);

        // TUTORIAL: Implementing Cursor Display
        // -------------------------------------
        // To show a blinking cursor in the text:
        // 1. Convert string to Vec<char> for easy manipulation
        // 2. Insert a '|' character at the cursor position
        // 3. Only show cursor when field is focused AND caret should be visible
        //
        // This is a simple text-based cursor. More sophisticated implementations
        // might use overlay elements or custom rendering.
        let display_text = if is_focused && caret_visible {
            let mut chars: Vec<char> = value.chars().collect();
            let safe_cursor = cursor_pos.min(chars.len()); // Prevent out-of-bounds
            chars.insert(safe_cursor, '|'); // Insert cursor character
            chars.into_iter().collect::<String>()
        } else {
            value.to_string() // No cursor when not focused or not visible
        };

        // Use theme colors for all input styling
        let theme = &self.theme;

        div()
            .flex()
            .flex_col()
            .gap_1()
            .flex_1()
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(theme.text_secondary) // Platform-specific secondary text
                    .child(label),
            )
            .child(
                div()
                    .id(label)
                    .px_2()
                    .py_1()
                    .bg(theme.input_bg) // Platform-specific input background
                    .border_1()
                    .border_color(if is_focused {
                        theme.input_border_focused // Platform-specific focus color
                    } else {
                        theme.input_border // Platform-specific border color
                    })
                    .rounded(px(4.0))
                    .cursor_text() // Show text cursor when hovering
                    // TUTORIAL: Focus Tracking
                    // ------------------------
                    // track_focus() associates this element with a FocusHandle.
                    // This allows the element to receive keyboard events and enables
                    // is_focused() checks for visual feedback.
                    .track_focus(focus_handle)
                    // TUTORIAL: Mouse Event Handling
                    // ------------------------------
                    // on_mouse_down() attaches a handler for mouse click events.
                    // Pattern: Clone handles into the closure to avoid lifetime issues.
                    .on_mouse_down(MouseButton::Left, {
                        let focus_handle = focus_handle.clone(); // Clone for move into closure
                        cx.listener(move |this, _event: &MouseDownEvent, window, cx| {
                            // Give this field keyboard focus
                            focus_handle.focus(window);

                            // Set cursor to end of field when clicked
                            // In a more advanced implementation, you'd calculate the click
                            // position to place the cursor at the clicked character
                            match label {
                                "Year" => this.year_cursor = this.year.len(),
                                "Month" => this.month_cursor = this.month.len(),
                                "Day" => this.day_cursor = this.day.len(),
                                _ => {}
                            }
                            this.caret_visible = true; // Show caret immediately on click
                            cx.notify(); // Trigger re-render to show focus change
                        })
                    })
                    // TUTORIAL: Keyboard Event Handling
                    // ---------------------------------
                    // on_key_down() receives keyboard events when this element has focus.
                    // event.keystroke.key contains the key name as a string.
                    //
                    // Supported keyboard shortcuts in this input field:
                    // - 0-9: Type digits (max length enforced per field)
                    // - BACKSPACE: Delete last character
                    // - TAB: Move to next field (Year→Month→Day→Year)
                    // - SHIFT+TAB: Move to previous field (Day→Month→Year→Day)
                    // - ENTER: Submit the form (validate and update chart)
                    //
                    // Pattern: Use match on keystroke.key for readable key handling
                    .on_key_down(cx.listener(move |this, event: &KeyDownEvent, window, cx| {
                        // Get mutable references to the current field's data
                        let (field_value, cursor) = match label {
                            "Year" => (&mut this.year, &mut this.year_cursor),
                            "Month" => (&mut this.month, &mut this.month_cursor),
                            "Day" => (&mut this.day, &mut this.day_cursor),
                            _ => return, // Unknown field, ignore
                        };

                        // Handle different key presses
                        match event.keystroke.key.as_str() {
                            "backspace" => {
                                if *cursor > 0 && !field_value.is_empty() {
                                    field_value.pop();
                                    *cursor = field_value.len();
                                    this.caret_visible = true; // Show caret immediately on input
                                    cx.notify();
                                }
                            }
                            key if key.len() == 1 && key.chars().all(|c| c.is_ascii_digit()) => {
                                // Limit length based on field
                                let max_len = match label {
                                    "Year" => 4,
                                    "Month" | "Day" => 2,
                                    _ => 4,
                                };
                                if field_value.len() < max_len {
                                    field_value.push_str(key);
                                    *cursor = field_value.len();
                                    this.caret_visible = true; // Show caret immediately on input
                                    cx.notify();
                                }
                            }
                            // TUTORIAL: Tab Navigation Between Fields
                            // ---------------------------------------
                            // TAB moves forward through fields: Year → Month → Day → Year (wraps)
                            // SHIFT+TAB moves backward: Day → Month → Year → Day (wraps)
                            // This creates intuitive keyboard navigation without reaching for the mouse
                            "tab" => {
                                // Move to next field (forward), wrapping from last to first
                                match label {
                                    "Year" => this.month_focus.focus(window),
                                    "Month" => this.day_focus.focus(window),
                                    "Day" => this.year_focus.focus(window), // Wrap to beginning
                                    _ => {}
                                }
                            }
                            "shift-tab" => {
                                // Move to previous field (backward), wrapping from first to last
                                match label {
                                    "Year" => this.day_focus.focus(window), // Wrap to end
                                    "Month" => this.year_focus.focus(window),
                                    "Day" => this.month_focus.focus(window),
                                    _ => {}
                                }
                            }
                            "enter" => {
                                // Submit the date (validate, update chart, and close)
                                this.submit_date(window, cx);
                            }
                            _ => {}
                        }
                    }))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(theme.input_text) // Platform-specific input text color
                            .child(display_text),
                    ),
            )
    }
}

// =============================================================================
// BIORHYTHM CHART WINDOW
// =============================================================================

struct BiorhythmChart {
    birthdate: Option<(i32, u32, u32)>,
    self_handle: Option<WindowHandle<BiorhythmChart>>,
}

impl BiorhythmChart {
    fn new() -> Self {
        Self {
            birthdate: Some((1990, 1, 1)), // Default birthdate
            self_handle: None,
        }
    }

    fn set_handle(&mut self, handle: WindowHandle<BiorhythmChart>) {
        self.self_handle = Some(handle);
    }

    fn update_birthdate(&mut self, year: i32, month: u32, day: u32, cx: &mut Context<Self>) {
        self.birthdate = Some((year, month, day));
        cx.notify(); // Trigger a re-render
    }

    fn on_double_click(
        &mut self,
        _event: &MouseDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Open the date input dialog and pass this chart window's handle
        let bounds = Bounds::centered(None, size(px(320.0), px(240.0)), cx);
        let chart_handle = self.self_handle.clone();
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Enter Birthdate")),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                focus: true,
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                ..Default::default()
            },
            move |_, cx| cx.new(|cx| DateInputDialog::new(false, chart_handle, cx)),
        )
        .ok();
    }
}

impl Render for BiorhythmChart {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (days_since_birth, birthdate_str) = if let Some((year, month, day)) = self.birthdate {
            (
                days_between_dates(year, month, day),
                format!("Birth: {}/{}/{}", month, day, year),
            )
        } else {
            (0, "No birthdate set".to_string())
        };

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0xFFFFFF))
            .key_context("BiorhythmChart")
            .on_action(cx.listener(|_this, _action: &Quit, _window, cx| {
                cx.quit();
            }))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, window, cx| {
                    if event.click_count == 2 {
                        this.on_double_click(event, window, cx);
                    }
                }),
            )
            .child(
                // Title bar
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .h(px(40.0))
                    .px_4()
                    .border_b_1()
                    .border_color(rgb(0xE0E0E0))
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x000000))
                            .child("Biorhythm Chart"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x666666))
                            .child(birthdate_str),
                    ),
            )
            .child(
                // Chart area
                div()
                    .flex()
                    .flex_col()
                    .p_3()
                    .child(self.render_chart(days_since_birth)),
            )
            .child(
                // Legend
                div()
                    .flex()
                    .gap_6()
                    .h(px(40.0))
                    .px_4()
                    .items_center()
                    .border_t_1()
                    .border_color(rgb(0xE0E0E0))
                    .child(self.legend_item("Physical (23 days)".to_string(), rgb(0xFF0000).into()))
                    .child(
                        self.legend_item("Emotional (28 days)".to_string(), rgb(0x00AA00).into()),
                    )
                    .child(
                        self.legend_item(
                            "Intellectual (33 days)".to_string(),
                            rgb(0x0000FF).into(),
                        ),
                    ),
            )
    }
}

impl BiorhythmChart {
    fn legend_item(&self, label: String, color: Hsla) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .items_center()
            .child(div().w(px(20.0)).h(px(3.0)).bg(color))
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(label),
            )
    }

    fn render_chart(&self, days_since_birth: i32) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x666666))
                    .child(format!(
                        "Days since birth: {} (showing next 33 days)",
                        days_since_birth
                    )),
            )
            .child(self.render_chart_lines(days_since_birth))
    }

    fn render_chart_lines(&self, days_since_birth: i32) -> impl IntoElement {
        let chart_width = 700.0;
        let chart_height = 300.0;
        let days_to_show = 33; // Match the longest biorhythm cycle (intellectual)

        div()
            .flex()
            .relative()
            .w(px(chart_width))
            .h(px(chart_height))
            .bg(rgb(0xF8F8F8))
            .border_1()
            .border_color(rgb(0xE0E0E0))
            .rounded(px(4.0))
            // Draw center line
            .child(
                div()
                    .absolute()
                    .left(px(0.0))
                    .top(px(chart_height / 2.0))
                    .w(px(chart_width))
                    .h(px(1.0))
                    .bg(rgb(0xCCCCCC)),
            )
            // Draw physical cycle (red) - lines and dots
            .children(self.create_cycle_lines(
                days_since_birth,
                23.0,
                rgb(0xFF0000).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
            .children(self.create_cycle_points(
                days_since_birth,
                23.0,
                rgb(0xFF0000).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
            // Draw emotional cycle (green) - lines and dots
            .children(self.create_cycle_lines(
                days_since_birth,
                28.0,
                rgb(0x00AA00).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
            .children(self.create_cycle_points(
                days_since_birth,
                28.0,
                rgb(0x00AA00).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
            // Draw intellectual cycle (blue) - lines and dots
            .children(self.create_cycle_lines(
                days_since_birth,
                33.0,
                rgb(0x0000FF).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
            .children(self.create_cycle_points(
                days_since_birth,
                33.0,
                rgb(0x0000FF).into(),
                chart_width,
                chart_height,
                days_to_show,
            ))
    }

    fn create_cycle_lines(
        &self,
        days_since_birth: i32,
        cycle_length: f64,
        color: Hsla,
        width: f32,
        height: f32,
        days: i32,
    ) -> Vec<impl IntoElement> {
        let mut lines = Vec::new();
        let x_step = width / (days as f32);

        for i in 0..(days - 1) {
            let day1 = days_since_birth + i;
            let day2 = days_since_birth + i + 1;

            let value1 = calculate_biorhythm(day1, cycle_length);
            let value2 = calculate_biorhythm(day2, cycle_length);

            let y1 = (height / 2.0) - (value1 as f32 * height / 2.5);
            let y2 = (height / 2.0) - (value2 as f32 * height / 2.5);

            let x1 = i as f32 * x_step;
            let x2 = (i + 1) as f32 * x_step;

            // Draw simple line approximation using small rectangles
            // Calculate the steps for the line
            let steps = 10;
            for step in 0..steps {
                let t = step as f32 / steps as f32;
                let x = x1 + t * (x2 - x1);
                let y = y1 + t * (y2 - y1);

                lines.push(
                    div()
                        .absolute()
                        .left(px(x))
                        .top(px(y))
                        .w(px(2.0))
                        .h(px(2.0))
                        .bg(color),
                );
            }
        }

        lines
    }

    fn create_cycle_points(
        &self,
        days_since_birth: i32,
        cycle_length: f64,
        color: Hsla,
        width: f32,
        height: f32,
        days: i32,
    ) -> Vec<impl IntoElement> {
        let mut points = Vec::new();
        let x_step = width / (days as f32);

        for i in 0..days {
            let day = days_since_birth + i;
            let value = calculate_biorhythm(day, cycle_length);

            // Convert value (-1 to 1) to y position (height to 0)
            let y = (height / 2.0) - (value as f32 * height / 2.5);
            let x = i as f32 * x_step;

            points.push(
                div()
                    .absolute()
                    .left(px(x - 2.0)) // Center the dot
                    .top(px(y - 2.0)) // Center the dot
                    .w(px(4.0))
                    .h(px(4.0))
                    .bg(color)
                    .rounded_full(),
            );
        }

        points
    }
}

// =============================================================================
// MAIN FUNCTION
// =============================================================================
//
// TUTORIAL: GPUI Application Setup
// --------------------------------
// The main function initializes the GPUI application and sets up the initial state.
// Key concepts:
// 1. Application::new().run() - Starts the GPUI event loop
// 2. cx: &mut App - The root application context for global operations
// 3. Window creation with cx.open_window()
// 4. WindowHandle for cross-window communication
// 5. Lifecycle management with observe_release()

fn main() {
    // TUTORIAL: Starting a GPUI Application
    // -------------------------------------
    // Application::new() creates the application instance.
    // .run() starts the event loop and provides the App context.
    // The closure passed to run() is called once at startup.
    Application::new().run(|cx: &mut App| {
        // Bring application to foreground (macOS-specific)
        cx.activate(true);

        // TUTORIAL: Setting Up Actions and Keybindings
        // --------------------------------------------
        // 1. Register action handler - connects action type to function
        // 2. Bind keyboard shortcut - maps key combo to action
        // 3. Add to menu - provides menu access to action
        cx.on_action(quit); // Register quit handler
        cx.on_action(show_about); // Register about dialog handler

        // Bind CMD+Q to trigger Quit action (None = no specific context required)
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

        // =============================================================================
        // TUTORIAL: Platform-Specific Menu Systems
        // =============================================================================
        //
        // Menus are one of the most platform-specific UI elements. Each OS has its
        // own conventions, APIs, and visual design for application menus.
        //
        // MACOS MENU SYSTEM
        // -----------------
        // • Uses NSMenu (part of Cocoa/AppKit framework)
        // • Menu bar is at the TOP OF THE SCREEN (not in window)
        // • Always visible, shared across all app windows
        // • Standard menus: Application, File, Edit, View, Window, Help
        // • Application menu (first menu) contains app-level items:
        //   - About [App Name]
        //   - Preferences (Cmd+,)
        //   - Services (system-provided submenu)
        //   - Hide/Show [App Name]
        //   - Quit [App Name] (Cmd+Q)
        // • System handles some items automatically (like Services)
        // • Menu shortcuts use Command (⌘) key primarily
        //
        // WINDOWS MENU SYSTEM
        // -------------------
        // • Uses Win32 Menu API (CreateMenu, AppendMenu, etc.)
        // • Menu bar is IN THE WINDOW (below title bar)
        // • Each window has its own menu bar
        // • Standard menus: File, Edit, View, Tools, Help
        // • No "Application" menu - app-level items go in File menu:
        //   - Exit (Alt+F4) in File menu
        //   - Options/Settings in Tools or Edit menu
        //   - About [App Name] in Help menu (rightmost)
        // • Menu shortcuts use Alt key to activate menu bar
        // • Keyboard accelerators shown with underlined letters
        // • Separator lines group related items
        //
        // LINUX/GTK MENU SYSTEM
        // ---------------------
        // • Uses GTK MenuBar/GMenu (GNOME) or Qt menus (KDE)
        // • Menu bar location depends on desktop environment:
        //   - GNOME: Menu bar at top of screen (like macOS) or hidden in overview
        //   - KDE/Xfce: Menu bar in window (like Windows)
        //   - Unity: Global menu bar (like macOS)
        // • Standard menus similar to Windows: File, Edit, View, Help
        // • Application menu items:
        //   - About in Help menu
        //   - Preferences in Edit menu
        //   - Quit in File menu (Ctrl+Q)
        // • Menu shortcuts use Ctrl key primarily
        // • Modern GNOME apps often use hamburger menu instead of menu bar
        //
        // GPUI's APPROACH
        // ---------------
        // GPUI provides cx.set_menus() which abstracts platform differences:
        // • On macOS: Creates NSMenu and adds to menu bar
        // • On Windows: Creates Win32 menu and attaches to window
        // • On Linux: Creates GTK menu bar
        //
        // The Menu struct defines menu structure in a platform-neutral way:
        // • Menu { name, items } - Top-level menu (File, Edit, etc.)
        // • MenuItem::action() - Triggers an action when clicked
        // • MenuItem::separator() - Visual divider between menu items
        // • MenuItem::os_submenu() - Platform-specific submenus (like Services on macOS)
        //
        // Best practices:
        // 1. Follow platform conventions for menu organization
        // 2. Use standard keyboard shortcuts (Cmd on Mac, Ctrl on others)
        // 3. Include separators to group related items
        // 4. Provide both menu and keyboard access to all actions
        // 5. Use MenuItem::os_submenu() for platform-specific menus

        // Create application menu
        // On macOS: This becomes the "Biorhythm Calculator" menu in the menu bar
        // On Windows/Linux: This could be organized differently (e.g., File, Help menus)
        cx.set_menus(vec![Menu {
            name: "Biorhythm Calculator".into(),
            items: vec![
                // About menu item - triggers ShowAbout action
                // macOS: Standard first item in application menu
                // Windows/Linux: Would typically go in Help menu
                MenuItem::action("About Biorhythm Calculator", ShowAbout),
                MenuItem::separator(),
                // Services submenu - macOS system feature
                // On macOS: System automatically populates this with available services
                // On Windows/Linux: This is ignored (platform-specific)
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::separator(),
                // Quit menu item - triggers Quit action
                // Also bound to Cmd+Q (macOS) / Ctrl+Q (Linux) / Alt+F4 (Windows)
                MenuItem::action("Quit", Quit),
            ],
        }]);

        // TUTORIAL: Creating Windows
        // -------------------------
        // cx.open_window() creates a new window and returns a WindowHandle.
        // WindowHandle allows you to:
        // - Update the window's view from other parts of the app
        // - Check if the window still exists
        // - Pass to other windows for communication
        let chart_window = cx
            .open_window(
                // Configure window appearance and behavior
                WindowOptions {
                    // Center the window on screen with specific size
                    window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                        None,                       // None = center on primary display
                        size(px(750.0), px(450.0)), // Width x Height
                        cx,
                    ))),
                    // Configure the title bar
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::from("Biorhythm Calculator")),
                        appears_transparent: false,   // Solid title bar
                        traffic_light_position: None, // Default position for close/minimize/maximize
                    }),
                    focus: false, // Don't focus initially (we'll focus the dialog instead)
                    show: true,   // Make window visible immediately
                    kind: WindowKind::Normal, // Standard window (vs popup, panel, etc.)
                    is_movable: true, // User can drag to reposition
                    ..Default::default()
                },
                // This closure creates the view for the window
                // cx.new() creates an entity instance that will be owned by the window
                |_, cx| cx.new(|_cx| BiorhythmChart::new()),
            )
            .unwrap();

        // TUTORIAL: Cross-Window Communication
        // ------------------------------------
        // Pattern: Give the chart window a reference to itself so it can pass
        // the handle to dialog windows it creates. This allows dialogs to
        // update the chart when the user submits a new birthdate.
        let chart_window_clone = chart_window.clone(); // WindowHandle is cheap to clone
        chart_window
            .update(cx, |chart, _window, _cx| {
                chart.set_handle(chart_window_clone);
            })
            .ok(); // .ok() ignores errors if window was already closed

        // TUTORIAL: Lifecycle Management with observe_release()
        // -----------------------------------------------------
        // observe_release() registers a callback that runs when an entity is dropped.
        // Use case: Quit the app when the main window closes.
        //
        // Steps:
        // 1. Get the entity reference from the window handle
        // 2. Register observer callback
        // 3. .detach() means we don't need to keep the subscription handle
        let chart_entity = chart_window.entity(cx).unwrap();
        cx.observe_release(&chart_entity, |_, _app_cx| {
            _app_cx.quit(); // When chart window closes, quit the entire app
        })
        .detach();

        // TUTORIAL: Dialog Pattern with Window Handle Passing
        // ---------------------------------------------------
        // This demonstrates how to create a dialog that can communicate back to
        // another window. Key points:
        //
        // 1. Pass the chart_window handle to the dialog's constructor
        // 2. Dialog stores the handle and uses it to update the chart
        // 3. is_initial=true tells dialog this is the first launch (affects Cancel behavior)
        // 4. focus=true ensures dialog appears on top and receives keyboard input
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                    None,
                    size(px(320.0), px(240.0)), // Smaller size for dialog
                    cx,
                ))),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from("Enter Birthdate")),
                    appears_transparent: false,
                    traffic_light_position: None,
                }),
                focus: true, // Give focus to dialog (not the chart window)
                show: true,
                kind: WindowKind::Normal,
                is_movable: true,
                ..Default::default()
            },
            // Create dialog, passing:
            // - is_initial=true (first launch, Cancel should quit)
            // - Some(chart_window) (handle to update chart)
            |_, cx| cx.new(|cx| DateInputDialog::new(true, Some(chart_window), cx)),
        )
        .unwrap();
    });
}

// TUTORIAL SUMMARY: Key GPUI Patterns Demonstrated
// ================================================
//
// 1. VIEW PATTERN: Implement Render trait to create views
//    - render() describes UI declaratively
//    - State changes trigger re-renders via cx.notify()
//
// 2. ELEMENT BUILDER PATTERN: Fluent API for UI construction
//    - div() creates containers
//    - Chain methods to configure styling and behavior
//    - .child() adds child elements
//
// 3. EVENT HANDLING: Attach handlers with cx.listener()
//    - Mouse events: on_mouse_down(), on_mouse_up()
//    - Keyboard events: on_key_down()
//    - Actions: on_action() for higher-level commands
//
// 4. FOCUS MANAGEMENT: FocusHandle tracks keyboard focus
//    - Create handles with cx.focus_handle()
//    - Associate with elements via .track_focus()
//    - Move focus with handle.focus(window)
//
// 5. ANIMATION: Continuous rendering with on_next_frame()
//    - Schedule work for next frame
//    - Call cx.notify() to trigger re-render
//    - Creates smooth animations without complex timers
//
// 6. WINDOW MANAGEMENT: Multiple windows with WindowHandle
//    - cx.open_window() creates windows
//    - WindowHandle enables cross-window communication
//    - observe_release() handles lifecycle events
//
// 7. CONTEXTS: Different contexts for different scopes
//    - App: Global application state
//    - Context<T>: Entity-specific operations
//    - Window: Window-specific operations
//
// For more GPUI documentation, visit: https://www.gpui.rs
