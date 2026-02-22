use crate::app::App;

pub mod app;
pub mod pages;
pub mod user_data;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
