pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;
pub const TETRONIMO_DIMENSION: usize = 4;

// Hack 0001: expand the board outside of the rendered area to make bounds checking easy
pub const ACTUAL_WIDTH: usize = TETRONIMO_DIMENSION + BOARD_WIDTH + TETRONIMO_DIMENSION;
pub const ACTUAL_HEIGHT: usize = BOARD_HEIGHT + TETRONIMO_DIMENSION;
pub const SCALING_FACTOR: f64 = 10.0;

pub const APP_WIDTH: f64 = (BOARD_WIDTH as f64) * SCALING_FACTOR;
pub const APP_HEIGHT: f64 = (BOARD_HEIGHT as f64) * SCALING_FACTOR;
pub const NUM_PIECES: u8 = 7;

pub const YLOWER: usize = 0;
pub const YUPPER: usize = BOARD_HEIGHT;
pub const XLOWER: usize = TETRONIMO_DIMENSION;
pub const XUPPER: usize = BOARD_WIDTH + TETRONIMO_DIMENSION;

