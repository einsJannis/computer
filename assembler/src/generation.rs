use core::panicking::panic;
use crate::{Address, AddressValue, AssemblyProgram, Flag, Instruction, Register, Value};

pub trait IntoCode {
    fn code(self, program: &AssemblyProgram) -> u8;
}

pub trait IntoBytes {
    fn bytes(self, program: &AssemblyProgram) -> &[u8];
    fn size(&self) -> usize;
}

impl IntoCode for Register {
    fn code(self, _: &AssemblyProgram) -> u8 {
        match self {
            Register::NUMBERED(n) => n,
            Register::HIGH => 2,
            Register::LOW => 3,
            Register::PC_HIGH => 4,
            Register::PC_LOW => 5,
            Register::STACK_PTR => 6,
            Register::FLAG => 7
        }
    }
}

impl IntoCode for Value {
    fn code(self, _: &AssemblyProgram) -> u8 {
        match self {
            Value::Register(register) => register.code(),
            Value::Literal(value) => value as u8
        }
    }
}

impl IntoBytes for AddressValue {
    fn bytes(self, program: &AssemblyProgram) -> &[u8] {
        & match self {
            AddressValue::Literal(value) => [(value >> 8) as u8, value as u8],
            AddressValue::Label(name) => todo!()
        }
    }
    fn size(&self) -> usize { 2 }
}

impl IntoBytes for Address {
    fn bytes(self, program: &AssemblyProgram) -> &[u8] {
        & match self {
            Address::HL => [],
            Address::Value(value) => value.bytes()
        }
    }
    fn size(&self) -> usize {
        match self {
            Address::HL => 0,
            Address::Value(value) => value.size()
        }
    }
}

impl IntoCode for Flag {
    fn code(self, program: &AssemblyProgram) -> u8 {
        match self {
            Flag::NUMBERED(n) => n,
            Flag::HALT => 0,
            Flag::CARRY => 1,
            Flag::BORROW => 2,
            Flag::OVERFLOW => 3,
            Flag::LESS => 4,
            Flag::EQUAL => 5,
        }
    }
}

trait InstructionFlag {
    fn flag(&self) -> u8;
}

impl InstructionFlag for Instruction {
    fn flag(&self) -> u8 {
        (match self {
            Instruction::LABEL(_) => panic!(),
            Instruction::NOP => 0,
            Instruction::MOV(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::LDW(_, address) => match address {
                Address::HL => 0,
                Address::Value(_) => 1
            }
            Instruction::STW(_, address) => match address {
                Address::HL => 0,
                Address::Value(_) => 1
            }
            Instruction::LDA(address) => match address {
                Address::HL => 0,
                Address::Value(_) => 1
            }
            Instruction::PSH(value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::POP(_) => 0,
            Instruction::JMP(_, address) => match address {
                Address::HL => 0,
                Address::Value(_) => 1
            }
            Instruction::ADD(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::SUB(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::AND(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::OR(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::INV(_) => 0,
            Instruction::CMP(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::SHL(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
            Instruction::SHR(_, value) => match value {
                Value::Register(_) => 0,
                Value::Literal(_) => 1
            }
        }) << 3
    }
}

impl IntoCode for Instruction {
    fn code(&self, program: &AssemblyProgram) -> u8 {
        (match self {
            Instruction::LABEL(_) => panic!(),
            Instruction::NOP => 0x0,
            Instruction::MOV(_, _) => 0x1,
            Instruction::LDW(_, _) => 0x2,
            Instruction::STW(_, _) => 0x3,
            Instruction::LDA(_) => 0x4,
            Instruction::PSH(_) => 0x5,
            Instruction::POP(_) => 0x6,
            Instruction::JMP(_, _) => 0x7,
            Instruction::ADD(_, _) => 0x8,
            Instruction::SUB(_, _) => 0x9,
            Instruction::AND(_, _) => 0xA,
            Instruction::OR(_, _) => 0xB,
            Instruction::INV(_) => 0xC,
            Instruction::CMP(_, _) => 0xD,
            Instruction::SHL(_, _) => 0xE,
            Instruction::SHR(_, _) => 0xF,
        }) << 4
    }
}

impl IntoBytes for Instruction {
    fn bytes(self, program: &AssemblyProgram) -> &[u8] {
        & match self {
            Instruction::LABEL(_) => [],
            Instruction::NOP => [self.code(program)],
            Instruction::MOV(register, value) =>
                [self.code()|self.flag()|register.code(program),value.code(program)],
            Instruction::LDW(register, address) => match address {
                Address::HL => [self.code()|self.flag()|register.code(program)],
                Address::Value(address) => {
                    let address = address.bytes(program);
                    [self.code()|self.flag()|register.code(),address[0],address[1]]
                }
            },
            Instruction::STW(register, address) => match address {
                Address::HL => [self.code()|self.flag()|register.code(program)],
                Address::Value(address) => {
                    let address = address.bytes(program);
                    [self.code()|self.flag()|register.code(),address[0],address[1]]
                }
            },
            Instruction::LDA(address) => match address {
                Address::HL => [self.code()],
                Address::Value(address) => {
                    let address = address.bytes(program);
                    [self.code()|self.flag(),address[0],address[1]]
                }
            }
            Instruction::PSH(value) => [self.code()|self.flag(),value.code()],
            Instruction::POP(register) => [self.code()|self.flag()|register.code()],
            Instruction::JMP(flag, address) => match address {
                Address::HL => [self.code()|self.flag()|flag.code()],
                Address::Value(address) => {
                    let address = address.bytes();
                    [self.code()|self.flag()|flag.code(), address[0], address[1]]
                }
            }
            Instruction::ADD(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::SUB(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::AND(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::OR(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::INV(register) =>
                [self.code()|self.flag()|register.code()],
            Instruction::CMP(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::SHL(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
            Instruction::SHR(register, value) =>
                [self.code()|self.flag()|register.code(),value.code()],
        }
    }
    fn size(&self) -> usize {
        match self {
            Instruction::LABEL(_) => 0,
            Instruction::NOP => 1,
            Instruction::MOV(_, _) => 2,
            Instruction::LDW(_, address) => match address {
                Address::HL => 1,
                Address::Value(_) => 3
            },
            Instruction::STW(_, address) => match address {
                Address::HL => 1,
                Address::Value(_) => 3
            },
            Instruction::LDA(address) => match address {
                Address::HL => 1,
                Address::Value(_) => 3
            },
            Instruction::PSH(_) => 2,
            Instruction::POP(_) => 1,
            Instruction::JMP(_, address) => match address {
                Address::HL => 1,
                Address::Value(_) => 3
            }
            Instruction::ADD(_, _) => 2,
            Instruction::SUB(_, _) => 2,
            Instruction::AND(_, _) => 2,
            Instruction::OR(_, _) => 2,
            Instruction::INV(_) => 1,
            Instruction::CMP(_, _) => 2,
            Instruction::SHL(_, _) => 2,
            Instruction::SHR(_, _) => 2,
        }
    }
}

impl AssemblyProgram {
    pub fn bytes(self) -> &[u8] {
        let mut result: Vec<&[u8]> = vec![];
        for instruction in self.instructions {
            result += instruction.bytes(&self)
        }
        result.concat().as_slice()
    }
    pub fn size(&self) -> usize {
        let mut result = 0;
        for instruction in self.instructions {
            result += instruction.size()
        }
        result
    }
}
