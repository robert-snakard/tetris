use wasm_bindgen::prelude::*;
use web_sys::*;

#[wasm_bindgen(start)]
pub fn start_game() {
    window().unwrap().alert().unwrap();
}
