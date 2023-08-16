use game::LAUNCHER_TITLE;
use yew::prelude::*;

fn set_window_title(title: &str) {
    web_sys::window()
        .and_then(|w| w.document())
        .expect("Unable to get DOM")
        .set_title(title);
}

#[function_component(Root)]
fn view() -> Html {
    set_window_title(LAUNCHER_TITLE);

    html! {
        <> </>
    }
}

fn main() {
    #[cfg(feature = "inspect")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    // Mount the DOM
    yew::Renderer::<Root>::new().render();
    // Start the Bevy App
    log::info!("Starting launcher: WASM");
    game::app(false).run();
}
