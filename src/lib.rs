#![allow(non_snake_case)]
// this is just some necessary code for wasm compilation and integration with nextjs.
// good examples and documentation: https://github.com/wasm-bindgen/wasm-bindgen
use wasm_bindgen::prelude::*;

mod steg;

#[wasm_bindgen]
pub fn encrypt(text: &str, image: &[u8]) -> Result<Vec<u8>, JsValue> {
    steg::tencryptimage(text, image).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn decrypt(image: &[u8]) -> Result<String, JsValue> {
    steg::imagedecrypttext(image).map_err(|e| JsValue::from_str(&e))
}
