use crate::piece::{self, Color, Piece, PieceType};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use crate::movement::{Move, SpecialMove};
use crate::file::File;
use crate::rank::Rank;
use crate::square::{Square, SquareError};

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
                                    unit: PieceType::Rook,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::B | File::G => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    unit: PieceType::Knight,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::C | File::F => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    unit: PieceType::Bishop,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::D => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    unit: PieceType::Queen,
                                    color: Color::White,
                                }),
                            );
                        }
                        File::E => {
                            squares.insert(
                                Square { file, rank },
                                Some(Piece {
                                    unit: PieceType::King,
                                    color: Color::White,
                                }),
                            );
                        }
                    },
                    Rank::Two => {
                        squares.insert(
                            Square { file, rank },
                            Some(Piece {
                                unit: PieceType::Pawn,
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
                                unit: PieceType::Pawn,
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
                                    unit: PieceType::Rook,
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
                                    unit: PieceType::Knight,
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
                                    unit: PieceType::Bishop,
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
                                    unit: PieceType::Queen,
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
                                    unit: PieceType::King,
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
                    piece.unit.value()
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn get_piece(&self, square: &Square) -> Result<&Option<Piece>, SquareError> {
        let search = self
            .squares.get(square);
        match search {
            Some(piece) => Ok(piece),
            None => Err(SquareError::NotFound(square.file, square.rank)),
        }
    }
    
    pub fn make_move(&mut self, m: Move) -> Result<(), SquareError> {
        if let Some(s) = m.special {
            // Get piece on "from" square 
            let from_piece = match self.get_piece(&m.from) {
                Ok(piece) => piece.to_owned(),
                Err(e) => return Err(e),
            };

            // Validate the "to" square exists
            self.get_piece(&m.to)?;

            if let Some(piece) = from_piece {
                match s {
                    SpecialMove::Promotion(unit) => {
                        if unit == PieceType::Pawn {
                            return Err(SquareError::InvalidPromotion)
                        }

                        match piece.color {
                            Color::White => {
                                if m.to.rank != Rank::Eight {
                                    return Err(SquareError::InvalidPromotion);
                                }
                            },
                            Color::Black => {
                                if m.to.rank != Rank::One {
                                    return Err(SquareError::InvalidPromotion);
                                }
                            },
                        };

                        let promotion_piece = Some(Piece {
                            unit,
                            color: piece.color,
                        });
                        // Validate the "to" square exists
                        self.get_piece(&m.to)?;
                        // Update pieces in squares
                        {
                            self.squares.entry(m.to).and_modify(|piece| *piece = promotion_piece);
                        }
                        {
                            self.squares.entry(m.from).and_modify(|piece| *piece = None);
                        }
                    },
                    SpecialMove::CastleKingside => {
                        if piece.unit != PieceType::King {
                            return Err(SquareError::CastleWithKing);
                        }

                        match piece.color {
                            Color::White => {
                                let e1 = Square {
                                    file: File::E,
                                    rank: Rank::One,
                                };
                                let g1 = Square {
                                    file: File::G,
                                    rank: Rank::One,
                                };
                                if m.from != e1 || m.to != g1 {
                                    return Err(SquareError::CastleKingsideWhite);
                                }

                                let rook = match self.get_piece(&Square {
                                    file: File::H,
                                    rank: Rank::One,
                                }) {
                                    Ok(piece) => piece.to_owned(),
                                    Err(e) => return Err(e),
                                };

                                // Move the rook to f1
                                {
                                    self.squares.entry(Square { 
                                        file: File::F, 
                                        rank: Rank::One 
                                    }).and_modify(|piece| *piece = rook);
                                }
                                // Move the king to g1
                                {
                                    self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
                                }
                                // Make e1 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::E,
                                        rank: Rank::One
                                    }).and_modify(|piece| *piece = None);
                                }
                                // Make h1 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::H,
                                        rank: Rank::One
                                    }).and_modify(|piece| *piece = None);
                                }
                            },
                            Color::Black => {
                                let e8 = Square {
                                    file: File::E,
                                    rank: Rank::Eight,
                                };
                                let g8 = Square {
                                    file: File::G,
                                    rank: Rank::Eight,
                                };
                                if m.from != e8 || m.to != g8 {
                                    return Err(SquareError::CastleKingsideWhite);
                                }

                                let rook = match self.get_piece(&Square {
                                    file: File::H,
                                    rank: Rank::Eight,
                                }) {
                                    Ok(piece) => piece.to_owned(),
                                    Err(e) => return Err(e),
                                };

                                // Move the rook to f8
                                {
                                    self.squares.entry(Square { 
                                        file: File::F, 
                                        rank: Rank::Eight
                                    }).and_modify(|piece| *piece = rook);
                                }
                                // Move the king to g8
                                {
                                    self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
                                }
                                // Make e8 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::E,
                                        rank: Rank::Eight
                                    }).and_modify(|piece| *piece = None);
                                }
                                // Make h8 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::H,
                                        rank: Rank::Eight
                                    }).and_modify(|piece| *piece = None);
                                }
                            },
                        }
                    },
                    SpecialMove::CastleQueenside => {
                         if piece.unit != PieceType::King {
                            return Err(SquareError::CastleWithKing);
                        }

                        match piece.color {
                            Color::White => {
                                let e1 = Square {
                                    file: File::E,
                                    rank: Rank::One,
                                };
                                let c1 = Square {
                                    file: File::C,
                                    rank: Rank::One,
                                };
                                if m.from != e1 || m.to != c1 {
                                    return Err(SquareError::CastleQueenSideWhite);
                                }

                                let rook = match self.get_piece(&Square {
                                    file: File::A,
                                    rank: Rank::One,
                                }) {
                                    Ok(piece) => piece.to_owned(),
                                    Err(e) => return Err(e),
                                };

                                // Move the rook to d1
                                {
                                    self.squares.entry(Square { 
                                        file: File::D, 
                                        rank: Rank::One 
                                    }).and_modify(|piece| *piece = rook);
                                }
                                // Move the king to c1
                                {
                                    self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
                                }
                                // Make e1 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::E,
                                        rank: Rank::One
                                    }).and_modify(|piece| *piece = None);
                                }
                                // Make a1 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::A,
                                        rank: Rank::One
                                    }).and_modify(|piece| *piece = None);
                                }
                            },
                            Color::Black => {
                                let e8 = Square {
                                    file: File::E,
                                    rank: Rank::Eight,
                                };
                                let c8 = Square {
                                    file: File::C,
                                    rank: Rank::Eight,
                                };
                                if m.from != e8 || m.to != c8 {
                                    return Err(SquareError::CastleQueenSideBlack);
                                }

                                let rook = match self.get_piece(&Square {
                                    file: File::A,
                                    rank: Rank::Eight,
                                }) {
                                    Ok(piece) => piece.to_owned(),
                                    Err(e) => return Err(e),
                                };

                                // Move the rook to d8
                                {
                                    self.squares.entry(Square { 
                                        file: File::D, 
                                        rank: Rank::Eight 
                                    }).and_modify(|piece| *piece = rook);
                                }
                                // Move the king to c8
                                {
                                    self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
                                }
                                // Make e8 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::E,
                                        rank: Rank::Eight
                                    }).and_modify(|piece| *piece = None);
                                }
                                // Make h8 empty
                                {
                                    self.squares.entry(Square {
                                        file: File::H,
                                        rank: Rank::Eight
                                    }).and_modify(|piece| *piece = None);
                                }
                            },
                        }
                   },
                }
            } else {
                return Err(SquareError::MoveEmptySquare);
            }
        } else {
            // Get piece on "from" square to be placed in the "to" square
            let from_piece = match self.get_piece(&m.from) {
                Ok(piece) => piece.to_owned(),
                Err(e) => return Err(e),
            };

            if from_piece == None {
                return Err(SquareError::MoveEmptySquare)
            }

            // Validate the "to" square exists
            self.get_piece(&m.to)?;
            // Update pieces in squares
            {
                self.squares.entry(m.to).and_modify(|piece| *piece = from_piece);
            }
            {
                self.squares.entry(m.from).and_modify(|piece| *piece = None);
            }
        }

        Ok(())
    }
}



