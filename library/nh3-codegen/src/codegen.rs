use crate::equation::Equation;

pub struct Codegen<'src> {
    pub src: &'src str,
}

impl<'src> Codegen<'src> {
    pub fn new(src: &'src str) -> Codegen<'src> {
        Self { src }
    }
}

pub struct CodegenOutput<'src> {
    pub equations: Vec<Equation<'src>>,
    pub errors: Vec<Box<dyn std::error::Error>>,
}

impl<'src> Codegen<'src> {
    pub fn generate(&self) -> CodegenOutput<'src> {
        let equations = vec![];
        let errors = vec![];

        CodegenOutput { equations, errors }
    }
}
