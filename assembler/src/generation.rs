use crate::{ADD, Address, AddressValue, AND, AssemblyProgram, CMP, Instruction, INV, JMP, Label, LDA, LDW, MOV, NOP, OR, Parsable, POP, PSH, Register, SHL, SHR, STW, SUB, Value};

trait OpCode {
    fn op_code() -> u8;
}

trait Flag {
    fn flag(&self) -> u8;
}

pub trait IntoBytes {
    type Context;
    fn as_bytes(&self, context: Self::Context) -> &[u8];
    fn byte_size(&self) -> usize;
}

impl IntoBytes for Register {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        &[self.0]
    }
    fn byte_size(&self) -> usize { 1 }
}

impl IntoBytes for Value {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        match self {
            Value::Register(register) => register.as_bytes(context),
            Value::Literal(value) => &[value as u8]
        }
    }
    fn byte_size(&self) -> usize { 1 }
}

impl IntoBytes for AddressValue {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        match self {
            AddressValue::Literal(value) => &[(value >> 8) as u8, (value & 0b11111111) as u8],
            AddressValue::Label(name) => {
                todo!()
            }
        }
    }
    fn byte_size(&self) -> usize { 2 }
}

impl IntoBytes for crate::Flag {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] { &[self.0] }
    fn byte_size(&self) -> usize { 1 }
}

impl IntoBytes for Label {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] { &[] }
    fn byte_size(&self) -> usize { 0 }
}

impl OpCode for NOP {
    fn op_code() -> u8 { 0 }
}

impl Flag for NOP {
    fn flag(&self) -> u8 { 0 }
}

impl IntoBytes for NOP {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] { &[Self::op_code() | self.flag()] }
    fn byte_size(&self) -> usize { 1 }
}

impl OpCode for MOV {
    fn op_code() -> u8 { 0x1 << 4 }
}

impl InstructionReg for MOV {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for MOV {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for LDW {
    fn op_code() -> u8 { 0x2 << 4 }
}

impl InstructionReg for LDW {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionAddr for LDW {
    fn addr(&self) -> &Address { &self.1 }
}

impl OpCode for STW {
    fn op_code() -> u8 { 0x3 << 4 }
}

impl InstructionReg for STW {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionAddr for STW {
    fn addr(&self) -> &Address { &self.1 }
}

impl OpCode for LDA {
    fn op_code() -> u8 { 0x4 << 4 }
}

impl InstructionAddr for LDA {
    fn addr(&self) -> &Address { &self.0 }
}

impl IntoBytes for LDA {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        match self.addr() {
            Address::HL => &[Self::op_code() | self.flag()],
            Address::Literal(address) => {
                let address = address.as_bytes(context);
                &[Self::op_code() | self.flag(), address[0], address[1]]
            }
        }
    }
    fn byte_size(&self) -> usize { match address { Address::HL => 1, _ => 3 } }
}

impl OpCode for PSH {
    fn op_code() -> u8 { 0x5 << 4 }
}

impl InstructionVal for PSH {
    fn val(&self) -> &Value { &self.0 }
}

impl IntoBytes for PSH {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        &[Self::op_code() | self.flag(), self.val().as_bytes(context)]
    }
    fn byte_size(&self) -> usize { 2 }
}

impl OpCode for POP {
    fn op_code() -> u8 { 0x6 << 4 }
}

impl InstructionReg for POP {
    fn reg(&self) -> &Register { &self.0 }
}

impl IntoBytes for POP {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        &[Self::op_code() | self.reg().as_bytes()]
    }
    fn byte_size(&self) -> usize { 1 }
}

impl OpCode for JMP {
    fn op_code() -> u8 { 0x7 << 4 }
}

impl JMP {
    fn flag_arg(&self) -> &crate::Flag { &self.0 }
}

impl InstructionAddr for JMP {
    fn addr(&self) -> &Address { &self.1 }
}

impl IntoBytes for JMP {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        match self.addr() {
            Address::HL => &[Self::op_code() | self.flag() | self.flag_arg()],
            Address::Literal(address) => {
                let address = address.as_bytes(context);
                &[Self::op_code() | self.flag() | self.flag_arg(), address[0], address[1]]
            }
        }
    }
    fn byte_size(&self) -> usize { match self.addr() { Address::HL => 0, _ => 1 } }
}

