#[derive(Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct Rank(u8);

impl Rank {
    pub const fn from_index(index: u8) -> Rank {
        debug_assert!(index < 8, "Rank::from_index, index out of range");

        Rank(index)
    }
}

impl TryFrom<char> for Rank {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1'..='8' => Ok(Rank::from_index(value as u8 - ('0' as u8) - 1)),
            _ => Err(format!("Rank::try_from::<char>, invalid rank char {value}")),
        }
    }
}

impl Rank {
    #[inline]
    pub const fn index(&self) -> u8 {
        self.0
    }
}
