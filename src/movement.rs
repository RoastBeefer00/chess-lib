use std::error::Error;

use crate::square::Square;
use crate::piece::PieceType;

pub enum SpecialMove {
    Promotion(PieceType),
    // EnPassant,
    CastleKingside,
    CastleQueenside,
}

pub struct Move {
    pub from: Square,
    pub to: Square,
    pub special: Option<SpecialMove>,
} 

impl Move {
    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     todo!();
    // }
}
