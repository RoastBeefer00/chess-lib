use thiserror::Error;
use crate::square::Square;
use crate::piece::{PieceType, Color};
use crate::rank::Rank;
use crate::file::File;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialMove {
    Promotion(PieceType),
    // EnPassant,
    CastleKingside,
    CastleQueenside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub special: Option<SpecialMove>,
} 

impl Move {
    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     todo!();
    // }
    
    pub fn up(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() + amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(square.file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::UpError(amount, square, color))
                }
            },
            Color::Black => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() - amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(square.file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::UpError(amount, square, color))
                }
            },
        }
    }

    pub fn down(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() - amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(square.file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DownError(amount, square, color))
                }
            },
            Color::Black => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() + amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(square.file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DownError(amount, square, color))
                }
            },
        }
    }

    pub fn left(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let Ok(file) = File::from_usize(square.file.value() - amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, square.rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::LeftError(amount, square, color))
                }
            },
            Color::Black => {
                if let Ok(file) = File::from_usize(square.file.value() + amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, square.rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::LeftError(amount, square, color))
                }
            },
        }
    }

    pub fn right(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let Ok(file) = File::from_usize(square.file.value() + amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, square.rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::RightError(amount, square, color))
                }
            },
            Color::Black => {
                if let Ok(file) = File::from_usize(square.file.value() - amount) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, square.rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::RightError(amount, square, color))
                }
            },
        }
    }

    pub fn diag_up_left(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() + amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagUpLeftError(amount, square, color))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() - amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagUpLeftError(amount, square, color))
                }                               
            },
        }
    }

    pub fn diag_up_right(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() + amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagUpRightError(amount, square, color))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() - amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagUpRightError(amount, square, color))
                }                               
            },
        }
    }

    pub fn diag_down_left(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() - amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagDownLeftError(amount, square, color))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() + amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagDownLeftError(amount, square, color))
                }                               
            },
        }
    }

    pub fn diag_down_right(amount: usize, square: Square, color: Color) -> Result<Move, MoveError> {
        match color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() - amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagDownRightError(amount, square, color))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() + amount)) {
                    Ok(Move {
                        from: square,
                        to: Square::new(file, rank),
                        special: None,
                    })
                } else {
                    Err(MoveError::DiagDownRightError(amount, square, color))
                }                               
            },
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveError {
    #[error("Cannot move up amount {0} from square {1:?} with color {2:?}")]
    UpError(usize, Square, Color),
    #[error("Cannot move down amount {0} from square {1:?} with color {2:?}")]
    DownError(usize, Square, Color),
    #[error("Cannot move left amount {0} from square {1:?} with color {2:?}")]
    LeftError(usize, Square, Color),
    #[error("Cannot move right amount {0} from square {1:?} with color {2:?}")]
    RightError(usize, Square, Color),
    #[error("Cannot move diagonally up+left amount {0} from square {1:?} with color {2:?}")]
    DiagUpLeftError(usize, Square, Color),
    #[error("Cannot move diagonally up+right amount {0} from square {1:?} with color {2:?}")]
    DiagUpRightError(usize, Square, Color),
    #[error("Cannot move diagonally down+left amount {0} from square {1:?} with color {2:?}")]
    DiagDownLeftError(usize, Square, Color),
    #[error("Cannot move diagonally down+right amount {0} from square {1:?} with color {2:?}")]
    DiagDownRightError(usize, Square, Color),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up() {
        let square = Square::new(File::D, Rank::Six);
        let up = Move::up(1, square, Color::White).unwrap();
        assert_eq!(Move {
            from: square,
            to: Square::new(File::D, Rank::Seven),
            special: None,
        }, up);

        let square = Square::new(File::D, Rank::Eight);
        assert_eq!(Err(MoveError::UpError(1, square, Color::White)), Move::up(1, square, Color::White));
    }
}
