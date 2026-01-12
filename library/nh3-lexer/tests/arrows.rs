#[cfg(test)]
mod tests {
    use chumsky::Parser;
    use nh3_lexer::Token;

    #[test]
    fn test_lex_add() {
        let src = "-> =>";

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![
                (
                    Token::ThinArrow,
                    nh3_lexer::Span {
                        start: 0,
                        end: 2,
                        context: ()
                    }
                ),
                (
                    Token::FatArrow,
                    nh3_lexer::Span {
                        start: 3,
                        end: 5,
                        context: ()
                    }
                ),
            ])
        )
    }
}
