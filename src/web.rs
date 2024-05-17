use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(module = "/web.js")]
extern "C" {
    fn getStringFromServer(url: &str) -> JsValue;
}

#[wasm_bindgen]
pub fn get_string(url: &str) -> Result<String, JsValue> {
    let result = getStringFromServer(url);
    match result.as_string() {
        Some(s) => Ok(s),
        None => Err(JsValue::from_str("Failed to fetch string from server")),
    }
}
