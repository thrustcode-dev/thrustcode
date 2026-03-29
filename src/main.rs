mod app;
mod components;

fn main() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(app::App);
}
