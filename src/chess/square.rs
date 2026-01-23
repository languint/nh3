#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Square(u8);

// CONSTRUCTORS //

impl Square {
    #[inline]
    pub const fn new(idx: u8) -> Self {
        debug_assert!(idx < 64, "Square index out of range");

        Self(idx)
    }

    #[inline]
    pub const fn from_file_rank(file: u8, rank: u8) -> Self {
        debug_assert!(file < 8, "Square file out of range");
        debug_assert!(rank < 8, "Rank file out of range");

        Self(file + (rank * 8))
    }
}

// METHODS //

impl Square {
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    #[inline]
    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    #[inline]
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + self.file()) as char,
            (b'1' + self.rank()) as char
        )
    }
}

// TESTS //

#[cfg(test)]
mod tests {
    use crate::chess::square::Square;

    #[test]
    fn from_index() {
        for i in 0..63 {
            Square::new(i);
        }
    }

    #[test]
    fn from_file_rank() {
        let a1 = Square::from_file_rank(0, 0);
        let h8 = Square::from_file_rank(7, 7);

        assert_eq!(a1.0, 0);
        assert_eq!(h8.0, 63);
    }

    #[test]
    fn rank_file_index() {
        let a1 = Square::new(0);
        let h8 = Square::new(63);

        assert_eq!(a1.file(), 0);
        assert_eq!(a1.rank(), 0);

        assert_eq!(h8.file(), 7);
        assert_eq!(h8.rank(), 7);
    }

    #[test]
    fn display() {
        let a1 = Square::new(0);
        let h8 = Square::new(63);

        assert_eq!(a1.to_string(), "a1");
        assert_eq!(h8.to_string(), "h8");
    }
}
