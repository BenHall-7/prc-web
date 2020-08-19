#![recursion_limit = "256"]

mod app;
mod components;
mod utils;

use prc::hash40::{set_custom_labels, Hash40};
use wasm_bindgen::prelude::*;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    utils::set_panic_hook();
    yew::start_app::<app::App>();
    Ok(())
}

#[wasm_bindgen]
pub fn load_labels(text: String) {
    let iterator = text.lines().filter_map(|line| {
        let mut split = line.split(',');
        let hash_opt = split.next();
        let label_opt = split.next();

        if let Some(hash_str) = hash_opt {
            if let Some(label) = label_opt {
                if let Ok(hash) = Hash40::from_hex_str(hash_str) {
                    return Some((hash, String::from(label)));
                }
            }
        }

        None
    });

    set_custom_labels(iterator);
}
