use crate::generation::IntoBytes;
use crate::parsing::Parsable;
use crate::parsing::Token;

mod parsing;
mod generation;

fn main() {}

struct Register(u8);

enum Value {
    Register(Register),
    Literal(i8)
}

enum AddressValue {
    Literal(u16),
    Label(String)
}

enum Address {
    HL,
    Literal(AddressValue)
}

struct Flag(u8);

pub trait Instruction : Parsable + IntoBytes {
}

struct Label { name: String }
impl Instruction for Label {}
struct NOP;
impl Instruction for NOP {}
struct MOV(Register, Value);
impl Instruction for MOV {}
struct LDW(Register, Address);
impl Instruction for LDW {}
struct STW(Register, Address);
impl Instruction for STW {}
struct LDA(Address);
impl Instruction for LDA {}
struct PSH(Value);
impl Instruction for PSH {}
struct POP(Register);
impl Instruction for POP {}
struct JMP(Flag, Address);
impl Instruction for JMP {}
struct ADD(Register, Value);
impl Instruction for ADD {}
struct SUB(Register, Value);
impl Instruction for SUB {}
struct AND(Register, Value);
impl Instruction for AND {}
struct OR (Register, Value);
impl Instruction for OR {}
struct INV(Register);
impl Instruction for INV {}
struct CMP(Register, Value);
impl Instruction for CMP {}
struct SHL(Register, Value);
impl Instruction for SHL {}
struct SHR(Register, Value);
impl Instruction for SHR {}

struct AssemblyProgram(Vec<dyn Instruction>);
