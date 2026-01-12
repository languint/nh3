#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Identifier(&'src str),
    Number(f64),
    Boolean(bool),
    String(&'src str),

    Control(char),
    Operator(&'src str),

    Fn,
    Let,
    If,
    Else,
    Return,

    ThinArrow,
    FatArrow,
}
