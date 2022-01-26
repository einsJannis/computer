use crate::parser::Parsable;
use crate::{Instruction, RelativeProgram};

pub trait RelativeInstruction: Parsable {
    fn unrelativice(&self, context: RelativeProgram) -> &dyn Instruction;
}

impl<T> RelativeInstruction for T where T: Instruction {
    fn unrelativice(&self, context: RelativeProgram) -> &dyn Instruction { self }
}

struct RelativeProgram(Vec<dyn RelativeInstruction>);
