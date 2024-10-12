use crate::piece::{Piece, PieceType, Color};
use strum::EnumIter;

pub struct Board {
    squares: Vec<Square>,
}

impl Default for Board {
    fn default() -> Self {
        todo!();
    }
}

#[derive(EnumIter)]
pub enum Rank {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    fn value(self) -> usize {
        self as usize
    }
}

#[derive(EnumIter)]
pub enum File {
    A = 1,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    fn value(self) -> usize {
        self as usize
    }
}

pub struct Square {
    rank: Rank,
    file: File,
    piece: Option<Piece>,
}
