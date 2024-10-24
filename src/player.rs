use crate::piece::Color;
use crate::board::Board;

pub struct Player {
     pub color: Color,
     pub castle_kingside: bool,
     pub castle_queenside: bool,
     pub material: usize,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self {
            color,
            castle_kingside: true,
            castle_queenside: true,
            material: 39,
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
