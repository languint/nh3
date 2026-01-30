#[cfg(test)]
mod tests {
    use nh3_core::square::File;

    #[test]
    fn from_char() {
        for c in 'a'..='h' {
            assert!(File::try_from(c).is_ok());
        }
    }

    #[test]
    fn from_char_out_of_range() {
        assert!(File::try_from('j').is_err());
        assert!(File::try_from('0').is_err());
    }
}
