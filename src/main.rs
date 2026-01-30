use gpui::*;
use gpui_component::{button::*, *};

actions!(gpui_demo, [Quit]);

// Takes a reference to the action (often unused) and mutable app context
fn quit(
    _: &Quit,
    cx: &mut App,
) {
    println!("Executing the Quit handler");
    cx.quit();
}

#[allow(unused)]
fn click_handler(
    event: &ClickEvent,
    window: &mut Window,
    cx: &mut App,
) {
    println!("Clicked!")
}

pub struct HelloWorld {
    _window_close_subscription: Option<Subscription>,
}

impl HelloWorld {
    fn new(cx: &mut Context<Self>) -> Self {
        let subscription = cx.on_window_closed(|cx: &mut App| {
            println!("Window closed callback!");
            quit(&Quit, cx);
        });

        Self {
            _window_close_subscription: Some(subscription),
        }
    }
}

impl Render for HelloWorld {
    fn render(
        &mut self,
        _: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(click_handler),
            )
    }
}

fn main() {
    let app = Application::new();

    app.run(move |cx: &mut App| {
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

        cx.spawn(async move |cx| {
            let _window_handle = cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| HelloWorld::new(cx));
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            // Get the root entity from the window - needs to be done inside cx.update()
            // let root_entity = cx.update(|cx| window_handle.entity(cx))??;

            // Observe when the root entity is released (window closes)
            // cx.update(|cx| {
            //     cx.observe_release(&root_entity, |_, cx| {
            //         cx.dispatch_action(&Quit);
            //         println!("Calling quit explicitly");
            //         quit(&Quit, cx);
            //     })
            //     .detach();
            // })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
