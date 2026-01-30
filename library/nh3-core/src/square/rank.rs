#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Rank(u8);
impl Rank {
    pub const FIRST: Rank = Rank(0);
    pub const SECOND: Rank = Rank(1);
    pub const THIRD: Rank = Rank(2);
    pub const FOURTH: Rank = Rank(3);
    pub const FIFTH: Rank = Rank(4);
    pub const SIXTH: Rank = Rank(5);
    pub const SEVENTH: Rank = Rank(6);
    pub const EIGHTH: Rank = Rank(7);

    pub const ALL: [Rank; 8] = [
        Rank::FIRST,
        Rank::SECOND,
        Rank::THIRD,
        Rank::FOURTH,
        Rank::FIFTH,
        Rank::SIXTH,
        Rank::SEVENTH,
        Rank::EIGHTH,
    ];
}

impl Rank {
    #[must_use]
    pub const fn from_index(index: u8) -> Rank {
        debug_assert!(index < 8, "Rank::from_index, index out of range");

        Rank(index)
    }
}

impl TryFrom<char> for Rank {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '1'..='8' => Ok(Rank::from_index(value as u8 - b'0' - 1)),
            _ => Err(format!("Rank::try_from::<char>, invalid rank char {value}")),
        }
    }
}

impl Rank {
    #[must_use]
    #[inline]
    pub const fn index(&self) -> u8 {
        self.0
    }
}
