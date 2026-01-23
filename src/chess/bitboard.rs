use std::fmt::Write;

use crate::chess::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Bitboard(u64);
impl Bitboard {
    pub const EMPTY: Bitboard = Bitboard(0);
    pub const FULL: Bitboard = Bitboard(u64::MAX);
}

// CONSTRUCTORS //

impl Bitboard {
    #[inline]
    pub fn new(bb: u64) -> Self {
        Self(bb)
    }

    #[inline]
    pub const fn mask(sq: Square) -> Self {
        debug_assert!(sq.index() < 64, "Square index out of rank");

        Bitboard(1u64 << sq.index())
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Bitboard::new(1u64 << value.index())
    }
}

// METHODS //

impl Bitboard {
    #[inline]
    pub fn contains(self, sq: Square) -> bool {
        debug_assert!(sq.index() < 64);

        (self.0 >> sq.index()) & 1 != 0
    }
}

impl std::ops::BitAnd for Bitboard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Bitboard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl std::ops::Not for Bitboard {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_str = String::new();
        for r in (0..=7).rev() {
            let mut file_str = String::new();

            for f in 0..=7 {
                file_str.push(if self.contains(Square::from_file_rank(f, r)) {
                    '1'
                } else {
                    '0'
                });
            }

            writeln!(board_str, "{file_str}")?;
        }

        write!(f, "{board_str}")
    }
}

// TESTS //

#[cfg(test)]
mod tests {
    use crate::chess::{bitboard::Bitboard, square::Square};

    #[test]
    fn from_square() {
        let a1_bb = Bitboard::from(Square::new(0));
        assert_eq!(a1_bb, Bitboard::new(1u64));
    }

    #[test]
    fn mask() {
        let a8_mask = Bitboard::mask(Square::new(63));
        assert_eq!(a8_mask, Bitboard::new(1u64 << 63));
    }

    #[test]
    fn contains() {
        let e4_bb = Bitboard::mask(Square::new(28));
        assert_eq!(e4_bb.contains(Square::new(28)), true);
        assert_eq!(e4_bb.contains(Square::new(29)), false);
    }
}
