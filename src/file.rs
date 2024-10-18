use strum::EnumIter;
use strum::IntoEnumIterator;
use thiserror::Error;

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

    pub fn from_char(c: char) -> Result<Self, FileError> {
        match c {
            'a' | 'A' => Ok(Self::A),
            'b' | 'B' => Ok(Self::B),
            'c' | 'C' => Ok(Self::C),
            'd' | 'D' => Ok(Self::D),
            'e' | 'E' => Ok(Self::E),
            'f' | 'F' => Ok(Self::F),
            'g' | 'G' => Ok(Self::G),
            'h' | 'H' => Ok(Self::H),
            _ => Err(FileError::CreateFromChar(c.to_string())),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FileError {
    #[error("Unable to create file from char {0}")]
    CreateFromChar(String),
}
