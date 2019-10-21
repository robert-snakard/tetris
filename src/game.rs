use crate::app::WebApp;
use crate::constants::*;
use crate::constants::TETRONIMO_DIMENSION as TD;
use crate::constants::SQUARE_LENGTH_IN_PIXELS as SQUARED;
use crate::pieces::*;
use crate::events::*;

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

pub struct Game {
    board: [[Option<u8>; ACTUAL_WIDTH]; ACTUAL_HEIGHT],
    cur_piece: Piece,
    npgen: NewPieceGenerator,
    total_time: f64,
    speed: f64,
    game_over: bool,
}

#[derive(Copy, Clone, Default)]
pub struct Piece {
    x: usize,
    y: usize,
    piece: usize,
    rotation: usize,
}

impl Game {
    pub fn new() -> Game {
        let mut npgen = NewPieceGenerator::new();
        let cur_piece = Piece::new(npgen.get_next_piece());

        //HACK 0001: set bounds
        let mut board = [[Some(NUM_PIECES); ACTUAL_WIDTH]; ACTUAL_HEIGHT];
        for y in 0..BOARD_HEIGHT {
            for x in TD..(BOARD_WIDTH+TD) {
                board[y][x] = None;
            }
        }

        Game { 
            board, 
            cur_piece,
            npgen,
            total_time: 0.0,
            speed: 1000.0,
            game_over: false,
        }
    }

    pub fn run(&mut self, etime: f64, app: &mut WebApp) {
        self.total_time += etime;

        if self.game_over {
            Game::print_game_over(app.get_ctx());
            return;
        }

        if let Some(Event::KeyDownEvent(ev)) = app.get_next_event() {
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
                // If piece can't move down lock it on the board and spawn a new one
                self.lock_piece(self.cur_piece);
                self.cur_piece = Piece::new(self.npgen.get_next_piece());

                // if spawning a piece is invalid set game over
                if !self.piece_valid(self.cur_piece) {
                    self.game_over = true;
                }
            }
        }

        self.draw_board(app.get_ctx());
        self.draw_piece(self.cur_piece, app.get_ctx());
    }

    fn draw_board(&mut self, ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                if let Some(c) = self.board[y][x+TD] {
                    ctx.set_fill_style(&get_piece_color(c as usize));
                } else {
                    ctx.set_fill_style(&JsValue::from("black"));
                }

                ctx.fill_rect((x as f64) * SQUARED, 
                                       (y as f64) * SQUARED,
                                       SQUARED, SQUARED);
            }
        }

        Ok(())
    }

    fn draw_piece(&mut self, piece: Piece,
                  ctx: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let p = PIECES[piece.piece][piece.rotation];
        ctx.set_fill_style(&get_piece_color(piece.piece));

        for y in 0..TD {
            for x in 0..TD {
                if p[(y*TD + x) as usize] == 1 {
                    ctx.fill_rect(((piece.x-TD)+x) as f64 * SQUARED,
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

    fn print_game_over(ctx: &CanvasRenderingContext2d) {
        ctx.set_fill_style(&JsValue::from("black"));
        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                ctx.fill_rect((x as f64) * SQUARED,
                (y as f64) * SQUARED,
                SQUARED, SQUARED);
            }
        }
        ctx.set_fill_style(&JsValue::from("white"));
        ctx.fill_text("Game\nOver", 23.0, 80.0);
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
