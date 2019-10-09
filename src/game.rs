use crate::app::WebApp;
use crate::pieces::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub struct Game {
pub app: WebApp,
    board: [[Option<u8>; 10]; 20],
    cur_piece: Piece,
    new_piece: NewPieceStructure,
    total_time: f64,
}

#[derive(Copy, Clone, Default)]
pub struct Piece {
    x: u8,
    y: u8,
    piece: usize,
    rotation: usize,
}

impl Game {
    pub fn new(hook_id: &str) -> Game {
        let app = WebApp::new(hook_id);
        let board = [[None; 10]; 20];
        let mut new_piece = NewPieceStructure::new();
        let cur_piece = Piece::new(new_piece.get_next_piece());

        Game { 
            app, 
            board, 
            cur_piece,
            new_piece,
            total_time: 0.0,
        }
    }

    pub fn run(&mut self, etime: f64) {
        self.total_time += etime;

        if self.total_time > 1000.0 {
            self.draw_board();
            self.draw_piece(self.cur_piece);
            self.cur_piece = Piece::new(self.new_piece.get_next_piece());
            self.total_time = 0.0;
        }
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

    fn draw_piece(&mut self, piece: Piece) -> Result<(), JsValue> {
        let p = PIECES[piece.piece][piece.rotation];
        self.app.ctx.set_fill_style(&JsValue::from(format!("hsl({}, 100%, 50%",
                                                           piece.piece*45)));

        // Special case yellow b/c people's green cones are too powerful!
        if piece.piece == 2 {
            self.app.ctx.set_fill_style(&JsValue::from("hsl(60, 100%, 50%"));
        }

        for y in 0..4 {
            for x in 0..4 {
                if p[(y*4 + x) as usize] == 1 {
                    self.app.ctx.fill_rect((piece.x+x) as f64 * 10.0,
                                           (piece.y+y) as f64 * 10.0,
                                           10.0, 10.0);
                }
            }
        }

        Ok(())
    }
}

impl Piece {
    fn new(piece: usize) -> Piece {
        Piece {
            x: 3,
            y: 0,
            piece: piece,
            rotation: 0,
        }
    }
}
