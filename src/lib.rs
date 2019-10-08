mod app;
mod events;
mod game;
mod pieces;

use crate::game::Game;

use wasm_bindgen::prelude::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

const HOOK_ID: &str = "tetris";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut game = Game::new(HOOK_ID);
    game.run()?;

    Ok(())
}
