use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
use components::app::App;

fn main() {
    // Initialize logging for debugging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <Router>
                <App/>
            </Router>
        }
    });
}
