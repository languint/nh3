use std::str::FromStr;

mod file;
mod rank;

pub use file::File;
pub use rank::Rank;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(u8);

impl Square {
    #[must_use]
    pub const fn from_index(index: u8) -> Square {
        debug_assert!(index < 64, "Square::from_index, index out of range");

        Square(index)
    }

    #[must_use]
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
    #[must_use]
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    #[must_use]
    #[inline]
    pub const fn rank(&self) -> Rank {
        Rank::from_index(self.0 / 8)
    }

    #[must_use]
    #[inline]
    pub const fn file(&self) -> File {
        File::from_index(self.0 % 8)
    }
}
