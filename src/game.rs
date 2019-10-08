use crate::app::WebApp;
use crate::events::*;
use crate::pieces::*;

use wasm_bindgen::prelude::*;

pub struct Game {
    app: WebApp,
    board: [[Option<u8>; 10]; 20],
}

impl Game {
    pub fn new(hook_id: &str) -> Game {
        let app = WebApp::new(hook_id);
        let board = [[None; 10]; 20];

        Game { app, board }
    }

    pub fn run(&mut self) -> Result<(), JsValue> {
        self.draw_board()?;
        
        self.draw_piece(0, 0, 0, 0);
        self.draw_piece(5, 0, 1, 0);
        self.draw_piece(0, 4, 2, 0);
        self.draw_piece(5, 4, 3, 0);
        self.draw_piece(0, 8, 4, 0);
        self.draw_piece(5, 8, 5, 0);
        self.draw_piece(3, 12, 6, 0);
       
        Ok(())
    }

    fn draw_board(&mut self) -> Result<(), JsValue> {
        for y in 0..20 {
            for x in 0..10 {
                if let Some(_) = self.board[y][x] {
                    self.app.ctx.set_fill_style(&JsValue::from("green"));
                } else {
                    self.app.ctx.set_fill_style(&JsValue::from("black"));
                }

                self.app.ctx.fill_rect((x as f64) * 10.0, (y as f64) * 10.0, 10.0, 10.0);
            }
        }

        Ok(())
    }

    fn draw_piece(&mut self, xcoord: u8, ycoord: u8, piece: usize, rotation: usize) -> Result<(), JsValue> {
        let p = PIECES[piece][rotation];
        self.app.ctx.set_fill_style(&JsValue::from(format!("hsl({}, 100%, 50%", piece*45)));

        // Special case yellow b/c people's green cones are too powerful!
        if piece == 2 {
            self.app.ctx.set_fill_style(&JsValue::from("hsl(60, 100%, 50%"));
        }

        for y in 0..4 {
            for x in 0..4 {
                if p[(y*4 + x) as usize] == 1 {
                    self.app.ctx.fill_rect((xcoord+x) as f64 * 10.0,
                                           (ycoord+y) as f64 * 10.0,
                                           10.0, 10.0);
                }
            }
        }

        Ok(())
    }
}
