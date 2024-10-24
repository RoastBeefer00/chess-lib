use crate::player::Player;
use crate::board::Board;
use crate::piece::{Color, PieceType};
use crate::movement::Move;
use crate::square::SquareError;

pub struct Game {
    pub board: Board,
    pub white: Player,
    pub black: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self { 
            board: Board::default(), 
            white: Player::new(Color::White), 
            black: Player::new(Color::Black), 
        }
    }
}

impl Game {
    pub fn make_move(&mut self, m: Move) -> Result<(), SquareError> {
        let result = self.board.make_move(m);
        if result.is_ok() {
            match m.from.1.color {
                Color::White => {
                    if m.from.1.unit == PieceType::King {
                        self.white.remove_kingside_castle();
                        self.white.remove_queenside_castle();
                    }
                },
                Color::Black => {
                    if m.from.1.unit == PieceType::King {
                        self.black.remove_kingside_castle();
                        self.black.remove_queenside_castle();
                    }
                },
            }
        }    

        result
    }
}
