#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use nh3_core::board::CastlingRights;

    #[test]
    fn from_str() {
        assert_eq!(CastlingRights::from_str("kqKQ"), Ok(CastlingRights::ALL));
    }
}
