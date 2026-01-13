use crate::codegen::CodegenInput;

pub enum IrNode<'src> {
    Variable {
        name: &'src str,
        initial_value: Option<Box<IrNode<'src>>>,
    },
}

pub struct Ir<'src> {
    pub nodes: Vec<IrNode<'src>>,
}

impl<'src> Ir<'src> {
    pub fn new() -> Ir<'src> {
        Ir { nodes: vec![] }
    }

    pub fn generate(input: CodegenInput) -> Result<Ir<'src>, IrError> {
        let ir = Ir::new();

        
        
        Ok(ir)
    }
}

pub enum IrError {}
