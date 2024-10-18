use thiserror::Error;
use crate::file::File;
use crate::rank::Rank;


#[derive(Eq, Hash, Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub file: File,
    pub rank: Rank,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self {
            file,
            rank,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, SquareError> {
       if s.len() != 2 {
           return Err(SquareError::CreateFromStr(s.to_string()));
       } 

       let file = match File::from_char(s.chars().nth(0).unwrap()) {
           Ok(f) => f,
           Err(_) => return Err(SquareError::CreateFromStr(s.to_string())),
       };
 
       let rank = match Rank::from_char(s.chars().nth(1).unwrap()) {
           Ok(r) => r,
           Err(_) => return Err(SquareError::CreateFromStr(s.to_string())),
       };
 
       Ok(Self {
           file,
           rank,
       })
    }
}

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("Unable to find square at rank {1:?} and file {0:?}")]
    NotFound(File, Rank),
    #[error("Unable to create square from str {0}")]
    CreateFromStr(String),
    #[error("Promotions must be on the first or eight rank and cannot be a pawn")]
    InvalidPromotion,
    #[error("Cannot make a move from an empty square")]
    MoveEmptySquare,
    #[error("Must select King when castling")]
    CastleWithKing,
    #[error("Must move king from e1 to g1 when castling kingside with white")]
    CastleKingsideWhite,
    #[error("Must move king from e1 to c1 when castling kingside with white")]
    CastleQueenSideWhite,
    #[error("Must move king from e8 to g8 when castling kingside with black")]
    CastleKingsideBlack,
    #[error("Must move king from e8 to c8 when castling kingside with black")]
    CastleQueenSideBlack,
}
