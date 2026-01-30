#[cfg(test)]
mod tests {
    use nh3_core::piece::Piece;

    #[test]
    fn piece_from_char() {
        for c in ['p', 'n', 'b', 'r', 'q', 'k'] {
            assert!(Piece::try_from(c).is_ok());
            assert!(Piece::try_from(c.to_ascii_uppercase()).is_ok());
        }
    }
}
