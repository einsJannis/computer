use std::process::Output;
use rpc::lexer::TokenIterator;
use rpc::{ContentLocation, WithContentLocation};
use crate::parser::Parsable;
use crate::generator::Generable;

mod parser;
mod generator;
mod relative;
mod expandable;

struct Register(u8);

enum Value {
    Register(Register),
    Literal(i8),
}

enum Address {
    HL,
    Literal(u16)
}

struct Flag(u8);

trait Instruction: Parsable + Generable {}

trait WithArg0 {
    type Output: Parsable + Generable;
    fn arg0(&self) -> &Self::Output;
}

trait WithArg1 {
    type Output: Parsable + Generable;
    fn arg1(&self) -> &Self::Output;
}

trait With1Args: WithArg0 + Sized {
    fn new(arg0: <Self as WithArg0>::Output) -> Self;
}

trait With2Args: WithArg0 + WithArg1 + Sized {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self;
}

struct NOP;
impl Instruction for NOP {}

struct MOV(Register, Value);
impl Instruction for MOV {}
impl WithArg0 for MOV {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for MOV {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for MOV {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        MOV(arg0, arg1)
    }
}

struct LDW(Register, Address);
impl Instruction for LDW {}
impl WithArg0 for LDW {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for LDW {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for LDW {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        LDW(arg0, arg1)
    }
}

struct STW(Register, Address);
impl Instruction for STW {}
impl WithArg0 for STW {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for STW {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for STW {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        STW(arg0, arg1)
    }
}

struct LDA(Address);
impl Instruction for LDA {}
impl WithArg0 for LDA {
    type Output = Address;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl With1Args for LDA {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { LDA(arg0) }
}

struct PSH(Value);
impl Instruction for PSH {}
impl WithArg0 for PSH {
    type Output = Value;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl With1Args for PSH {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { PSH(arg0) }
}

struct POP(Register);
impl Instruction for POP {}
impl WithArg0 for POP {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl With1Args for POP {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { POP(arg0) }
}

struct JMP(Flag, Address);
impl Instruction for JMP {}
impl WithArg0 for JMP {
    type Output = Flag;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for JMP {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for JMP {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        JMP(arg0, arg1)
    }
}

struct ADD(Register, Value);
impl Instruction for ADD {}
impl WithArg0 for ADD {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for ADD {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for ADD {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        ADD(arg0, arg1)
    }
}

struct SUB(Register, Value);
impl Instruction for SUB {}
impl WithArg0 for SUB {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for SUB {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for SUB {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        SUB(arg0, arg1)
    }
}

struct AND(Register, Value);
impl Instruction for AND {}
impl WithArg0 for AND {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for AND {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for AND {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        AND(arg0, arg1)
    }
}

struct OR(Register, Value);
impl Instruction for OR {}
impl WithArg0 for OR {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for OR {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for OR {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        OR(arg0, arg1)
    }
}

struct INV(Register);
impl Instruction for INV {}
impl WithArg0 for INV {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl With1Args for INV {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { INV(arg0) }
}

struct CMP(Register, Value);
impl Instruction for CMP {}
impl WithArg0 for CMP {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for CMP {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for CMP {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        CMP(arg0, arg1)
    }
}

struct SHL(Register, Value);
impl Instruction for SHL {}
impl WithArg0 for SHL {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for SHL {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for SHL {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        SHL(arg0, arg1)
    }
}

struct SHR(Register, Value);
impl Instruction for SHR {}
impl WithArg0 for SHR {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.0 }
}
impl WithArg1 for SHR {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.1 }
}
impl With2Args for SHR {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        SHR(arg0, arg1)
    }
}

struct AssemblyProgram(Vec<dyn Instruction>);

fn main() {}
