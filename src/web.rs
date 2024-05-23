use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen(module = "/web.js")]
extern "C" {
    async fn fetchStringFromServer(url: &str) -> JsValue;
    async fn fetchBytesFromServer(url: &str) -> JsValue;
}

pub async fn get_string(url: &str) -> Result<String, JsValue> {
    let result = fetchStringFromServer(url).await;
    match result.as_string() {
        Some(s) => Ok(s),
        None => Err(JsValue::from_str("Failed to fetch string from server")),
    }
}

pub async fn get_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
    let result = fetchBytesFromServer(url).await;
    if let Some(uint8_array) = result.dyn_ref::<Uint8Array>() {
        Ok(uint8_array.to_vec())
    } else {
        Err(JsValue::from_str("Failed to fetch bytes from server"))
    }
}

pub fn add_text_field(title: &str, id: &str) {
    let overlay = web_sys::window().unwrap().document().unwrap().get_element_by_id("overlay").unwrap();

    let mut new_html = overlay.inner_html();
    new_html += "\n";
    new_html += format!("<div>{title}: <span id=\"{id}\"></span></div>").as_str();

    overlay.set_inner_html(new_html.as_str());
}
