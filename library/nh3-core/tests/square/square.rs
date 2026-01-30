#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use nh3_core::square::{File, Rank, Square};

    #[test]
    fn square_from_str() {
        assert_eq!(Square::from_str("a1"), Ok(Square::from_index(0)));
        assert_eq!(Square::from_str("h8"), Ok(Square::from_index(63)));

        for r in '1'..='8' {
            for f in 'a'..='h' {
                let mut string = String::new();
                string.push(f);
                string.push(r);

                assert!(Square::from_str(&string).is_ok())
            }
        }
    }

    #[test]
    fn square_rank_file() {
        for r in (0..=7).map(|r| Rank::from_index(r)) {
            for f in (0..=7).map(|f| File::from_index(f)) {
                let sq = Square::from_rank_file(&r, &f);
                assert_eq!(sq.file(), f);
                assert_eq!(sq.rank(), r);
            }
        }
    }
}
