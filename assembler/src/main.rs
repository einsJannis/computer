use std::env::args;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::process::exit;
use crate::parsing::Parseable;
use crate::generation::IntoBytes;

mod parsing;
mod generation;

fn main() {
    if args().len() != 3 {
        println!("Usage: {} <asm_file> <target_file>", args()[0])
    }
    let mut i = 0;
    let content = read_to_string(Path::new(args()[1].to_string()))?.as_str();
    let program = AssemblyProgram::parse(content, i).bytes();
    write(Path::new(args()[1].to_string()), program)?;
    exit(0);
}

#[derive(Clone, Debug)]
enum Register {
    NUMBERED(u8),
    HIGH,
    LOW,
    PC_HIGH,
    PC_LOW,
    STACK_PTR,
    FLAG
}

#[derive(Clone)]
enum Value {
    Register(Register),
    Literal(i8),
}

#[derive(Clone)]
enum AddressValue {
    Literal(u16),
    Label(String),
}

#[derive(Clone)]
enum Address {
    HL,
    Value(AddressValue)
}

#[derive(Clone)]
enum Flag {
    NUMBERED(u8),
    HALT,
    CARRY,
    BORROW,
    OVERFLOW,
    LESS,
    EQUAL
}

#[derive(Clone)]
enum Instruction {
    LABEL(String),
    NOP,
    MOV(Register, Value),
    LDW(Register, Address),
    STW(Register, Address),
    LDA(Address),
    PSH(Value),
    POP(Register),
    JMP(Flag, Address),
    ADD(Register, Value),
    SUB(Register, Value),
    AND(Register, Value),
    OR(Register, Value),
    INV(Register),
    CMP(Register, Value),
    SHL(Register, Value),
    SHR(Register, Value),
}

struct AssemblyProgram {
    instructions: Vec<Instruction>
}
