use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Piece {
    pub piece: PieceType,
    pub color: Color,
}

#[derive(EnumIter, Debug, PartialEq, Copy, Clone)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> usize {
        match self {
            Self::Pawn => 1,
            Self::Knight => 3,
            Self::Bishop => 3,
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => 0,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}
