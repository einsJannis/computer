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

struct MOV {
    arg0: Register,
    arg1: Value,
}
impl Instruction for MOV {}
impl WithArg0 for MOV {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl WithArg1 for MOV {
    type Output = Value;
    fn arg1(&self) -> &Self::Output { &self.arg1 }
}
impl With2Args for MOV {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        MOV { arg0, arg1 }
    }
}

struct LDW {
    arg0: Register,
    arg1: Address
}
impl Instruction for LDW {}
impl WithArg0 for LDW {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl WithArg1 for LDW {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.arg1 }
}
impl With2Args for LDW {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        LDW { arg0, arg1 }
    }
}

struct STW {
    arg0: Register,
    arg1: Address
}
impl Instruction for STW {}
impl WithArg0 for STW {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl WithArg1 for STW {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.arg1 }
}
impl With2Args for STW {
    fn new(arg0: <Self as WithArg0>::Output, arg1: <Self as WithArg1>::Output) -> Self {
        STW { arg0, arg1 }
    }
}

struct LDA {
    arg0: Address
}
impl Instruction for LDA {}
impl WithArg0 for LDA {
    type Output = Address;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl With1Args for LDA {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { LDA { arg0 } }
}

struct PSH {
    arg0: Value
}
impl Instruction for PSH {}
impl WithArg0 for PSH {
    type Output = Value;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl With1Args for PSH {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { PSH { arg0 } }
}

struct POP {
    arg0: Register
}
impl Instruction for POP {}
impl WithArg0 for PSH {
    type Output = Register;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl With1Args for POP {
    fn new(arg0: <Self as WithArg0>::Output) -> Self { POP { arg0 } }
}

struct JMP {
    arg0: Flag,
    arg1: Address
}
impl Instruction for JMP {}
impl WithArg0 for JMP {
    type Output = Flag;
    fn arg0(&self) -> &Self::Output { &self.arg0 }
}
impl WithArg1 for JMP {
    type Output = Address;
    fn arg1(&self) -> &Self::Output { &self.arg1 }
}

struct AssemblyProgram(Vec<dyn Instruction>);

fn main() {}