impl OpCode for ADD {
    fn op_code() -> u8 { 0x8 << 4 }
}

impl InstructionReg for ADD {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for ADD {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for SUB {
    fn op_code() -> u8 { 0x9 << 4 }
}

impl InstructionReg for SUB {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for SUB {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for AND {
    fn op_code() -> u8 { 0xA << 4 }
}

impl InstructionReg for AND {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for AND {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for OR {
    fn op_code() -> u8 { 0xB << 4 }
}

impl InstructionReg for OR {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for OR {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for INV {
    fn op_code() -> u8 { 0xC << 4 }
}

impl InstructionReg for INV {
    fn reg(&self) -> &Register { &self.0 }
}

impl IntoBytes for INV {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        &[Self::op_code() | self.flag(), self.val().as_bytes(context)]
    }
    fn byte_size(&self) -> usize { 2 }
}

impl OpCode for CMP {
    fn op_code() -> u8 { 0xD << 4 }
}

impl InstructionReg for CMP {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for CMP {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for SHL {
    fn op_code() -> u8 { 0xE << 4 }
}

impl InstructionReg for SHL {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for SHL {
    fn val(&self) -> &Value { &self.1 }
}

impl OpCode for SHR {
    fn op_code() -> u8 { 0xF << 4 }
}

impl InstructionReg for SHR {
    fn reg(&self) -> &Register { &self.0 }
}

impl InstructionVal for SHR {
    fn val(&self) -> &Value { &self.1 }
}

trait InstructionReg: OpCode + Instruction {
    fn reg(&self) -> &Register;
}

trait FirstByte: InstructionReg {
    fn first_byte(&self) -> u8;
}

impl<T> FirstByte for T where T: InstructionReg {
    fn first_byte(&self) -> u8 { Self::op_code() | self.flag() | self.reg().as_bytes()[0] }
}

trait InstructionAddr: OpCode + Instruction {
    fn addr(&self) -> &Address;
}

trait InstructionRegAddr: InstructionReg + InstructionAddr {}

impl<T> InstructionRegAddr for T where T: InstructionReg + InstructionAddr {}

trait InstructionVal: OpCode + Instruction {
    fn val(&self) -> &Value;
}

trait InstructionRegVal: InstructionReg + InstructionVal {}

impl<T> InstructionRegVal for T where T: InstructionReg + InstructionVal {}

impl<T> Flag for T where T: InstructionAddr {
    fn flag(&self) -> u8 { (match self.addr() { Address::HL => 0, _ => 1 }) << 3 }
}

impl<T> Flag for T where T: InstructionVal {
    fn flag(&self) -> u8 { (match self.val() { Value::Literal(_) => 0, _ => 1 }) << 3 }
}

impl<T> IntoBytes for T where T: InstructionRegAddr {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        match self.addr() {
            Address::HL =>
                &[self.first_byte()],
            Address::Literal(address) => {
                let address = address.as_bytes(context);
                &[self.first_byte(), address[0], address[1]]
            }
        }
    }
    fn byte_size(&self) -> usize { match self.addr() { Address::HL => 1, _ => 3 } }
}

impl<T> IntoBytes for T where T: InstructionRegVal {
    type Context = AssemblyProgram;
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        &[self.first_byte(), self.val().as_bytes(context)[0]]
    }
    fn byte_size(&self) -> usize { 2 }
}

impl IntoBytes for AssemblyProgram {
    type Context = ();
    fn as_bytes(&self, context: Self::Context) -> &[u8] {
        self.0.iter().map(|it| it.as_bytes(self)).collect().concat()
    }
    fn byte_size(&self) -> usize {
        self.0.iter().map(|it| it.byte_size()).sum()
    }
}
