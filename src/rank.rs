use strum::EnumIter;
use strum::IntoEnumIterator;
use thiserror::Error;

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

    pub fn from_char(c: char) -> Result<Self, RankError> {
        match c {
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            _ => Err(RankError::CreateFromChar(c.to_string()))
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RankError {
    #[error("Unable to create rank from char {0}")]
    CreateFromChar(String),
}
