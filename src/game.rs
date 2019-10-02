use crate::app::WebApp;
use crate::events::*;

use wasm_bindgen::prelude::*;

pub struct Game {
    app: WebApp,
    board: [[u8; 10]; 20],
}

impl Game {
    pub fn new(hook_id: &str) -> Game {
        let app = WebApp::new(hook_id);
        let board = [[0 as u8; 10]; 20];

        Game { app, board }
    }

    pub fn run(&mut self) -> Result<(), JsValue> {
        self.draw_board()?;
       
        Ok(())
    }

    pub fn draw_board(&mut self) -> Result<(), JsValue> {
        self.app.ctx.set_fill_style(&JsValue::from("green"));
        self.app.ctx.fill_rect(0.0, 0.0, 50.0, 50.0);

        Ok(())
    }
}
