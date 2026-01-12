#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'src> {
    Literal(&'src str),

    Boolean(bool),
}
