#![allow(dead_code)]
#![allow(unused_imports)]

use wasm_bindgen::prelude::*;

mod enums;
mod exports;
mod grids;
mod modules;
mod nodes;
mod traits;
mod types;

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!( $ ( $t )* ).into());
    };
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local, js_name = canvas)]
    static CANVAS: JsValue;
}
