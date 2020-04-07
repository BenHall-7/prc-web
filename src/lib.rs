#![recursion_limit="256"]

mod app;
mod utils;
mod components;

use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    yew::start_app::<app::App>();
    Ok(())
}
