use rand::prelude::*;
use wasm_bindgen::prelude::*;
use crate::game::Piece;

pub const PIECES: [[[u8; 16]; 4]; 7] = [
[// Z
    [1, 1, 0, 0,
    0, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 0, 1, 0,
    0, 1, 1, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
    [0, 0, 0, 0,
    1, 1, 0, 0,
    0, 1, 1, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    1, 1, 0, 0,
    1, 0, 0, 0,
    0, 0, 0, 0]
], [// L
    [0, 0, 1, 0,
    1, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    0, 1, 0, 0,
    0, 1, 1, 0,
    0, 0, 0, 0],
    [0, 0, 0, 0,
    1, 1, 1, 0,
    1, 0, 0, 0,
    0, 0, 0, 0],
    [1, 1, 0, 0,
    0, 1, 0, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
], [// O
    [0, 1, 1, 0,
    0, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 1, 0,
    0, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 1, 0,
    0, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 1, 0,
    0, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
], [// S
    [0, 1, 1, 0,
    1, 1, 0, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    0, 1, 1, 0,
    0, 0, 1, 0,
    0, 0, 0, 0],
    [0, 0, 0, 0,
    0, 1, 1, 0,
    1, 1, 0, 0,
    0, 0, 0, 0],
    [1, 0, 0, 0,
    1, 1, 0, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
], [// I
    [0, 0, 0, 0,
    1, 1, 1, 1,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 0, 1, 0,
    0, 0, 1, 0,
    0, 0, 1, 0,
    0, 0, 1, 0],
    [0, 0, 0, 0,
    0, 0, 0, 0,
    1, 1, 1, 1,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    0, 1, 0, 0,
    0, 1, 0, 0,
    0, 1, 0, 0],
], [// J
    [1, 0, 0, 0,
    1, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 1, 0,
    0, 1, 0, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
    [0, 0, 0, 0,
    1, 1, 1, 0,
    0, 0, 1, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    0, 1, 0, 0,
    1, 1, 0, 0,
    0, 0, 0, 0],
], [// T
    [0, 1, 0, 0,
    1, 1, 1, 0,
    0, 0, 0, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    0, 1, 1, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
    [0, 0, 0, 0,
    1, 1, 1, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
    [0, 1, 0, 0,
    1, 1, 0, 0,
    0, 1, 0, 0,
    0, 0, 0, 0],
]];

pub struct NewPieceGenerator {
    grab_bag: Vec<usize>,
//    rng: ThreadRng,
}

impl NewPieceGenerator {
    pub fn new() -> NewPieceGenerator {
 //       let mut rng = rand::thread_rng();
        let mut grab_bag: Vec<usize> = (0..7).collect();
  //      grab_bag.shuffle(&mut rng);

        NewPieceGenerator {
                grab_bag,
   //             rng,
        }
    }

    pub fn get_next_piece(&mut self) -> usize {

        if self.grab_bag.is_empty() {
            self.populate_grab_bag();
        }
        self.grab_bag.pop().unwrap()
    }

    fn populate_grab_bag(&mut self) {
        self.grab_bag = (0..7).collect();
    //    self.grab_bag.shuffle(&mut self.rng);
    }
}

pub fn get_piece_color(piece: usize) -> JsValue {
   if piece == 2 {
        // Special case yellow b/c people's green cones are too powerful!
       JsValue::from("hsl(60, 100%, 50%")
   } else {
       JsValue::from(format!("hsl({}, 100%, 50%", piece*45))
   }
}
