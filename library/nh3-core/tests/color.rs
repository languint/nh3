#[cfg(test)]
mod tests {
    use nh3_core::color::Color;

    #[test]
    fn color_from_char() {
        assert_eq!(Color::try_from('w'), Ok(Color::White));
        assert_eq!(Color::try_from('b'), Ok(Color::Black));
    }
}
