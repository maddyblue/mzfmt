use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn pretty_str(str: &str, width: usize) -> Result<String, JsValue> {
    mzfmt::pretty_str(str, width).map_err(|err| JsValue::from_str(&err))
}
