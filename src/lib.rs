use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

const HOOK_NAME: &str = "tetris";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // get context
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id(HOOK_NAME).unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas.get_context("2d")?.unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // demo canvas
    canvas.set_width(WIDTH);
    canvas.set_height(HEIGHT);
    ctx.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

    Ok(())
}
