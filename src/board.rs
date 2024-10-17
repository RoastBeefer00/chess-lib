use crate::piece::{self, Color, Piece, PieceType};
use std::collections::HashMap;
use crate::movement::{Move, SpecialMove};
use strum::{EnumIter, IntoEnumIterator};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Board {
    pub squares: HashMap<Square, Option<Piece>>,
}

impl Default for Board {
    fn default() -> Self {
        let mut squares: HashMap<Square, Option<Piece>> = HashMap::new();
        for rank in Rank::iter() {
            for file in File::iter() {
                match rank {
                    Rank::One => match file {
                        File::A | File::H => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    piece: PieceType::Rook,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::B | File::G => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    piece: PieceType::Knight,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::C | File::F => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    piece: PieceType::Bishop,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::D => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    piece: PieceType::Queen,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::E => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    piece: PieceType::King,
                                    color: Color::White,
                                }),
                            );
                        }
                    },
                    Rank::Two => {
                        squares.insert(
                            Square { file, rank },
                            Some(Piece {
                                piece: PieceType::Pawn,
                                color: Color::White,
                            }),
                        );
                    }
                    Rank::Three | Rank::Four | Rank::Five | Rank::Six => {
                        squares.insert(Square { file, rank }, None);
                    }
                    Rank::Seven => {
                        squares.insert(
                            Square { file, rank },
                            Some(Piece {
                                piece: PieceType::Pawn,
                                color: Color::Black,
                            }),
                        );
                    }
                    Rank::Eight => match file {
                        File::A | File::H => {
                            squares.insert(
                                Square {
                                file,
                                rank,
                                },
                                Some(Piece {
                                    piece: PieceType::Rook,
                                    color: Color::Black,
                                }),
                            );
                        }
                        File::B | File::G => {
                            squares.insert(
                                Square {
                                file,
                                rank,
                                },
                                Some(Piece {
                                    piece: PieceType::Knight,
                                    color: Color::Black,
                                }),
                            );
                        }
                        File::C | File::F => {
                            squares.insert(
                                Square {
                                file,
                                rank,
                                },
                                Some(Piece {
                                    piece: PieceType::Bishop,
                                    color: Color::Black,
                                }),
                            );
                        }
                        File::D => {
                            squares.insert(
                                Square {
                                file,
                                rank,
                                },
                                Some(Piece {
                                    piece: PieceType::Queen,
                                    color: Color::Black,
                                }),
                            );
                        }
                        File::E => {
                            squares.insert(
                                Square {
                                file,
                                rank,
                                },
                                Some(Piece {
                                    piece: PieceType::King,
                                    color: Color::Black,
                                }),
                            );
                        }
                    },
                }
            }
        }

        Self { squares }
    }
}

impl Board {
    pub fn count_material(&self, color: piece::Color) -> usize {
        self.squares
            .iter()
            .filter(|s| {
                if let Some(piece) = s.1 {
                    piece.color == color
                } else {
                    false
                }
            })
            .map(|s| {
                if let Some(piece) = s.1 {
                    piece.piece.value()
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn get_piece(&mut self, square: &Square) -> Result<&Option<Piece>, SquareError> {
        let search = self
            .squares.get(square);
        match search {
            Some(piece) => Ok(piece),
            None => Err(SquareError::NotFound(square.file, square.rank)),
        }
    }
    
    pub fn make_move(&mut self, m: Move) -> Result<(), SquareError> {
        // Get piece on "from" square to be placed in the "to" square
        let from_piece = match self.get_piece(&m.from) {
            Ok(piece) => piece.to_owned(),
            Err(e) => return Err(e),
        };
        // Validate the "to" square exists
        if let Err(e) = self.get_piece(&m.to) {
            return Err(e);
        }
        // Update pieces in squares
        {
            self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
        }
        {
            self.squares.entry(m.from).and_modify(|piece| *piece = None);
        }

        Ok(())
    }
}

#[derive(Eq, Hash, EnumIter, Debug, PartialEq, Clone, Copy)]
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
    pub fn value(self) -> usize {
        self as usize
    }
}

#[derive(Eq, Hash, EnumIter, Debug, PartialEq, Clone, Copy)]
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
    pub fn value(self) -> usize {
        self as usize
    }
}

#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("Unable to find square at rank {1:?} and file {0:?}")]
    NotFound(File, Rank),
}
