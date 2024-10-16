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

pub fn make_move(to: &mut Square, from: &mut Square, special: Option<SpecialMove>) {
    if let Some(s) = special {
        match s {
            SpecialMove::Promotion(piece_type) => {
                to.piece = Some(Piece {
                    piece: piece_type,
                    color: to.piece.unwrap().color,
                });
                from.piece = None;
            },
            // SpecialMove::EnPassant => {},
            SpecialMove::CastleKingside => {},
            SpecialMove::CastleQueenside => {},
        };
    } else {
        to.piece = from.piece;
        from.piece = None;
    }
}
