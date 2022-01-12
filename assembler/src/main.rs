use core::panicking::panic;
use std::borrow::Borrow;
use std::ops::{Add, Index, Shl};
use std::rc::Rc;
use regex::Regex;
use crate::MacroValue::Value;

fn main() {
}

#[derive(Clone)]
enum Register {
    NUMBERED(u8),
    HIGH,
    LOW,
    PC_HIGH,
    PC_LOW,
    STACK_PTR,
    FLAG
}

impl Register {
    fn parse(content: &str, &mut i: usize) -> Register {
        let result = if Regex::new("reg[0-7]]").unwrap().is_match_at(content, i) {
            i += 4;
            Register::NUMBERED(content.index(i - 1).into())
        } else if content.matches_at(i, "high") {
            i += 4;
            Register::HIGH
        } else if content.matches_at(i, "low") {
            i += 3;
            Register::LOW
        } else if content.matches_at(i, "pc_high") {
            i += 7;
            Register::PC_HIGH
        } else if content.matches_at(i, "pc_low") {
            i += 6;
            Register::PC_LOW
        } else if content.matches_at(i, "stack_ptr") {
            i += 9;
            Register::STACK_PTR
        } else if content.matches_at(i, "flag") {
            i += 4;
            Register::FLAG
        } else { panic!() };
        if !(content.matches_at(i, " ") || content.matches_at(i, "\n")) {
            panic!()
        }
        i += 1;
        result
    }
    fn code(&self) -> u8 {
        match self {
            Register::NUMBERED(n) => n.clone(),
            Register::HIGH => 2,
            Register::LOW => 3,
            Register::PC_HIGH => 4,
            Register::PC_LOW => 5,
            Register::STACK_PTR => 6,
            Register::FLAG => 7,
        }
    }
}

#[derive(Clone)]
enum Value {
    Register(Register),
    Literal(i8),
}

impl Value {
    fn parse(content: &str, &mut i: usize) -> Value {
        todo!()
    }
    fn code(&self) -> u8 {
        match self {
            Value::Register(reg) => reg.code(),
            Value::Literal(value) => value.clone()
        }
    }
}

#[derive(Clone)]
enum AddressValue {
    Literal(u16),
    Label(String),
}

impl AddressValue {
    fn parse(content: &str, &mut i: usize) -> AddressValue { todo!() }
    fn value(&self, program: &AssemblyProgram) -> u16 {
        match self {
            AddressValue::Literal(v) => v.clone(),
            AddressValue::Label(name) => {
                let mut counter = 0;
                for instruction in program.instructions {
                    if let Instruction::LABEL(lname) = instruction {
                        if name == lname { break }
                    }
                    counter += instruction.size()
                }
                counter
            }
        }
    }
    fn high(&self, program: &AssemblyProgram) -> u8 {
        (self.value(program) >> 8) as u8
    }
    fn low(&self, program: &AssemblyProgram) -> u8 {
        (self.value(program) & 0b11111111) as u8
    }
}

#[derive(Clone)]
enum Address {
    HL,
    Value(AddressValue)
}

