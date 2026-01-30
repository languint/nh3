use std::str::FromStr;

use crate::square::{file::File, rank::Rank};

mod file;
mod rank;

#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(u8);

impl Square {
    pub const fn from_index(index: u8) -> Square {
        debug_assert!(index < 64, "Square::from_index, index out of range");

        Square(index)
    }

    pub const fn from_rank_file(rank: &Rank, file: &File) -> Square {
        Square::from_index(rank.index() * 8 + file.index())
    }
}

impl FromStr for Square {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(format!("Square::from_str, invalid length: {}", s.len()));
        }

        let chars: Vec<char> = s.chars().collect();

        let file = File::try_from(chars[0])
            .map_err(|e| format!("Square::from_str, invalid file: {e}!"))?;
        let rank = Rank::try_from(chars[1])
            .map_err(|e| format!("Square::from_str, invalid rank: {e}!"))?;

        Ok(Square::from_rank_file(&rank, &file))
    }
}

impl Square {
    pub const fn index(self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::square::Square;

    #[test]
    fn square_from_str() {
        assert_eq!(Square::from_str("a1"), Ok(Square(0)));
        assert_eq!(Square::from_str("h8"), Ok(Square(63)));

        for r in '1'..='8' {
            for f in 'a'..='h' {
                let mut string = String::new();
                string.push(f);
                string.push(r);

                assert!(Square::from_str(&string).is_ok())
            }
        }
    }
}
