use crate::parser::Parsable;
use crate::relative::RelativeInstruction;
use crate::macros::Macro;

pub trait ExpandableInstruction: Parsable {
    fn expand(&self, context: ExpandableProgram) -> Vec<dyn RelativeInstruction>;
}

pub trait MacroArgument {
    fn macro_type(&self) -> MacroArgumentType;
}

pub struct MacroCall {
    name: String,
    arguments: Vec<dyn MacroArgument>
}

impl ExpandableInstruction for MacroCall {
    fn expand(&self, context: ExpandableProgram) -> Vec<dyn RelativeInstruction> {
        let _macro = context.macros.iter().find(|it| it.name == self.name).ok_or(|| todo!("error"))?;
        return _macro.expand(context, self.arguments).iter().map(|it| it.expand(context)); //add expanded to context
    }
}

impl<T> ExpandableInstruction for T where T: RelativeInstruction {
    fn expand(&self, context: ExpandableProgram) -> Vec<dyn RelativeInstruction> { vec![self] }
}

pub struct ExpandableProgram {
    macros: Vec<Macro>,
    instruction: Vec<dyn ExpandableInstruction>
}

