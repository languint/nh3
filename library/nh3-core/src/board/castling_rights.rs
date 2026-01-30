use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct CastlingRights(u8);
impl CastlingRights {
    pub const WK: CastlingRights = CastlingRights(0b0001);
    pub const WQ: CastlingRights = CastlingRights(0b0010);
    pub const BK: CastlingRights = CastlingRights(0b0100);
    pub const BQ: CastlingRights = CastlingRights(0b1000);

    pub const ALL: CastlingRights = CastlingRights(0b1111);
    pub const NONE: CastlingRights = CastlingRights(0b0000);
}

impl std::ops::BitOrAssign for CastlingRights {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl FromStr for CastlingRights {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rights = CastlingRights::NONE;

        for c in s.chars() {
            match c {
                'K' => rights |= CastlingRights::WK,
                'Q' => rights |= CastlingRights::WQ,
                'k' => rights |= CastlingRights::BK,
                'q' => rights |= CastlingRights::BQ,
                _ => return Err(format!("CastlingRights::from_str, invalid char: {c}!")),
            }
        }

        Ok(rights)
    }
}
