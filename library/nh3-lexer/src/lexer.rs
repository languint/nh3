use chumsky::prelude::*;

use crate::token::Token;

pub type Span = SimpleSpan;
pub type Spanned<T> = (T, Span);

pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>> {
    let number = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Number);

    let token = number;

    token
        .map_with(|t, e| (t, e.span()))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        lexer::{Span, lexer},
        token::Token,
    };

    #[test]
    fn lex_int() {
        let lex_result = lexer().parse("42");

        assert!(!lex_result.has_errors());
        assert_eq!(
            lex_result.output(),
            Some(&vec![(
                Token::Number(42.0),
                Span {
                    start: 0,
                    end: 2,
                    context: ()
                }
            )])
        )
    }
}