impl Address {
    fn parse(content: &str, &mut i: usize) -> Address {
        if content.matches_at(i, "\n") { Address::HL } else {
            Address::Value(AddressValue::parse(content, i))
        }
    }
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

impl Flag {
    fn parse(content: &str, &mut i: usize) -> Flag {
        let result = if Regex::new("flag[0-7]")?.is_match_at(content, i) {
            i += 5;
            Flag::NUMBERED(content.index(i - 1).into())
        } else if content.matches_at(i, "halt") {
            i += 4;
            Flag::HALT
        } else if content.matches_at(i, "carry") {
            i += 5;
            Flag::CARRY
        } else if content.matches_at(i, "borrow") {
            i += 6;
            Flag::BORROW
        } else if content.matches_at(i, "overflow") {
            i += 8;
            Flag::OVERFLOW
        } else if content.matches_at(i, "less") {
            i += 4;
            Flag::LESS
        } else if content.matches_at(i, "equal") {
            i += 5;
            Flag::EQUAL
        } else { panic!() };
        i += 1;
        result
    }
    fn code(&self) -> u8 {
        match self {
            Flag::NUMBERED(n) => n.clone(),
            Flag::HALT => 0,
            Flag::CARRY => 1,
            Flag::BORROW => 2,
            Flag::OVERFLOW => 3,
            Flag::LESS => 4,
            Flag::EQUAL => 5,
        }
    }
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
    INV(Register),
    CMP(Register, Value),
    SHL(Register, Value),
    SHR(Register, Value),
}

impl Instruction {
    fn code(&self) -> u8 {
        match self {
            Instruction::LABEL(_)  => panic!(),
            Instruction::NOP       => 0x0,
            Instruction::MOV(_, _) => 0x1,
            Instruction::LDW(_, _) => 0x2,
            Instruction::STW(_, _) => 0x3,
            Instruction::LDA(_)    => 0x4,
            Instruction::PSH(_)    => 0x5,
            Instruction::POP(_)    => 0x6,
            Instruction::JMP(_, _) => 0x7,
            Instruction::ADD(_, _) => 0x8,
            Instruction::SUB(_, _) => 0x9,
            Instruction::AND(_, _) => 0xA,
            Instruction::INV(_)    => 0xB,
            Instruction::CMP(_, _) => 0xC,
            Instruction::SHL(_, _) => 0xD,
            Instruction::SHR(_, _) => 0xF,
        }.shl(4)
    }
    fn flag(&self) -> u8 {
        fn v_flag(v: &Value) -> u8 {
            match v {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
        }
        fn a_flag(a: &Address) -> u8 {
            match a {
                Address::HL => 0,
                Address::Value(_) => 1,
            }
        }
        match self {
            Instruction::MOV(_, v) => v_flag(v),
            Instruction::LDW(_, a) => a_flag(a),
            Instruction::STW(_, a) => a_flag(a),
            Instruction::LDA(a) => a_flag(a),
            Instruction::PSH(v) => v_flag(v),
            Instruction::JMP(_, a) => a_flag(a),
            Instruction::ADD(_, v) => v_flag(v),
            Instruction::SUB(_, v) => v_flag(v),
            Instruction::AND(_, v) => v_flag(v),
            Instruction::CMP(_, v) => v_flag(v),
            Instruction::SHL(_, v) => v_flag(v),
            Instruction::SHR(_, v) => v_flag(v),
            _ => 0,
        }.shl(3)
    }
    pub fn bytes(&self, program: &AssemblyProgram) -> &[u8] {
        &match self {
            Instruction::LABEL(_) => [],
            Instruction::NOP => [0],
            Instruction::MOV(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            },
            Instruction::LDW(reg, address) => match address {
                Address::HL => [self.code() | self.flag() | reg.code()],
                Address::Value(address) => {
                    [self.code() | self.flag() | reg.code(), address.high(program), address.low(program)]
                }
            }
            Instruction::STW(reg, address) => match address {
                Address::HL => [self.code() | self.flag() | reg.code()],
                Address::Value(address) => {
                    [self.code() | self.flag() | reg.code(), address.high(program), address.low(program)]
                }
            }
            Instruction::LDA(address) => match address {
                Address::HL => [self.code() | 0b0000],
                Address::Value(address) => {
                    [self.code() | self.flag() | 0b000, address.high(program), address.low(program)]
                }
            }
            Instruction::PSH(value) => [self.code() | self.flag() | 0b000, value.code()],
            Instruction::POP(reg) => [self.code() | self.flag() | reg.code()],
            Instruction::JMP(flag, address) => match address {
                Address::HL => [self.code() | self.flag() | flag.code()],
                Address::Value(address) => {
                    [self.code() | self.flag() | flag.code(), address.high(program), address.low(program)]
                }
            }
            Instruction::ADD(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
            Instruction::SUB(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
            Instruction::AND(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
            Instruction::INV(reg) => [self.code() | self.flag() | reg.code()],
            Instruction::CMP(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
            Instruction::SHL(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
            Instruction::SHR(reg, value) => {
                [self.code() | self.flag() | reg.code(), value.code()]
            }
        }
    }
    const fn size(&self) -> usize {
        match self {
            Instruction::LABEL(_) => 0,
            Instruction::NOP => 1,
            Instruction::MOV(_, _) => 2,
            Instruction::LDW(_, v) => match v {
                Address::HL => 1,
                Address::Literal(_) => 3,
                Address::Label(_) => 3
            }
            Instruction::STW(_, v) => match v {
                Address::HL => 1,
                Address::Literal(_) => 3,
                Address::Label(_) => 3
            }
            Instruction::LDA(v) => match v {
                Address::HL => 1,
                Address::Literal(_) => 3,
                Address::Label(_) => 3,
            }
            Instruction::PSH(_) => 2,
            Instruction::POP(_) => 1,
            Instruction::JMP(_, v) => match v {
                Address::HL => 1,
                Address::Literal(_) => 3,
                Address::Label(_) => 3
            }
            Instruction::ADD(_, _) => 2,
            Instruction::SUB(_, _) => 2,
            Instruction::AND(_, _) => 2,
            Instruction::INV(_) => 1,
            Instruction::CMP(_, _) => 2,
            Instruction::SHL(_, _) => 2,
            Instruction::SHR(_, _) => 2,
        }
    }
}

struct AssemblyProgram { instructions: Vec<Instruction> }

impl AssemblyProgram {
    fn bytes(&self) -> &[u8] {
        let mut result: Vec<&[u8]> = vec![];
        for block in self.instructions {
            result += block.bytes(self);
        }
        result.concat().as_slice()
    }
}

enum MacroRegister {
    Register(Register),
    MArgument(String)
}

impl MacroRegister {
    fn complete(&self, arguments: &Vec<MacroArgument>, macro_d: &Macro) -> Register {
        match self {
            MacroRegister::Register(register) => register.clone(),
            MacroRegister::MArgument(name) => {
                match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                    MacroArgument::MacroRegister(register) =>
                        register.value.clone(),
                    _ => panic!()
                }
            }
        }
    }
}

enum MacroValue {
    Value(Value),
    MArgument(String)
}

impl MacroValue {
    fn complete(&self, arguments: &Vec<MacroArgument>, macro_d: &Macro) -> Value {
        match self {
            MacroValue::Value(value) => value.clone(),
            MacroValue::MArgument(name) => {
                match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                    MacroArgument::MacroValue(value) =>
                        value.value.clone(),
                    _ => panic!()
                }
            }
        }
    }
}

enum MacroAddress {
    Address(Address),
    MacroArgument(String)
}

impl MacroAddress {
    fn complete(&self, arguments: &Vec<MacroArgument>, macro_d: &Macro) -> Address {
        match self {
            MacroAddress::Address(address) => address.clone(),
            MacroAddress::MacroArgument(name) => {
                match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                    MacroArgument::MacroAddress(address) =>
                        address.value.clone(),
                    _ => { panic!() }
                }
            }
        }
    }
}

enum MacroFlag {
    Flag(Flag),
    MacroArgument(String)
}

impl MacroFlag {
    fn complete(&self, arguments: &Vec<MacroArgument>, macro_d: &Macro) -> Flag {
        match self {
            MacroFlag::Flag(flag) => flag.clone(),
            MacroFlag::MacroArgument(name) => {
                match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                    MacroArgument::MacroFlag(flag) =>
                        flag.value.clone(),
                    _ => { panic!() }
                }
            }
        }
    }
}

enum MacroIdentifier {
    Identifier(String),
    MacroArgument(String)
}

struct MacroArgumentValue<T> {
    value: T
}

enum MacroArgument {
    MacroRegister(MacroArgumentValue<Register>),
    MacroValue(MacroArgumentValue<Value>),
    MacroAddress(MacroArgumentValue<Address>),
    MacroFlag(MacroArgumentValue<Flag>),
    MacroInstruction(MacroArgumentValue<Instruction>),
    MacroIdentifier(MacroArgumentValue<String>),
}

impl MacroArgument {
    fn from_name(name: String, arguments: &Vec<MacroArgument>, macro_d: &Macro) -> &MacroArgument {
        arguments[macro_d.macro_argument_definitions.iter().find(|it| it.name == name)]
    }
}

enum MacroInstruction {
    MacroArgument(String),
    MacroCall(String, Vec<MacroArgument>),
    LABEL(MacroIdentifier),
    NOP,
    MOV(MacroRegister, MacroValue),
    LDW(MacroRegister, MacroAddress),
    STW(MacroRegister, MacroAddress),
    LDA(MacroAddress),
    PSH(MacroValue),
    POP(MacroRegister),
    JMP(MacroFlag, MacroAddress),
    ADD(MacroRegister, MacroValue),
    SUB(MacroRegister, MacroValue),
    AND(MacroRegister, MacroValue),
    OR(MacroRegister, MacroValue),
    INV(MacroRegister),
    CMP(MacroRegister, MacroValue),
    SHL(MacroRegister, MacroValue),
    SHR(MacroRegister, MacroValue),
}

impl MacroInstruction {
    fn complete(&self, arguments: &Vec<MacroArgument>, macro_d: &Macro, program: &PartialAssemblyProgram) -> Vec<Instruction> {
        match self {
            MacroInstruction::MacroArgument(name) => {
                let i = macro_d.macro_argument_definitions.iter().enumerate()
                    .find(|(_, it)| it.name == name)?.0;
                match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                    MacroArgument::MacroInstruction(instruction) => {
                        vec![instruction.value]
                    }
                    _ => { panic!(); }
                }
            }
            MacroInstruction::MacroCall(macro_name, args) => {
                let m = program.macros.iter().find(|it| it.name == macro_name)?;
                m.expand(args, program)
            }
            MacroInstruction::LABEL(label) => {
                match label {
                    MacroIdentifier::Identifier(name) => vec![Instruction::LABEL(name.clone())],
                    MacroIdentifier::MacroArgument(name) => {
                        match MacroArgument::from_name(name.clone(), arguments, macro_d) {
                            MacroArgument::MacroIdentifier(identifier) =>
                                vec![Instruction::LABEL(identifier.value.clone())],
                            _ => panic!()
                        }
                    }
                }
            },
            MacroInstruction::NOP => vec![Instruction::NOP],
            MacroInstruction::MOV(register, value) =>
                vec![Instruction::MOV(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::LDW(register, address) =>
                vec![Instruction::LDW(
                    register.complete(arguments, macro_d),
                    address.complete(arguments, macro_d)
                )],
            MacroInstruction::STW(register, address) =>
                vec![Instruction::STW(
                    register.complete(arguments, macro_d),
                    address.complete(arguments, macro_d)
                )],
            MacroInstruction::LDA(address) =>
                vec![Instruction::LDA(address.complete(arguments, macro_d))],
            MacroInstruction::PSH(value) =>
                vec![Instruction::PSH(value.complete(arguments, macro_d))],
            MacroInstruction::POP(register) =>
                vec![Instruction::POP(register.complete(arguments, macro_d))],
            MacroInstruction::JMP(flag, address) =>
                vec![Instruction::JMP(
                    flag.complete(arguments, macro_d),
                    address.complete(arguments, macro_d)
                )],
            MacroInstruction::ADD(register, value) =>
                vec![Instruction::ADD(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::SUB(register, value) =>
                vec![Instruction::SUB(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::AND(register, value) =>
                vec![Instruction::AND(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::OR(register, value) =>
                vec![Instruction::OR(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::INV(register) =>
                vec![Instruction::INV(
                    register.complete(arguments, macro_d),
                )],
            MacroInstruction::CMP(register, value) =>
                vec![Instruction::CMP(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::SHL(register, value) =>
                vec![Instruction::SHL(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
            MacroInstruction::SHR(register, value) =>
                vec![Instruction::SHR(
                    register.complete(arguments, macro_d),
                    value.complete(arguments, macro_d)
                )],
        }
    }
}

struct MacroArgumentDefinitionContainer<T> {
    name: String,
}

enum MacroArgumentDefinition {
    MacroRegister(Rc<MacroArgumentDefinitionContainer<Register>>),
    MacroValue(Rc<MacroArgumentDefinitionContainer<Value>>),
    MacroAddress(Rc<MacroArgumentDefinitionContainer<Address>>),
    MacroFlag(Rc<MacroArgumentDefinitionContainer<Flag>>),
    MacroInstruction(Rc<MacroArgumentDefinitionContainer<Instruction>>),
    MacroIdentifier(Rc<MacroArgumentDefinitionContainer<String>>),
}

struct Macro {
    name: String,
    macro_argument_definitions: Vec<MacroArgumentDefinition>,
    macro_instructions: Vec<MacroInstruction>
}

impl Macro {
    fn expand(&self, arguments: &Vec<MacroArgument>, program: &PartialAssemblyProgram) -> Vec<Instruction> {
        self.macro_instructions.iter().map(|it| it.complete(arguments, self, program)).collect()
    }
}

enum InstructionContainer {
    Literal(Instruction),
    MacroCall(String, Vec<MacroArgument>)
}

impl InstructionContainer {
    fn complete(&self, program: &PartialAssemblyProgram) -> Vec<Instruction> {
        match self {
            InstructionContainer::Literal(it) => vec![it.clone()],
            InstructionContainer::MacroCall(name, args) => {
                program.macros.iter().find(|it| it.name == name).expand(args, program)
            }
        }
    }
}

struct PartialAssemblyProgram {
    macros: Vec<Macro>,
    instruction_blocks: Vec<InstructionContainer>
}

trait MatchesAt {
    fn matches_at(&self, offset: usize, with: &Self) {}
}
impl MatchesAt for str {
    fn matches_at(&self, offset: usize, with: &Self) -> bool {
        let selfb = self.as_bytes();
        let withb = self.as_bytes();
        for i in 0..withb.len() {
            if selfb[offset + i] != withb[i] {
                return false
            }
        }
        return true
    }
}

impl PartialAssemblyProgram {
    fn parse(content: &str) -> PartialAssemblyProgram {
        let mut i = 0;
        let mut instructions: Vec<InstructionContainer> = Vec::new();
        loop {
            if content.matches_at(i, "NOP") {
                i += 4;
                instructions += InstructionContainer::Literal(Instruction::NOP)
            } else if content.matches_at(i, "MOV ") {
                i += 4;
                let register = Register::parse(content, i);
                let value = Value::parse(content, i);
                instructions += InstructionContainer::Literal(Instruction::MOV(register, value))
            } else if content.matches_at(i, "LDW ") {
                i += 4;
                let register = Register::parse(content, i);
                let address = Address::parse(content, i);
                instructions += InstructionContainer::Literal(Instruction::LDW(register, address))
            } else if content.matches_at(i, "STW ") {
                i += 4;
                let register = Register::parse(content, i);
                let address = Address::parse(content, i);
                instructions += InstructionContainer::Literal(Instruction::LDW(register, address))
            } else if content.matches_at(i, "LDA ") {
                i += 4;
                let address = if content.matches_at(i, "\n") { Address::HL } else {
                    Address::Value(AddressValue::parse(content, i))
                };
                instructions += InstructionContainer::Literal(Instruction::LDA(address))
            } else if content.matches_at(i, "PSH ") {
                i += 4;
                let value = Value::parse(content, i);
                instructions += InstructionContainer::Literal(Instruction::PSH(value))
            } else if content.matches_at(i, "POP ") {
                i += 4;
                let register = Register::parse(content, i);
                instructions += InstructionContainer::Literal(Instruction::POP(value))
            } else if content.matches_at(i, "JMP ") {
                i += 4;
                let flag = Flag::parse(content, i);
                let address =
                instructions += InstructionContainer::Literal(Instruction::JMP())
            }
            if content.matches_at(i, "\n") { i += 1; } else { panic!() }
        }
    }
    fn complete(self) -> AssemblyProgram {
        AssemblyProgram { instructions: self.instructions.iter().map(|it| it.complete(self)).collect() }
    }
}
