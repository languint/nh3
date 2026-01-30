use crate::square::{File, Rank, Square};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(transparent)]
pub struct Bitboard(u64);

impl Bitboard {
    #[must_use]
    #[inline]
    pub const fn from_u64(bb: u64) -> Bitboard {
        Bitboard(bb)
    }
}

impl Bitboard {
    #[must_use]
    #[inline]
    pub fn file_mask(file: &File) -> Bitboard {
        Bitboard(0xFF << (file.index() * 8))
    }

    #[must_use]
    #[inline]
    pub fn rank_mask(rank: &Rank) -> Bitboard {
        Bitboard(0x0101_0101_0101_0101 * u64::pow(2, u32::from(rank.index())))
    }

    #[must_use]
    #[inline]
    pub fn square_mask(square: &Square) -> Bitboard {
        Bitboard(1u64 << square.index())
    }
}

impl Bitboard {
    #[must_use]
    #[inline]
    pub fn get_value(self) -> u64 {
        self.0
    }

    #[must_use]
    #[inline]
    pub fn get_bit_at_square(&self, square: &Square) -> bool {
        (self.get_value() & Self::square_mask(square).get_value()) != 0
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in (0..=7).rev() {
            let mut file_str = String::new();
            for f in 0..=7 {
                let bit = self.get_bit_at_square(&Square::from_index(r * 8 + f));

                if bit {
                    file_str.push_str("1 ");
                } else {
                    file_str.push_str("0 ");
                }
            }
            writeln!(f, "{file_str}")?;
        }

        Ok(())
    }
}
