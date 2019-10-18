use crate::app::WebApp;
use crate::constants::*;
use crate::constants::TETRONIMO_DIMENSION as TD;
use crate::constants::SQUARE_LENGTH_IN_PIXELS as SQUARED;
use crate::pieces::*;
use crate::events::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;

pub struct Game {
pub app: WebApp,
    board: [[Option<u8>; ACTUAL_WIDTH]; ACTUAL_HEIGHT],
    cur_piece: Piece,
    npgen: NewPieceGenerator,
    total_time: f64,
    speed: f64,
    over: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Piece {
    x: usize,
    y: usize,
    piece: usize,
    rotation: usize,
}

impl Game {
    pub fn new(hook_id: &str) -> Game {
        let app = WebApp::new(hook_id);
        let mut board = [[Some(7); ACTUAL_WIDTH]; ACTUAL_HEIGHT];
        let mut new_piece = NewPieceStructure::new();
        let cur_piece = Piece::new(new_piece.get_next_piece());

        //HACK 0001: set bounds
        for y in 0..BOARD_HEIGHT {
            for x in TD..(BOARD_WIDTH+TD) {
                board[y][x] = None;
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
            for y in 0..BOARD_HEIGHT {
                for x in 0..BOARD_WIDTH {
                    self.app.ctx.fill_rect((x as f64) * SQUARED,
                                           (y as f64) * SQUARED, 
                                           SQUARED, SQUARED);
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

                // if spawning a piece is invalid set game over
                if !self.piece_valid(self.cur_piece) {
                    self.over = true;
                }
            }
        }

        self.draw_board();
        self.draw_piece(self.cur_piece);
    }

    fn draw_board(&mut self) -> Result<(), JsValue> {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if let Some(c) = self.board[y][x+TD] {
                    self.app.ctx.set_fill_style(&get_piece_color(c as usize));
                } else {
                    self.app.ctx.set_fill_style(&JsValue::from("black"));
                }

                self.app.ctx.fill_rect((x as f64) * SQUARED, 
                                       (y as f64) * SQUARED,
                                       SQUARED, SQUARED);
            }
        }

        Ok(())
    }

    fn draw_piece(&mut self, piece: Piece) -> Result<(), JsValue> {
        let p = PIECES[piece.piece][piece.rotation];
        self.app.ctx.set_fill_style(&get_piece_color(piece.piece));

        for y in 0..TD {
            for x in 0..TD {
                if p[(y*TD + x) as usize] == 1 {
                    self.app.ctx.fill_rect(((piece.x-TD)+x) as f64 * SQUARED,
                                           (piece.y+y) as f64 * SQUARED,
                                           SQUARED, SQUARED);
                }
            }
        }

        Ok(())
    }

    fn lock_piece(&mut self, piece: Piece) {
       let p = PIECES[piece.piece][piece.rotation]; 

       for y in 0..TETRONIMO_DIMENSION {
           for x in 0..TETRONIMO_DIMENSION {
                if p[y*TD + x] == 1 {
                    self.board[piece.y as usize + y][piece.x as usize + x] = Some(piece.piece as u8);
                }
           }
       }
    }

    fn piece_valid(&self, piece: Piece) -> bool {
        let p = PIECES[piece.piece][piece.rotation];

        for y in (0..TD).rev() {
            for x in 0..TETRONIMO_DIMENSION {
                if p[y*TD+x] == 1 &&
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
            x: NUM_PIECES,
            y: 0,
            piece: piece,
            rotation: 0,
        }
    }
}
