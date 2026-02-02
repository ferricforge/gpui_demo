use gpui::*;
use gpui_component::{
    Root,
    StyledExt,
    button::*,
    input::{Input, InputState},
};
use gpui_demo::{Quit, preferences::WindowPreferences, quit, setup_app};

/// An example demonstrating how to combine a text input with a button in gpui-component.
///
/// This example showcases several key concepts:
/// - Creating and managing input state with [`InputState`]
/// - Using buttons with click handlers via [`Button::on_click`]
/// - Setting initial focus programmatically
/// - Tab navigation between focusable elements
/// - Horizontal layout using flexbox-style containers
struct ButtonExample {
    /// The state entity for the text input field.
    /// Wrapped in an [`Entity`] so it can be shared and updated across the view.
    text_input: Entity<InputState>,

    /// Subscription to handle window close events.
    /// Stored to keep the subscription alive for the lifetime of the view.
    /// The underscore prefix indicates we don't access it directly after creation.
    _window_close_subscription: Option<Subscription>,
}

impl ButtonExample {
    /// Creates a new [`ButtonExample`] view.
    ///
    /// This sets up:
    /// 1. A window close handler that quits the application
    /// 2. A text input with placeholder text
    /// 3. Initial focus on the text input
    fn new(
        window: &mut Window,
        view_cx: &mut Context<Self>,
    ) -> Self {
        // Subscribe to window close events so we can quit the app gracefully.
        // Without this, closing the window would leave the app running.
        let subscription = view_cx.on_window_closed(|app_cx: &mut App| {
            println!("Window closed callback!");
            quit(&Quit, app_cx);
        });

        // Create the input state entity. InputState manages the text content,
        // cursor position, selection, and focus state internally.
        let text_input = view_cx
            .new(|input_cx| InputState::new(window, input_cx).placeholder("Enter text here..."));

        // Focus the text input when the window opens.
        // We need to call update() to get mutable access to the InputState,
        // then call its focus() method with both the window and context.
        text_input.update(view_cx, |input, input_cx| {
            input.focus(window, input_cx);
        });

        Self {
            text_input,
            _window_close_subscription: Some(subscription),
        }
    }

    /// Clears the text input field.
    ///
    /// This is the click handler for the "Clear" button. It receives:
    /// - `_`: The [`ClickEvent`] (unused here, but contains click position, modifiers, etc.)
    /// - `window`: Mutable reference to the window for updating UI state
    /// - `view_cx`: The view context for accessing and updating entities
    fn clear_input(
        &mut self,
        _: &ClickEvent,
        window: &mut gpui::Window,
        view_cx: &mut Context<Self>,
    ) {
        println!("Clearing input!");
        self.text_input.update(view_cx, |input, input_cx| {
            input.set_value("", window, input_cx);
        });
    }
}

impl Render for ButtonExample {
    /// Renders the view's UI.
    ///
    /// The layout uses flexbox-style containers:
    /// - Outer `div().v_flex()`: Vertical flex container, centers content in the window
    /// - Inner `div().h_flex()`: Horizontal flex container, places label, input, and button in a row
    ///
    /// The `gap_*` methods add spacing between flex children.
    /// `items_center` vertically centers children within each flex container.
    /// `justify_center` horizontally centers the inner container within the outer one.
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        view_cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            // Vertical flex container - children stack top to bottom
            .v_flex()
            // Spacing between children (using spacing scale, 4 = 1rem = 16px by default)
            .gap_4()
            // Fill the entire window
            .size_full()
            // Center children horizontally within this container
            .items_center()
            // Center children vertically within this container
            .justify_center()
            .child(
                div()
                    // Horizontal flex container - children flow left to right
                    .h_flex()
                    // Smaller gap between the label, input, and button
                    .gap_2()
                    // Vertically align children to center (important when they have different heights)
                    .items_center()
                    .child("Enter text:")
                    // Input component wraps the InputState entity
                    // w_64 sets a fixed width (64 units = 16rem = 256px by default)
                    .child(Input::new(&self.text_input).w_64())
                    .child(
                        Button::new("clear")
                            // Primary style gives the button a prominent appearance
                            .primary()
                            .label("Clear")
                            // Connect the button click to our handler method
                            // view_cx.listener() creates a callback that includes the view context
                            .on_click(view_cx.listener(Self::clear_input)),
                    ),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |app_cx: &mut App| {
        setup_app(app_cx);

        let prefs = WindowPreferences::default();

        // Window creation is async because it may need to query the display
        // for bounds calculation (especially for centering).
        app_cx
            .spawn(async move |async_cx| {
                let bounds = async_cx
                    .update(|app_cx: &mut App| Bounds::centered(None, prefs.size, app_cx))?;

                let _window_handle = async_cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |window: &mut gpui::Window, window_cx| {
                        // Create the main view
                        let view = window_cx.new(|view_cx: &mut Context<ButtonExample>| {
                            ButtonExample::new(window, view_cx)
                        });
                        // Wrap in Root for proper theming and focus management
                        window_cx.new(|root_cx| Root::new(view, window, root_cx))
                    },
                )?;

                Ok::<_, anyhow::Error>(())
            })
            .detach();
    });
}
