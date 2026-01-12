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

    let str = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(Token::String);

    let token = number.or(str);

    token
        .map_with(|t, e| (t, e.span()))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
