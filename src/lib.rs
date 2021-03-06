use js_sys::Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use mod_c;
use rs_audio;
use rs_image;
use rs_text;
use rs_zip;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

#[wasm_bindgen]
pub fn greet_c(n: u32) -> u32 {
    mod_c::hi(n)
}

#[wasm_bindgen]
pub fn synth(freq: f32, sec: u32, sample_rate: u32) -> Vec<f32> {
    rs_audio::synth(freq, sec, sample_rate)
}

#[derive(Serialize, Deserialize)]
pub struct JsImage {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
pub fn process(buffer: &[u8]) -> Result<JsValue, JsValue> {
    let res = rs_image::process(buffer).or_else(|e| Err(e.to_string()))?;
    JsValue::from_serde(&JsImage {
        pixels: res.pixels,
        width: res.width,
        height: res.height,
    })
    .or_else(|e| Err(e.to_string().into()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<string>")]
    pub type StringArray;
}

#[wasm_bindgen]
pub fn tokenize(text: String) -> StringArray {
    rs_text::tokenize(text)
        .iter()
        .map(JsValue::from)
        .collect::<Array>()
        .unchecked_into::<StringArray>()
}

#[wasm_bindgen]
pub fn archive(buffer: &[u8]) -> Result<Vec<u8>, JsValue> {
    rs_zip::gzip(buffer).or_else(|e| Err(e.to_string().into()))
}
