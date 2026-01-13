#[cfg(test)]
mod tests {
    use nh3_codegen::{
        codegen::CodegenInput,
        ir::{Ir, IrNode},
    };

    #[test]
    fn number_literal() {
        let src = CodegenInput {
            content: "local x = 100",
            file_name: "test",
        };

        let ir = Ir::generate(src).expect("IR should be valid!");

        assert!(ir.nodes.contains(&IrNode::NumberLiteral(100.0)));
    }
}
