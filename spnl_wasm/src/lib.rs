use wasm_bindgen::prelude::*;

use spnl::from_str;

#[wasm_bindgen]
pub fn execute_query(query: &str) -> Result<(), JsError> {
    let program = from_str(query)?;

    Ok(())
}

#[wasm_bindgen]
pub fn compile_query(query: &str) -> Result<String, JsError> {
    let program = from_str(query)?;

    //Ok(serde_wasm_bindgen::to_value(&program)?)
    Ok(serde_json::to_string(&program)?)
}
