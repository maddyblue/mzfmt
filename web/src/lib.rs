use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pretty_str(str: &str, width: usize) -> String {
    match mz_sql_pretty::pretty_strs(str, width) {
        Ok(strs) => strs.join("\n\n"),
        Err(e) => e.to_string(),
    }
}
