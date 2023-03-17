use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pretty_str(str: &str, width: usize) -> Result<String, JsValue> {
    mzfmt::pretty_str(str, width).map_err(|err| JsValue::from_str(&err.to_string()))
}
