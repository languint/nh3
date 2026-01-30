#[cfg(test)]
mod tests {
    use nh3_core::square::Rank;

    #[test]
    fn from_char() {
        for c in '1'..='8' {
            assert!(Rank::try_from(c).is_ok());
        }
    }

    #[test]
    fn from_char_out_of_range() {
        assert!(Rank::try_from('0').is_err());
        assert!(Rank::try_from('9').is_err());
    }
}
