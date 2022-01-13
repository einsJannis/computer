use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use std::ops::{Add, Index, RangeBounds};
use std::process::Output;
use crate::{Address, AddressValue, AssemblyProgram, Flag, Instruction, Register, Value};
use crate::util::when;

#[derive(Debug)]
struct ParseError {
    content: String,
    index: usize,
    name: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Tried to parse {} but failed at index {} with the following content:\n{}", self.name, self.index, self.content))
    }
}

impl Error for ParseError {}

type ParseResult<T: Parseable> = Result<T, ParseError>;

pub trait Parseable : Sized {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self>;
}

fn next_term(content: &str, &i: usize) -> usize {
    (content.index(i..) as &str)
        .find(' ')
        .or(content[i..].find('\n'))
        .unwrap_or(content.len())
}

impl Parseable for String {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        let last: usize = (content.index(i..).find(' ')
            .or(content[i..].find('\n')) as Option<usize>).unwrap_or(content.len());
        let string = &content[i..last];
        i = last + 1;
        Ok(string.to_string())
    }
}

impl Parseable for i8 {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        let last: usize = next_term(content, i);
        let string = &content[i..last];
        let result = i8::from_str(string).map_err(|_| {
            ParseError {
                content: content.to_string(),
                index: i,
                name: "i8".to_string()
            }
        })?;
        i = last + 1;
        Ok(result)
    }
}

impl Parseable for u16 {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        let last: usize = next_term(content, i);
        let string = &content[i..last];
        let result = u16::from_str(string).map_err(|_| {
            ParseError {
                content: content.to_string(),
                index: i,
                name: "u16".to_string()
            }
        })?;
        i = last + 1;
        Ok(result)
    }
}

impl Parseable for Register {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        fn error() -> ParseError {
            ParseError { content: content.into(), index: i, name: "Register".into() }
        }
        if content[i..].starts_with("reg") {
            i += 3;
            let result = Register::NUMBERED(u8::from_str(content[i]).map_err(|_| {
                i -= 3; error()
            })?);
            i += 1;
            if content[i] != ' ' && content[i] != '\n' {
                i -= 4;
                return Err(error())
            }
            i += 1;
            Ok(result)
        } else if content[i..].starts_with("high") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 4;
            Ok(Register::HIGH)
        } else if content[i..].starts_with("low") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 3;
            Ok(Register::LOW)
        } else if content[i..].starts_with("pc_high") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 7;
            Ok(Register::PC_HIGH)
        } else if content[i..].starts_with("pc_low") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 6;
            Ok(Register::PC_LOW)
        } else if content[i..].starts_with("stack_ptr") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 9;
            Ok(Register::STACK_PTR)
        } else if content[i..].starts_with("flags") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 5;
            Ok(Register::FLAG)
        } else {
            Err(error());
        }
    }
}

impl Parseable for Value {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        Register::parse(content, i).map(|it| Value::Register(it))
            .or_else(|_| i8::parse(content, i).map(|it| Value::Literal(it)))
    }
}

impl Parseable for AddressValue {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        u16::parse(content, i).map(|it| AddressValue::Literal(it))
            .or_else(|_| String::parse().map(|it| AddressValue::Label(it)))
    }
}

impl Parseable for Address {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        fn error() -> ParseError {
            ParseError { content: content.into(), index: i, name: "Address".into() }
        }
        if content.starts_with("HL") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 2;
            Ok(Address::HL)
        } else { AddressValue::parse(content, i) }
    }
}

impl Parseable for Flag {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        fn error() -> ParseError {
            ParseError { content: content.into(), index: i, name: "Address".into() }
        }
        if content[i..].starts_with("flag") {
            i += 4;
            let value = u8::from_str(content[i]).map_err(|_| {
                i -= 4;
                error()
            })?;
            i += 1;
            if content[i] != ' ' && content[i] != '\n' {
                i -= 5;
                return Err(error())
            }
            Ok(Flag::NUMBERED(value))
        } else if content[i..].starts_with("halt") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 5;
            Ok(Flag::HALT)
        } else if content[i..].starts_with("carry") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 6;
            Ok(Flag::CARRY)
        } else if content[i..].starts_with("borrow") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 7;
            Ok(Flag::BORROW)
        } else if content[i..].starts_with("overflow") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 9;
            Ok(Flag::OVERFLOW)
        } else if content[i..].starts_with("less") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 5;
            Ok(Flag::LESS)
        } else if content[i..].starts_with("equal") {
            if content[i] != ' ' && content[i] != '\n' { return Err(error()) }
            i += 6;
            Ok(Flag::EQUAL)
        } else {
            Err(error())
        }
    }
}

impl Parseable for Instruction {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        fn error() -> ParseError {
            ParseError { content: content.to_string(), index: i, name: "Instruction".to_string() }
        }
        if content[i..].starts_with("@") {
            i += 1;
            let last = content[i..].find(':').map_err(|_| {
                i -= 1;
                error()
            })?;
            let name = content[i..last];
            i = last + 1;
            Ok(Instruction::LABEL(name))
        } else if content[i..].starts_with("nop\n") {
            i += 4;
            Ok(Instruction::NOP)
        } else if content[i..].starts_with("mov ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::MOV(register, value))
        } else if content[i..].starts_with("ldw ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let address = Address::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::LDW(register, address))
        } else if content[i..].starts_with("stw ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let address = Address::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::STW(register, address))
        } else if content[i..].starts_with("lda ") {
            i += 4;
            let address = Address::parse(content, i).map_err(|it| {
                i -= 4;
                it
            })?;
            Ok(Instruction::LDA(address))
        } else if content[i..].starts_with("psh ") {
            i += 4;
            let value = Value::parse(content, i).map_err(|it| {
                i -= 4;
                it
            })?;
            Ok(Instruction::PSH(value))
        } else if content[i..].starts_with("pop ") {
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i -= 4;
                it
            })?;
            Ok(Instruction::POP(register))
        } else if content[i..].starts_with("jmp ") {
            let result = i;
            i += 4;
            let flag = Flag::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let address = Address::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::JMP(flag, address))
        } else if content[i..].starts_with("add ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::ADD(register, value))
        } else if content[i..].starts_with("sub ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::SUB(register, value))
        } else if content[i..].starts_with("and ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::AND(register, value))
        } else if content[i..].starts_with("or ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::OR(register, value))
        } else if content[i..].starts_with("INV ") {
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i -= 4;
                it
            })?;
            Ok(Instruction::INV(register))
        } else if content[i..].starts_with("cmp ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::CMP(register, value))
        } else if content[i..].starts_with("shl ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::SHL(register, value))
        } else if content[i..].starts_with("shr ") {
            let result = i;
            i += 4;
            let register = Register::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            let value = Value::parse(content, i).map_err(|it| {
                i = result;
                it
            })?;
            Ok(Instruction::SHR(register, value))
        } else {
            Err(error())
        }
    }
}

impl Parseable for AssemblyProgram {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        let mut instructions: Vec<Instruction> = vec![];
        loop {
            if i == content.len() {
                break;
            } else if content[i..].starts_with("\n") {
                i += 1;
            } else if content[i..].start_with("#") {
                i = content[i..].find('\n') + 1
            } else {
                instructions += Instruction::parse(content, i)?;
            }
        }
        Ok(AssemblyProgram { instructions })
    }
}
