use crate::piece::{Piece, PieceType, Color};
use strum::{IntoEnumIterator, EnumIter};

#[derive(Debug, PartialEq)]
pub struct Board {
    pub squares: Vec<Square>,
}

impl Default for Board {
    fn default() -> Self {
        let mut squares: Vec<Square> = Vec::new();
        for rank in Rank::iter() {
            for file in File::iter() {
                match rank {
                    Rank::One => {
                        match file {
                            File::A | File::H => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Rook,
                                        color: Color::White,
                                    }),
                                });
                            },
                            File::B | File::G => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Knight,
                                        color: Color::White,
                                    }),
                                });
                            },
                            File::C | File::F => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Bishop,
                                        color: Color::White,
                                    }),
                                });
                            },
                            File::D => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Queen,
                                        color: Color::White,
                                    }),
                                });
                            },
                            File::E => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::King,
                                        color: Color::White,
                                    }),
                                });
                            },
                        }
                    },
                    Rank::Two => {
                        squares.push(Square {
                            file,
                            rank,
                            piece: Some(Piece {
                                piece: PieceType::Pawn,
                                color: Color::White,
                            }),
                        });
                    },
                    Rank::Three | Rank::Four | Rank::Five | Rank::Six => {
                        squares.push(Square {
                            file,
                            rank,
                            piece: None,
                        });
                    }, 
                    Rank::Seven => {
                        squares.push(Square {
                            file,
                            rank,
                            piece: Some(Piece {
                                piece: PieceType::Pawn,
                                color: Color::Black,
                            }),
                        });
                    },
                    Rank::Eight => {
                        match file {
                            File::A | File::H => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Rook,
                                        color: Color::Black,
                                    }),
                                });
                            },
                            File::B | File::G => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Knight,
                                        color: Color::Black,
                                    }),
                                });
                            },
                            File::C | File::F => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Bishop,
                                        color: Color::Black,
                                    }),
                                });
                            },
                            File::D => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::Queen,
                                        color: Color::Black,
                                    }),
                                });
                            },
                            File::E => {
                                squares.push(Square {
                                    file,
                                    rank,
                                    piece: Some(Piece {
                                        piece: PieceType::King,
                                        color: Color::Black,
                                    }),
                                });
                            },
                        }
                    },
                }
            }
        }
        
        Self {
            squares
        }
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
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

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
    pub piece: Option<Piece>,
}
