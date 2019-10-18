pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const TETRONIMO_DIMENSION: usize = 4;

// Hack 0001: expand the board outside of the rendered area to make bounds checking easy
pub const ACTUAL_WIDTH: usize = TETRONIMO_DIMENSION + BOARD_WIDTH + TETRONIMO_DIMENSION;
pub const ACTUAL_HEIGHT: usize = BOARD_HEIGHT + TETRONIMO_DIMENSION;
pub const SQUARE_LENGTH_IN_PIXELS: f64 = 10.0;

pub const APP_WIDTH: f64 = (BOARD_WIDTH as f64) * SQUARE_LENGTH_IN_PIXELS;
pub const APP_HEIGHT: f64 = (BOARD_HEIGHT as f64) * SQUARE_LENGTH_IN_PIXELS;
pub const NUM_PIECES: u8 = 7;
