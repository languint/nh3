use crate::{codegen::CodegenInput, ir::literals::parse_number_literal};
use full_moon::{
    LuaVersion,
    ast::Stmt,
    node::Node,
    parse_fallible,
    tokenizer::{TokenReference, TokenType},
};

mod literals;

#[derive(Debug, PartialEq, Clone)]
pub enum IrNode {
    NumberLiteral(f64),
    StringLiteral(String),
    Variable {
        name: String,
        initial_value: Option<Box<IrNode>>,
    },
}

pub struct Ir {
    pub nodes: Vec<IrNode>,
}

impl Ir {
    pub fn new() -> Ir {
        Ir { nodes: vec![] }
    }

    pub fn generate(input: CodegenInput) -> Result<Ir, IrError> {
        let mut ir = Ir::new();

        let ast = parse_fallible(input.content, LuaVersion::lua52())
            .into_result()
            .map_err(IrError::InvalidSrc)?;

        for stmt in ast.nodes().stmts() {
            let nodes = Self::parse_stmt(stmt)?;

            for node in nodes {
                ir.nodes.push(node);
            }
        }

        Ok(ir)
    }
}

impl Ir {
    fn parse_stmt(stmt: &Stmt) -> Result<Vec<IrNode>, IrError> {
        let mut nodes = vec![];

        for token_reference in stmt.tokens() {
            nodes.push(Self::parse_token(token_reference)?)
        }

        Ok(nodes)
    }

    fn parse_token(token_reference: &TokenReference) -> Result<IrNode, IrError> {
        match token_reference.token_type() {
            TokenType::Number { text } => parse_number_literal(text),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum IrError {
    InvalidSrc(Vec<full_moon::Error>),
    InvalidNumberLiteral(std::num::ParseFloatError),
}
