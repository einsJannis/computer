use crate::parser::Parsable;
use crate::relative::RelativeInstruction;

pub trait ExpandableInstruction: Parsable {
    fn expand(&self, context: ExpandableProgram) -> &dyn RelativeInstruction;
}

impl<T> ExpandableInstruction for T where T: RelativeInstruction {
    fn expand(&self, context: ExpandableProgram) -> &dyn RelativeInstruction { self }
}

struct ExpandableProgram(Vec<dyn ExpandableInstruction>);

