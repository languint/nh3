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

    let string = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(Token::String);

    let arrow = just("->")
        .map(|_| Token::ThinArrow)
        .or(just("=>").map(|_| Token::FatArrow));

    let op = one_of("+*-/!=")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Operator);

    let control = one_of("()[]{};:,").map(Token::Control);

    let identifier = text::ascii::ident().map(|ident: &str| match ident {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "true" => Token::Boolean(true),
        "false" => Token::Boolean(false),
        "if" => Token::If,
        "else" => Token::Else,
        "return" => Token::Return,
        _ => Token::Identifier(ident),
    });

    let token = number
        .or(string)
        .or(arrow)
        .or(op)
        .or(control)
        .or(identifier);

    token
        .map_with(|token, err| (token, err.span()))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
