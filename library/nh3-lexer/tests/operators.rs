#[cfg(test)]
mod tests {
    use chumsky::Parser;
    use nh3_lexer::Token;

    #[test]
    fn test_lex_add() {
        let src = "x + 2";

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![
                (
                    Token::Identifier("x"),
                    nh3_lexer::Span {
                        start: 0,
                        end: 1,
                        context: ()
                    }
                ),
                (
                    Token::Operator("+"),
                    nh3_lexer::Span {
                        start: 2,
                        end: 3,
                        context: ()
                    }
                ),
                (
                    Token::Number(2.0),
                    nh3_lexer::Span {
                        start: 4,
                        end: 5,
                        context: ()
                    }
                )
            ])
        )
    }

    #[test]
    fn test_lex_eq_eq() {
        let src = "x == 2";

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![
                (
                    Token::Identifier("x"),
                    nh3_lexer::Span {
                        start: 0,
                        end: 1,
                        context: ()
                    }
                ),
                (
                    Token::Operator("=="),
                    nh3_lexer::Span {
                        start: 2,
                        end: 4,
                        context: ()
                    }
                ),
                (
                    Token::Number(2.0),
                    nh3_lexer::Span {
                        start: 5,
                        end: 6,
                        context: ()
                    }
                )
            ])
        )
    }
}
