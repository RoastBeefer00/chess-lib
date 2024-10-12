pub struct Piece {
    pub piece: PieceType,
    pub color: Color,
}

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    fn value(&self) -> usize {
        match self {
            Self::Pawn => 1,
            Self::Knight => 8,
            Self::Bishop => 8,
            Self::Rook => 5,
            Self::Queen => 9,
            Self::King => 0,
        }
    }
}

pub enum Color {
    Black,
    White,
}
