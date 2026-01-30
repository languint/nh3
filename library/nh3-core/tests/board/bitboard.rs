#[cfg(test)]
mod tests {
    use nh3_core::{
        board::Bitboard,
        square::{File, Rank, Square},
    };

    #[test]
    fn from_file() {
        assert_eq!(
            Bitboard::file_mask(&File::from_index(0)),
            Bitboard::from_u64(0xFF)
        );

        assert_eq!(
            Bitboard::file_mask(&File::from_index(7)),
            Bitboard::from_u64(0xFF00000000000000)
        );
    }

    #[test]
    fn from_rank() {
        assert_eq!(
            Bitboard::rank_mask(&Rank::from_index(0)),
            Bitboard::from_u64(0x101010101010101)
        );

        assert_eq!(
            Bitboard::rank_mask(&Rank::from_index(7)),
            Bitboard::from_u64(0x8080808080808080)
        );
    }

    #[test]
    fn from_square() {
        assert_eq!(
            Bitboard::square_mask(&Square::from_index(0)),
            Bitboard::from_u64(0x1)
        );

        assert_eq!(
            Bitboard::square_mask(&Square::from_index(63)),
            Bitboard::from_u64(0x8000000000000000)
        );
    }
}
