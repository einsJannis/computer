use crate::parser::Parsable;
use crate::{Flag, Register, Instruction, Register, With2Args, WithArg1, WithArg0};

mod parser;

pub trait RelativeInstructionType {
    type Output: Instruction + Sized;
}

pub trait RelativeInstruction: Parsable + RelativeInstruction {
    fn is_label(label_name: LabelName) -> bool { false }
    fn unrelativice(&self, context: &RelativeProgram) -> Self::Output;
    fn size(&self) -> usize;
}

type LabelName = String;

struct LabeledInstruction<T: RelativeInstruction>(LabelName, T);

impl<T> RelativeInstructionType for LabeledInstruction<T> {
    type Output = T;
}

impl<T> RelativeInstruction for LabeledInstruction<T> {
    fn is_label(label_name: LabelName) -> bool {
        self.0 == label_name
    }
    fn unrelativice(&self, context: &RelativeProgram) -> Self::Output {
        self.1.unrelativice(context)
    }
    fn size(&self) -> usize { self.1.size() }
}

fn unrelativice(context: &RelativeProgram, label_name: LabelName) -> u16 {
    let index = context.0.iter().enumerate().find(|it| it.is_label(label_name))?.0;
    let mut size = 0;
    for i in 0..index {
        size += context.0[i].size();
    }
    return size as u16;
}

impl<T, A0> RelativeInstruction for T where T: 
    With2Args + 
    WithArg0<Output = A0> + 
    WithArg1<Output = LabelName> + 
    RelativeInstructionType<Output = With2Args + WithArg0<A0> + WithArg1<u16>>
{
    fn unrelativice(&self, context: &RelativeProgram) -> Self::Output {
        Self::Output::new(self.arg0(), unrelativice(context, self.arg1()))
    }
    fn size(&self) -> usize { 3 }
} 

impl<T> RelativeInstruction for T where T:
    WithArg1 +
    WithArg0<Output = LabelName> +
    RelativeInstructionType<Output = With2Args + WithArg0<u16>>
{
    fn unrelativice(&self, context: &RelativeProgram) -> Self::Output {
        Self::Output::new(self.arg0(), unrelativice(context, self.arg1()))
    }
    fn size(&self) -> usize { 3 }
}

pub struct LDW(Register, LabelName);

pub struct STW(Register, LabelName);

pub struct LDA(LabelName);

pub struct JMP(Flag, LabelName);

macro_rules! relative_instruction_type_impl {
    ($($relative:expr, $other:expr),*) => {
        $(impl RelativeInstructionType $relative {
            type Output = $other;
        })*
    };
}

relative_instruction_type_impl!(
    LDW, crate::LDW,
    STW, crate::STW,
    LDA, crate::LDA,
    JMP, crate::JMP
);

impl<T> RelativeInstructionType for T where T: Instruction {
    type Output = T;
}

impl<T> RelativeInstruction for T where T: Instruction {
    fn unrelativice(&self, context: &RelativeProgram) -> &dyn Instruction { self }
    fn size(&self) -> usize { <self as Generable>.size() }
}

pub struct RelativeProgram(Vec<dyn RelativeInstruction>);

