use crate::app::WebApp;
use crate::pieces::*;
use crate::events::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub struct Game {
pub app: WebApp,
    board: [[Option<u8>; 18]; 24], // HACK 0001: height is 20. add hidden board space for the piece array
    cur_piece: Piece,
    new_piece: NewPieceStructure,
    total_time: f64,
    speed: f64,
    over: bool,
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
        let mut board = [[None; 18]; 24];
        let mut new_piece = NewPieceStructure::new();
        let cur_piece = Piece::new(new_piece.get_next_piece());

        //HACK 0001: set bounds
        board[20] = [Some(7); 18];
        board[21] = [Some(7); 18];
        board[22] = [Some(7); 18];
        board[23] = [Some(7); 18];
        for y in 0..20 {
            for x in 0..4 {
                board[y][x] = Some(7);
                board[y][x+14] = Some(7);
            }
        }

        Game { 
            app, 
            board, 
            cur_piece,
            new_piece,
            total_time: 0.0,
            speed: 1000.0,
            over: false,
        }
    }

    pub fn run(&mut self, etime: f64) {
        self.total_time += etime;

        if self.over {
            self.app.ctx.set_fill_style(&JsValue::from("black"));
            for y in 0..20 {
                for x in 4..14 {
                    self.app.ctx.fill_rect(((x-4) as f64) * 10.0, (y as f64) * 10.0, 10.0, 10.0);
                }
            }
            self.app.ctx.set_fill_style(&JsValue::from("white"));
            self.app.ctx.fill_text("Game\nOver", 23.0, 80.0);
            return;
        }

        if let Some(Event::KeyDownEvent(ev)) = self.app.get_next_event() {
            match ev.key().as_ref() {
                "ArrowUp" => {
                    self.cur_piece.rotation = (self.cur_piece.rotation + 1) % 4;
                    //HACK 0002: move piece, then move back if invalid
                    if !self.piece_valid(self.cur_piece) {
                        self.cur_piece.rotation = (self.cur_piece.rotation - 1) % 4;
                    }
                },
                "ArrowLeft" => {
                    if self.cur_piece.x != 0 {
                        self.cur_piece.x = self.cur_piece.x - 1;
                        //HACK 0002: move piece, then move back if invalid
                        if !self.piece_valid(self.cur_piece) {
                            self.cur_piece.x = (self.cur_piece.x + 1);
                        }
                    }
                },
                "ArrowRight" => {
                    self.cur_piece.x = self.cur_piece.x + 1;
                    //HACK 0002: move piece, then move back if invalid
                    if !self.piece_valid(self.cur_piece) {
                        self.cur_piece.x = (self.cur_piece.x - 1);
                    }
                },
                _ => {},
            }
        }

        if self.total_time > self.speed {
            self.total_time = 0.0;

            // 1 "game tick. Move piece
            self.cur_piece.y = self.cur_piece.y + 1;
            if !self.piece_valid(self.cur_piece) {
                //HACK 0002: move piece, then move back if invalid
                self.cur_piece.y = self.cur_piece.y - 1;
                self.lock_piece(self.cur_piece);
                self.cur_piece = Piece::new(self.new_piece.get_next_piece());

                if !self.piece_valid(self.cur_piece) {
                    self.over = true;
                }
            }
        }

        self.draw_board();
        self.draw_piece(self.cur_piece);
    }

    fn draw_board(&mut self) -> Result<(), JsValue> {
        for y in 0..20 {
            for x in 4..14 {
                if let Some(c) = self.board[y][x] {
                    self.app.ctx.set_fill_style(&get_piece_color(c as usize));
                } else {
                    self.app.ctx.set_fill_style(&JsValue::from("black"));
                }

                self.app.ctx.fill_rect(((x-4) as f64) * 10.0, (y as f64) * 10.0, 10.0, 10.0);
            }
        }

        Ok(())
    }

    fn draw_piece(&mut self, piece: Piece) -> Result<(), JsValue> {
        let p = PIECES[piece.piece][piece.rotation];
        self.app.ctx.set_fill_style(&get_piece_color(piece.piece));

        for y in 0..4 {
            for x in 0..4 {
                if p[(y*4 + x) as usize] == 1 {
                    self.app.ctx.fill_rect((piece.x+x-4) as f64 * 10.0,
                                           (piece.y+y) as f64 * 10.0,
                                           10.0, 10.0);
                }
            }
        }

        Ok(())
    }

    fn lock_piece(&mut self, piece: Piece) {
       let p = PIECES[piece.piece][piece.rotation]; 

       for y in 0..4 {
           for x in 0..4 {
                if p[y*4 + x] == 1 {
                    self.board[piece.y as usize + y][piece.x as usize + x] = Some(piece.piece as u8);
                }
           }
       }
    }

    fn piece_valid(&self, piece: Piece) -> bool {
        let p = PIECES[piece.piece][piece.rotation];

        for y in (0..4).rev() {
            for x in 0..4 {
                if p[y*4+x] == 1 &&
                    self.board[piece.y as usize + y][piece.x as usize + x] != None {
                        return false;
                    }
            }
        }

        true
    }
}

impl Piece {
    fn new(piece: usize) -> Piece {
        Piece {
            x: 7,
            y: 0,
            piece: piece,
            rotation: 0,
        }
    }
}
