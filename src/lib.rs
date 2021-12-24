#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate console_error_panic_hook;
#[cfg(feature = "release")]
extern crate wee_alloc;
use std::panic;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod eips;
pub mod faq;
pub mod footer;
pub mod nav;
pub mod p2p;
pub mod search;
pub mod slides;
pub mod view;
use wasm_bindgen::prelude::*;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<view::View>::new().mount_as_body();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log2(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);

    // #[wasm_bindgen(js_namespace = window.ethereum, js_name = request)]
    // fn request(m: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! console_warn {
    ($($t:tt)*) => (warn(&format_args!($($t)*).to_string()))
}
