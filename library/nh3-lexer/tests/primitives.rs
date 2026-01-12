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
}
