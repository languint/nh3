use full_moon::ShortString;

use crate::ir::{IrError, IrNode};

pub fn parse_number_literal(text: &ShortString) -> Result<IrNode, IrError> {
    let number = text.parse::<f64>().map_err(IrError::InvalidNumberLiteral)?;

    Ok(IrNode::NumberLiteral(number))
}
