use crate::{NOP, With1Args, WithArg0, WithArg1, Address, Value};

pub trait Generable {
    fn generate(&self) -> &[u8];
    fn size(&self) -> usize;
}

impl Generable for NOP {
    fn generate(&self) -> &[u8] {
        return &[0]
    }
}

pub trait WithCode {
    fn code(&self) -> u8;
}

pub trait WithFlag {
    fn flag(&self) -> u8;
}

impl<T> Generable for NOP {
    fn generate(&self) -> &[u8] {
        &[0]
    }
    fn size(&self) -> usize { 1 }
}

impl<T> Generable for T where T: WithArg0<Output = Register> + WithArg1<Output = Value> + WithCode + WithFlag {
    fn generate(&self) -> &[u8] {
        &[self.code()|self.flag()|self.arg0().generate()[0],self.arg1().generate()[0]]
    }
    fn size(&self) -> usize { 2 }
}

impl<T> Generable for T where T: WithArg0<Output = Register> + WithArg1<Output = Address> + WithCode + WithFlag {
    fn generate(&self) -> &[u8] {
        match self.arg1() {
            Address::HL => &[self.code()|self.flag()|self.arg0().generate()[0]],
            Address::Literal(address) => {
                let high = (address >> 8) as u8;
                let low = (address & 0b11111111) as u8;
                &[self.code()|self.flag()|self.arg0().generate()[0],high,low]
            }
        }
    }
    fn size(&self) -> usize {
        match self.arg1() {
            Address::HL => 1,
            Address::Literal(_) => 3,
        }
    }
}

impl<T> Generable for T where T: With1Args + WithArg0<Output = Register> + WithCode {
    fn generate(&self) -> &[u8] {
        &[self.code()|self.arg0().generate()]
    }
    fn size(&self) -> usize { 1 }
}

impl<T> Generable for T where T: With1Args + WithArg0<Output = Address> + WithCode + WithFlag {
    fn generate(&self) -> &[u8] {
        match self.arg0() {
            Address::HL => &[self.code()|self.flag()|self.arg0().generate()[0]],
            Address::Literal(address) => {
                let high = (address >> 8) as u8;
                let low = (address & 0b11111111) as u8;
                &[self.code()|self.flag()|self.arg0().generate()[0],high,low]
            }
        }
    }
    fn size(&self) -> usize {
        match self.arg0() {
            Address::HL => 1,
            Address::Literal(_) => 3,
        }
    }
}

impl<T> Generable for T where T: WithArg0<Output = Flag> + WithArg1<Output = Address> + WithCode + WithFlag {
    fn generate(&self) -> &[u8] {
        match self.arg1() {
            Address::HL => &[self.code()|self.flag()|self.arg0().generate()[0]],
            Address::Literal(address) => {
                let high = (address >> 8) as u8;
                let low = (address & 0b11111111) as u8;
                &[self.code()|self.flag()|self.arg0().generate()[0],high,low]
            }
        }
    }
    fn size(&self) -> usize {
        match self.arg1() {
            Address::HL => 1,
            Address::Literal(_) => 3,
        }
    }
}

impl<T> WithFlag for T where T: WithArg1<Output = Value> {
    fn flag(&self) -> u8 {
        (match self.arg1() {
            Value::Register(_) => 0,
            Value::Literal(_) => 1,
        }) << 3
    }
}

impl<T> WithFlag for T where T: WithArg1<Output = Address> {
    fn flag(&self) -> u8 {
        (match self.arg1() {
            Address::HL => 0,
            Address::Literal(_) => 1,
        }) << 3
    }
}

impl<T> WithFlag for T where T: WithArg0<Output = Value> {
    fn flag(&self) -> u8 {
        (match self.arg0() {
           Value::Register(_) => 0,
           Value::Literal(_) => 1,
        }) << 3
    }
}

impl<T> WithFlag for T where T: WithArg0<Output = Address> {
    fn flag(&self) -> u8 {
        (match self.arg0() {
            Address::HL => 0,
            Address::Literal(_) => 1,
        }) << 3
    }
}

macro_rules! code_impls {
    ($($struct:expr,$code:expr),*) => {
        $(
            impl WithCode for $struct {
                fn code(&self) -> u8 { $code << 4 }
            }
        )*
    };
}

code_impls!(
    MOV, 0x1,
    LDW, 0x2,
    STW, 0x3,
    LDA, 0x4,
    PSH, 0x5,
    POP, 0x6,
    JMP, 0x7,
    ADD, 0x8,
    SUB, 0x9,
    AND, 0xA,
    OR , 0xB,
    INV, 0xC,
    CMP, 0xD,
    SHL, 0xE,
    SHR, 0xF
);

