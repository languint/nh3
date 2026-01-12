#[cfg(test)]
mod tests {
    use chumsky::{Parser, error::Simple};
    use nh3_lexer::Token;

    #[test]
    fn test_lex_int() {
        let src = "42";

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![(
                Token::Number(42.0),
                nh3_lexer::Span {
                    start: 0,
                    end: 2,
                    context: ()
                }
            )])
        )
    }

    #[test]
    fn test_lex_float() {
        let src = "42.5";

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![(
                Token::Number(42.5),
                nh3_lexer::Span {
                    start: 0,
                    end: 4,
                    context: ()
                }
            )])
        )
    }

    #[test]
    fn test_lex_string() {
        let src = r#""Hello world!""#;

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![(
                Token::String("Hello world!"),
                nh3_lexer::Span {
                    start: 0,
                    end: 14,
                    context: ()
                }
            )])
        )
    }

    #[test]
    fn test_lex_identifier() {
        let src = r#"fn name() {}"#;

        let lex = nh3_lexer::lexer().parse(src);

        assert!(!lex.has_errors());
        assert_eq!(
            lex.into_output(),
            Some(vec![
                (
                    Token::Fn,
                    nh3_lexer::Span {
                        start: 0,
                        end: 2,
                        context: ()
                    }
                ),
                (
                    Token::Identifier("name"),
                    nh3_lexer::Span {
                        start: 3,
                        end: 7,
                        context: ()
                    }
                ),
                (
                    Token::Control('('),
                    nh3_lexer::Span {
                        start: 7,
                        end: 8,
                        context: ()
                    }
                ),
                (
                    Token::Control(')'),
                    nh3_lexer::Span {
                        start: 8,
                        end: 9,
                        context: ()
                    }
                ),
                (
                    Token::Control('{'),
                    nh3_lexer::Span {
                        start: 10,
                        end: 11,
                        context: ()
                    }
                ),
                (
                    Token::Control('}'),
                    nh3_lexer::Span {
                        start: 11,
                        end: 12,
                        context: ()
                    }
                )
            ])
        )
    }
}
