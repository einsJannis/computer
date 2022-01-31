use crate::{parser::Parsable, expandable::{ExpandableProgram, MacroArgument}, With2Args};

pub enum MacroArgumentType {
    Instruction()
}

#[derive(Debug)]
struct ArgumentName(String);

pub trait MacroInstruction: Parsable {
    fn expand(&self, m_context: &Macro, p_context: &ExpandableProgram, arguments: Vec<MacroArgument>) -> Result<dyn ExpandableInstruction, ExpandError>;
}

impl MacroInstruction for ArgumentName {
    fn expand(&self, m_context: &Macro, p_context: &ExpandableProgram, arguments: Vec<MacroArgument>) -> Result<dyn ExpandableInstruction, ExpandError> {
        let index = m_context.argument_index(self);
        let argument = arguments[index];
        let excpected = MacroArgumentType::Instruction();
        let found = argument.macro_type();
        if found == excpected {
            return Ok(argument as ExpandableInstruction);
        }
        return Err(ExpandError::WrongArgumentType { excpected, found });
    }
}

impl<T> MacroInstruction for T where T: ExpandableInstruction {
    fn expand(&self, m_context: &Macro, p_context: &ExpandableProgram, arguments: Vec<MacroArgument>) -> Result<dyn ExpandableInstruction, ExpandError> {
        Ok(self)
    }
}

pub struct Macro {
    name: String,
    arguments: Vec<ArgumentName>,
    instructions: Vec<dyn MacroInstruction>
}

#[derive(Debug)]
pub enum ExpandError {
    UnknownArgument(ArgumentName),
    InsufficientArgumentsSupplied(),
    WrongArgumentType { excpected: MacroArgumentType, found: MacroArgumentType }
}

impl Macro {
    fn expand(&self, context: &ExpandableProgram, arguments: Vec<MacroArgument>) -> Result<dyn RelativeInstruction, ExpandError> {
        self.instructions.iter().map(|it| it.expand(self, context, arguments)?);
        todo!("handle recursive macro expantion")
    }
    fn argument_index(&self, name: ArgumentName) -> Result<usize, ExpandError> {
        let index = self.arguments.iter().enumerate()
            .find(|it| it.0 == name.0).ok_or(|| Err(ExpandError::UnknownArgument(name)))?.0;
        return Ok(index);
    }
}

