use thiserror::Error;
use crate::square::Square;
use crate::piece::{PieceType, Color};
use crate::rank::Rank;
use crate::file::File;
use crate::piece::Piece;
use crate::board::Board;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialMove {
    Promotion(PieceType),
    // EnPassant,
    CastleKingside,
    CastleQueenside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: (Square, Piece),
    pub to: Square,
    pub is_capture: bool,
    pub special: Option<SpecialMove>,
} 

impl Default for Move {
    fn default() -> Self {
        Self {
            from: (Square::new(File::E, Rank::Two), Piece::new(PieceType::Pawn, Color::White)),
            to: Square::new(File::E, Rank::Four),
            is_capture: false,
            special: None,
        }
    }
}

impl Move {
    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     todo!();
    // }
    
    pub fn up(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() + amount) {
                    let to_square = Square::new(square.file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::UpError(amount, square, *from_piece))
                }
            },
            Color::Black => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() - amount) {
                    let to_square = Square::new(square.file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::UpError(amount, square, *from_piece))
                }
            },
        }
    }

    pub fn down(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() - amount) {
                    let to_square = Square::new(square.file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DownError(amount, square, *from_piece))
                }
            },
            Color::Black => {
                if let Ok(rank) = Rank::from_usize(square.rank.value() + amount) {
                    let to_square = Square::new(square.file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DownError(amount, square, *from_piece))
                }
            },
        }
    }

    pub fn left(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let Ok(file) = File::from_usize(square.file.value() - amount) {
                    let to_square = Square::new(file, square.rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::LeftError(amount, square, *from_piece))
                }
            },
            Color::Black => {
                if let Ok(file) = File::from_usize(square.file.value() + amount) {
                    let to_square = Square::new(file, square.rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::LeftError(amount, square, *from_piece))
                }
            },
        }
    }

    pub fn right(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let Ok(file) = File::from_usize(square.file.value() + amount) {
                    let to_square = Square::new(file, square.rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::RightError(amount, square, *from_piece))
                }
            },
            Color::Black => {
                if let Ok(file) = File::from_usize(square.file.value() - amount) {
                    let to_square = Square::new(file, square.rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::RightError(amount, square, *from_piece))
                }
            },
        }
    }

    pub fn diag_up_left(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() + amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagUpLeftError(amount, square, *from_piece))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() - amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagUpLeftError(amount, square, *from_piece))
                }                               
            },
        }
    }

    pub fn diag_up_right(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() + amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagUpRightError(amount, square, *from_piece))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() - amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagUpRightError(amount, square, *from_piece))
                }                               
            },
        }
    }

    pub fn diag_down_left(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() - amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagDownLeftError(amount, square, *from_piece))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() + amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagDownLeftError(amount, square, *from_piece))
                }                               
            },
        }
    }

    pub fn diag_down_right(board: &Board, amount: usize, square: Square) -> Result<Move, MoveError> {
        let from_piece = match board.get_piece(&square) {
            Ok(piece) => {
                match piece {
                    Some(p) => p,
                    None => return Err(MoveError::PieceNotFound(square)),
                }
            },
            Err(_) => return Err(MoveError::PieceNotFound(square)),
        };

        match from_piece.color {
            Color::White => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() + amount), Rank::from_usize(square.rank.value() - amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagDownRightError(amount, square, *from_piece))
                }                               
            },
            Color::Black => {
                if let (Ok(file), Ok(rank)) = (File::from_usize(square.file.value() - amount), Rank::from_usize(square.rank.value() + amount)) {
                    let to_square = Square::new(file, rank);
                    let to_piece = match board.get_piece(&to_square) {
                        Ok(piece) => piece,
                        Err(_) => return Err(MoveError::PieceNotFound(square)),
                    };

                    let mut m = Move {
                        from: (square, *from_piece),
                        to: to_square,
                        ..Default::default()
                    };

                    if to_piece.is_some() {
                        m.is_capture = true;
                    }

                    Ok(m)
                } else {
                    Err(MoveError::DiagDownRightError(amount, square, *from_piece))
                }                               
            },
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MoveError {
    #[error("Cannot move up amount {0} from square {1:?} with piece {2:?}")]
    UpError(usize, Square, Piece),
    #[error("Cannot move down amount {0} from square {1:?} with piece {2:?}")]
    DownError(usize, Square, Piece),
    #[error("Cannot move left amount {0} from square {1:?} with piece {2:?}")]
    LeftError(usize, Square, Piece),
    #[error("Cannot move right amount {0} from square {1:?} with piece {2:?}")]
    RightError(usize, Square, Piece),
    #[error("Cannot move diagonally up+left amount {0} from square {1:?} with piece {2:?}")]
    DiagUpLeftError(usize, Square, Piece),
    #[error("Cannot move diagonally up+right amount {0} from square {1:?} with piece {2:?}")]
    DiagUpRightError(usize, Square, Piece),
    #[error("Cannot move diagonally down+left amount {0} from square {1:?} with piece {2:?}")]
    DiagDownLeftError(usize, Square, Piece),
    #[error("Cannot move diagonally down+right amount {0} from square {1:?} with piece {2:?}")]
    DiagDownRightError(usize, Square, Piece),
    #[error("Unable to find piece at square {0:?}")]
    PieceNotFound(Square),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up() {
        let board = Board::default();
        let square = Square::new(File::D, Rank::Two);
        let piece = board.get_piece(&square).unwrap().unwrap();        
        let up = Move::up(&board, 1, square).unwrap();
        assert_eq!(Move {
            from: (square, piece),
            to: Square::new(File::D, Rank::Three),
            ..Default::default()
        }, up);

        let square = Square::new(File::D, Rank::Three);
        assert_eq!(Err(MoveError::PieceNotFound(square)), Move::up(&board, 1, square));
    }
}
