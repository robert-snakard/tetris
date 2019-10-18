mod app;
mod constants;
mod events;
mod game;
mod pieces;

use crate::game::Game;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

const HOOK_ID: &str = "tetris";

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let mut game = Rc::new(RefCell::new(Game::new(HOOK_ID)));

    pub fn run(game: Rc<RefCell<Game>>) -> Result<(), JsValue> {
        let mut old_time = 0.0;
        let mut etime = 0.0;

        let g2 = game.clone();
        let lp1 = Rc::new(RefCell::new(None));
        let lp2 = lp1.clone();

        *lp2.borrow_mut() = Some(Closure::wrap(Box::new(move |new_time: f64| {
            etime = new_time - old_time;
            old_time = new_time;
            game.borrow_mut().run(etime);

            game.borrow_mut().app.request_animation_frame(lp1.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>));
        g2.borrow_mut().app.request_animation_frame(lp2.borrow().as_ref().unwrap());
       
        Ok(())
    } run(game.clone());

    Ok(())
}
