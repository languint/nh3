use full_moon::{LuaVersion, ast::AstResult};

use crate::{
    equation::Equation,
    ir::{Ir, IrError},
};

pub struct CodegenInput<'src> {
    file_name: &'src str,
    content: &'src str,
}

pub type CodegenInputs<'src> = Vec<CodegenInput<'src>>;
pub type CodegenIRS = Vec<AstResult>;

pub struct Codegen<'src> {
    pub inputs: CodegenInputs<'src>,
}

impl<'src> Codegen<'src> {
    pub fn new(inputs: CodegenInputs<'src>) -> Codegen<'src> {
        Self { inputs }
    }
}

pub enum CodegenError {
    ParseError(full_moon::Error),
    IrError(IrError),
}

pub struct CodegenOutput<'src> {
    pub equations: Vec<Equation<'src>>,
    pub errors: Vec<CodegenError>,
}

impl<'src> Codegen<'src> {
    pub fn generate_ir(input: CodegenInput) -> Result<Ir, CodegenError> {
        let ir = Ir::generate(input).map_err(CodegenError::IrError)?;

        Ok(ir)
    }

    pub fn generate(&self) -> CodegenOutput<'src> {
        let equations = vec![];
        let errors = vec![];

        CodegenOutput { equations, errors }
    }
}
