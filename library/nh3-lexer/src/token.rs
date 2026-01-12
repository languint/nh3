#[derive(Debug, PartialEq)]
pub enum Token<'src> {
    Identifier(&'src str),
    Number(f64),
    Bool(bool),
    String(&'src str),

    Control(char),
}
