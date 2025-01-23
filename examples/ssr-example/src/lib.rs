pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;

    const LEPTOS_HYDRATED: &str = "_leptos_hydrated"; 

    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);

}
