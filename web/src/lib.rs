use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pretty_str(str: &str, width: usize) -> Result<String, JsValue> {
    mz_sql_pretty::pretty_str(str, width).map_err(|err| JsValue::from_str(&err.to_string()))
}

#[wasm_bindgen]
pub fn pretty_str_simple(s: &str, width: usize) -> String {
    match mz_sql_pretty::pretty_str(s, width) {
        Ok(s) => s,
        Err(_) => s.to_string(),
    }
}
