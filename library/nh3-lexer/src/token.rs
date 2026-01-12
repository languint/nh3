#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    LParen,
    RParen,

    Identifier(&'src str),

    Number(f64),
}
