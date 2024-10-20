use strum::EnumIter;
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

    pub fn from_usize(u: usize) -> Result<Self, RankError> {
        match u {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            _ => Err(RankError::CreateFromUsize(u))
        }
    }
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RankError {
    #[error("Unable to create rank from char {0}")]
    CreateFromChar(String),
    #[error("Unable to create rank from usize {0}")]
    CreateFromUsize(usize),
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_rank_values() {
        let mut value = 1;
        Rank::iter().for_each(|rank| {
            assert_eq!(value, rank.value());
            value += 1;
        });
    }

    #[test]
    fn test_rank_from_char() {
        let one = Rank::from_char('1').unwrap();
        assert_eq!(Rank::One, one);
        let two = Rank::from_char('2').unwrap();
        assert_eq!(Rank::Two, two);
        let three = Rank::from_char('3').unwrap();
        assert_eq!(Rank::Three, three);
        let four = Rank::from_char('4').unwrap();
        assert_eq!(Rank::Four, four);
        let five = Rank::from_char('5').unwrap();
        assert_eq!(Rank::Five, five);
        let six = Rank::from_char('6').unwrap();
        assert_eq!(Rank::Six, six);
        let seven = Rank::from_char('7').unwrap();
        assert_eq!(Rank::Seven, seven);
        let eight = Rank::from_char('8').unwrap();
        assert_eq!(Rank::Eight, eight);

        assert_eq!(Err(RankError::CreateFromChar('0'.to_string())), Rank::from_char('0'));
        assert_eq!(Err(RankError::CreateFromChar('9'.to_string())), Rank::from_char('9'));
    }
}
