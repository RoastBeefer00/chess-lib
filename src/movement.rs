use crate::board::Square;
use crate::piece::{Piece, PieceType};

pub enum SpecialMove {
    Promotion(PieceType),
    // EnPassant,
    CastleKingside,
    CastleQueenside,
}

pub struct Move<'a> {
    from: &'a mut Square,
    to: &'a mut Square,
    special: Option<SpecialMove>,
} 

