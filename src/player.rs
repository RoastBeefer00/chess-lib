use crate::piece::Color;
use crate::board::Board;

pub struct Player {
     color: Color,
     castle_kingside: bool,
     castle_queenside: bool,
     material: usize,
}

impl Player {
    pub fn new(color: Color, board: Board) -> Self {
        Self {
            color,
            castle_kingside: true,
            castle_queenside: true,
            material: board.count_material(color),
        }
    }

    pub fn update_material(&mut self, board: Board) {
        self.material = board.count_material(self.color);
    }

    pub fn remove_kingside_castle(&mut self) {
       self.castle_kingside = false; 
    }

    pub fn remove_queenside_castle(&mut self) {
       self.castle_queenside = false; 
    }
}
