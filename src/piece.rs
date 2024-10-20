use strum::EnumIter;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub unit: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(unit: PieceType, color: Color) -> Self {
        Self {
            unit,
            color,
        }
    }

}

#[derive(EnumIter, Debug, PartialEq, Eq, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_piece_values() {
        let values = vec![1, 3, 3, 5, 9, 0];
        PieceType::iter().enumerate().for_each(|(i, piece)| {
            assert_eq!(values[i], piece.value());
        });
    }

}
