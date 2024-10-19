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

    pub fn from_usize(u: usize) -> Result<Self, FileError> {
        match u {
            1 => Ok(Self::A),
            2 => Ok(Self::B),
            3 => Ok(Self::C),
            4 => Ok(Self::D),
            5 => Ok(Self::E),
            6 => Ok(Self::F),
            7 => Ok(Self::G),
            8 => Ok(Self::H),
            _ => Err(FileError::CreateFromUsize(u)),
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum FileError {
    #[error("Unable to create file from char {0}")]
    CreateFromChar(String),
    #[error("Unable to create file from usize {0}")]
    CreateFromUsize(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_values() {
        let mut value = 1;
        File::iter().for_each(|file| {
            assert_eq!(value, file.value());
            value += 1;
        });
    }

    #[test]
    fn test_file_from_char() {
        let a = File::from_char('a').unwrap();
        assert_eq!(File::A, a);
        let b = File::from_char('b').unwrap();
        assert_eq!(File::B, b);
        let c = File::from_char('c').unwrap();
        assert_eq!(File::C, c);
        let d = File::from_char('d').unwrap();
        assert_eq!(File::D, d);
        let e = File::from_char('e').unwrap();
        assert_eq!(File::E, e);
        let f = File::from_char('f').unwrap();
        assert_eq!(File::F, f);
        let g = File::from_char('g').unwrap();
        assert_eq!(File::G, g);
        let h = File::from_char('h').unwrap();
        assert_eq!(File::H, h);

        assert_eq!(Err(FileError::CreateFromChar('j'.to_string())), File::from_char('j'));
        assert_eq!(Err(FileError::CreateFromChar('x'.to_string())), File::from_char('x'));
    }
}
