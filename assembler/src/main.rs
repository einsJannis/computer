use std::env::args;
use std::fs::{read_to_string, write};
use std::path::Path;
use std::process::exit;
use crate::parsing::Parseable;
use crate::generation::IntoBytes;

mod parsing;
mod generation;
mod macros;

fn main() {
    if args().len() != 3 {
        println!("Usage: {} <asm_file> <target_file>", args()[0])
    }
    compile_file(Path::new(args()[1]), Path::new(args()[2]));
    exit(0);
}

fn compile_file(input: &Path, target: &Path) {
    let content = read_to_string(input)?.as_str();
    let program = compile(content);
    write(target, program);
}

fn compile(content: &str) -> &[u8] {
    let mut i = 0;
    AssemblyProgram::parse(content, i).bytes()
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
    Label(Label),
    NOP(NOP),
    MOV(MOV),
    LDW(LDW),
    STW(STW),
    LDA(LDA),
    PSH(PSH),
    POP(POP),
    JMP(JMP),
    ADD(ADD),
    SUB(SUB),
    AND(AND),
    OR (OR ),
    INV(INV),
    CMP(CMP),
    SHL(SHL),
    SHR(SHR),
}

struct Label { name: String }

struct NOP;

struct MOV(Register, Value);

struct LDW(Register, Value);

struct STW(Register, Value);

struct LDA(Address);

struct PSH(Value);

struct POP(Register);

struct JMP(Flag, Address);

struct ADD(Register, Value);

struct SUB(Register, Value);

struct AND(Register, Value);

struct OR(Register, Value);

struct INV(Register);

struct CMP(Register, Value);

struct SHL(Register, Value);

struct SHR(Register, Value);

struct AssemblyProgram {
    instructions: Vec<Instruction>
}
