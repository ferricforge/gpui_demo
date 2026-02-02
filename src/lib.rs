pub mod components;
pub mod preferences;

use gpui::*;

actions!(gpui_demo, [Quit]);

// Takes a reference to the action (often unused) and mutable app context
pub fn quit(
    _: &Quit,
    cx: &mut App,
) {
    println!("Executing the Quit handler");
    cx.quit();
}

pub fn setup_app(cx: &mut App) {
    // This must be called before using any GPUI Component features.
    gpui_component::init(cx);

    cx.activate(true);

    // Bind Cmd+Q to the Quit action
    cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);

    // Register the quit action handler
    cx.on_action(quit);

    // Set up the application menu with Quit
    cx.set_menus(vec![
        Menu {
            name: "App".into(),
            items: vec![MenuItem::action("Quit", Quit)],
        },
    ]);
}
