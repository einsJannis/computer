use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::{ADD, Address, AND, AssemblyProgram, CMP, Flag, Instruction, IntoBytes, INV, JMP, Label, LDA, LDW, MOV, NOP, OR, POP, PSH, Register, SHL, SHR, STW, SUB, Value};

pub enum Token {
    label(String),
    NOP,
    MOV,
    LDW,
    STW,
    LDA,
    PSH,
    POP,
    JMP,
    ADD,
    SUB,
    AND,
    OR,
    INV,
    CMP,
    SHL,
    SHR,
    reg(u8),
    r_high,
    r_low,
    r_pc_high,
    r_pc_low,
    r_stack_ptr,
    r_flag,
    flag(u8),
    carry,
    borrow,
    overflow,
    less,
    equal,
    number(i32),
    label_ref(String)
}

pub struct TokenIterator {
    vector: Vec<Token>,
    indices: Vec<usize>,
    i: usize
}

impl TokenIterator {
    pub fn push(&mut self) {
        self.indices.push(i);
    }
    pub fn pop(&mut self) {
        self.indices.pop().and_then(|it| { self.i = it; None });
    }
    pub fn spop(&mut self) {
        self.indices.pop();
    }
}

impl Iterator for TokenIterator {
    type Item = Token;
    fn next(&mut self) -> Option<&Self::Item> {
        Some(&self.vector[self.i])
    }
}

fn word(current: &str, &mut i: usize) -> &str {
    let index = current.find(" ").or_else(current.find("\n")).get_or_insert(current.len());
    i += index;
    current[..index]
}

fn next_token(content: &str, &mut i: usize) -> Option<Token> {
    let current: &str = content.get(i..)?;
    if current.starts_with("\n") {
        i += 1;
        return None
    } else if current.starts_with("#") {
        i += current.find("\n");
        return None
    }
    match word(current, i) {
        "nop" => Some(Token::NOP),
        "mov" => Some(Token::MOV),
        "ldw" => Some(Token::LDW),
        "stw" => Some(Token::STW),
        "lda" => Some(Token::LDA),
        "psh" => Some(Token::PSH),
        "pop" => Some(Token::POP),
        "jmp" => Some(Token::JMP),
        "add" => Some(Token::ADD),
        "sub" => Some(Token::SUB),
        "and" => Some(Token::AND),
        "or"  => Some(Token::OR ),
        "inv" => Some(Token::INV),
        "cmp" => Some(Token::CMP),
        "shl" => Some(Token::SHL),
        "shr" => Some(Token::SHR),
        "high" => Some(Token::r_high),
        "low" => Some(Token::r_low),
        "pc_high" => Some(Token::r_pc_high),
        "pc_low" => Some(Token::r_pc_low),
        "stack_ptr" => Some(Token::r_stack_ptr),
        "flag" => Some(Token::r_flag),
        "carry" => Some(Token::carry),
        "borrow" => Some(Token::borrow),
        "overflow" => Some(Token::overflow),
        "less" => Some(Token::less),
        "equal" => Some(Token::equal),
        word => {
            if word.starts_with("@") {
                if word.ends_with(":") {
                    Some(Token::label(word.get(1..word.len())?.to_string()))
                } else {
                    Some(Token::label_ref(word.get(1..)?.to_string()))
                }
            } else if word.starts_with("reg") {
                if word.len() > 4 { panic!() }
                let n = u8::from_str(word.get(3)?)?;
                if n > 7 { panic!() }
                Some(Token::reg(n))
            } else if word.starts_with("flag") {
                if word.len() > 5 { panic!() }
                let n = u8::from_str(word.get(4)?)?;
                if n > 7 { panic!() }
                Some(Token::flag(n))
            } else {
                Some(i32::from_str(word).map(|it| Token::number(it))?)
            }
        }
    }
}

fn lex(content: &str) -> TokenIterator {
    let mut i = 0;
    let mut res = Vec::new();
    loop {
        if i == content.len() {
            break
        }
        next_token(content, i).and_then(|it| {res += it; None});
    }
    return TokenIterator { vector: res, indices: vec![], i }
}

#[derive(Debug)]
struct ParseError {
    iterator: TokenIterator,
    name: String,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Failed to parse {} at token {}", self.name, self.iterator.i))
    }
}

impl Error for ParseError {}

type ParseResult<T> = Result<T, ParseError>;

pub trait Parsable {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self>;
}

impl Parsable for Register {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        tokens.push();
        let result = Register(match tokens.next()? {
            Token::reg(n) => n.clone(),
            Token::r_high => 2,
            Token::r_low => 3,
            Token::r_pc_high => 4,
            Token::r_pc_low => 5,
            Token::r_stack_ptr => 6,
            Token::r_flag => 7,
            _ => {
                tokens.pop();
                return Err(ParseError { iterator: tokens.clone(), name: "Register".to_string() })
            }
        });
        tokens.spop();
        Ok(result)
    }
}

impl Parsable for Value {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Register::parse(tokens).map(|it| Value::Register(it)).or_else(|err| {
            tokens.push();
            if let Token::number(n) = tokens.next() {
                tokens.spop();
                return Ok(Value::Literal(n as i8));
            }
            tokens.pop();
            Err(err)
        })
    }
}

impl Parsable for Label {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        tokens.push();
        if let Token::label_ref(name) = tokens.next()? {
            tokens.spop();
            return Ok(Label { name: name.clone() })
        }
        tokens.pop();
        Err(ParseError { iterator: tokens.clone(), name: "Label".to_string() })
    }
}

impl SimpleInstruction for NOP {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::NOP => true, _ => false } }
}

impl InnerParse for NOP {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> { Ok(NOP) }
}

impl SimpleInstruction for MOV {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::MOV => true, _ => false } }
}

impl InstructionRegVal for MOV {
    fn new(arg0: Register, arg1: Value) -> Self {
        MOV(arg0, arg1)
    }
}

impl SimpleInstruction for LDW {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::LDW => true, _ => false } }
}

impl InstructionRegAddr for LDW {
    fn new(arg0: Register, arg1: Address) -> Self { LDW(arg0, arg1) }
}

impl SimpleInstruction for STW {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::STW => true, _ => false } }
}

impl InstructionRegAddr for STW {
    fn new(arg0: Register, arg1: Address) -> Self { STW(arg0, arg1) }
}

impl SimpleInstruction for LDA {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::LDA => true, _ => false } }
}

impl InnerParse for LDA {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Ok(LDA(Address::parse(tokens)?))
    }
}

impl SimpleInstruction for PSH {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::PSH => true, _ => false } }
}

impl InnerParse for PSH {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Ok(PSH(Value::parse(tokens)?))
    }
}

impl SimpleInstruction for POP {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::POP => true, _ => false } }
}

impl InstructionReg for POP {
    fn new(arg0: Register) -> Self { POP(arg0) }
}

impl SimpleInstruction for JMP {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::JMP => true, _ => false } }
}

impl InnerParse for JMP {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Ok(JMP(Flag::parse(tokens)?, Address::parse(tokens)?))
    }
}

impl SimpleInstruction for ADD {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::ADD => true, _ => false } }
}

impl InstructionRegVal for ADD {
    fn new(arg0: Register, arg1: Value) -> Self { ADD(arg0, arg1) }
}

impl SimpleInstruction for SUB {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::SUB => true, _ => false } }
}

impl InstructionRegVal for SUB {
    fn new(arg0: Register, arg1: Value) -> Self { SUB(arg0, arg1) }
}

impl SimpleInstruction for AND {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::AND => true, _ => false } }
}

impl InstructionRegVal for AND {
    fn new(arg0: Register, arg1: Value) -> Self { AND(arg0, arg1) }
}

impl SimpleInstruction for OR {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::OR => true, _ => false } }
}

impl InstructionRegVal for OR {
    fn new(arg0: Register, arg1: Value) -> Self { OR(arg0, arg1) }
}

impl SimpleInstruction for INV {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::INV => true, _ => false } }
}

impl InstructionReg for INV {
    fn new(arg0: Register) -> Self { INV(arg0) }
}

impl SimpleInstruction for CMP {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::CMP => true, _ => false } }
}

impl InstructionRegVal for CMP {
    fn new(arg0: Register, arg1: Value) -> Self { CMP(arg0, arg1) }
}

impl SimpleInstruction for SHL {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::SHL => true, _ => false } }
}

impl InstructionRegVal for SHL {
    fn new(arg0: Register, arg1: Value) -> Self { SHL(arg0, arg1) }
}

impl SimpleInstruction for SHR {
    fn is_token_op_name(token: &Token) -> bool { match token { Token::SHR => true, _ => false } }
}

impl InstructionRegVal for SHR {
    fn new(arg0: Register, arg1: Value) -> Self { SHR(arg0, arg1) }
}

trait InstructionReg: SimpleInstruction {
    fn new(arg0: Register) -> Self;
}

trait InstructionRegAddr: SimpleInstruction {
    fn new(arg0: Register, arg1: Address) -> Self;
}

trait InstructionRegVal: SimpleInstruction {
    fn new(arg0: Register, arg1: Value) -> Self;
}

trait SimpleInstruction: Instruction {
    fn is_token_op_name(token: &Token) -> bool;
}

trait InnerParse {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self>;
}

impl<T> Parsable for T where T: SimpleInstruction + InnerParse {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        tokens.push();
        if Self::is_token_op_name(tokens.next()?) {
            let result = Self::inner_parse(tokens)?;
            tokens.spop();
            return Ok(result)
        }
        tokens.pop();
        Err(ParseError { iterator: tokens.clone(), name: "Instruction".to_string() })
    }
}

impl<T> InnerParse for T where T: InstructionReg {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Ok(Self::new(Register::parse(tokens)?))
    }
}

impl<T> InnerParse for T where T: InstructionRegAddr {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Ok(Self::new(Register::parse(tokens)?, Address::parse(tokens)?))
    }
}

impl<T> InnerParse for T where T: InstructionRegVal {
    fn inner_parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        Self::new(Register::parse(tokens)?, Value::parse(tokens)?)
    }
}

impl Parsable for AssemblyProgram {
    fn parse(tokens: &mut TokenIterator) -> ParseResult<Self> {
        let mut res = Vec::new();
        loop {
            if tokens.i == tokens.vector.len() { break; }
            res += Label::parse(tokens).map(|it| {it as Instruction})
                .or_else(||NOP::parse(tokens))
                .or_else(||MOV::parse(tokens))
                .or_else(||LDW::parse(tokens))
                .or_else(||STW::parse(tokens))
                .or_else(||LDA::parse(tokens))
                .or_else(||PSH::parse(tokens))
                .or_else(||POP::parse(tokens))
                .or_else(||JMP::parse(tokens))
                .or_else(||ADD::parse(tokens))
                .or_else(||SUB::parse(tokens))
                .or_else(||AND::parse(tokens))
                .or_else(||OR ::parse(tokens))
                .or_else(||INV::parse(tokens))
                .or_else(||CMP::parse(tokens))
                .or_else(||SHL::parse(tokens))
                .or_else(||SHR::parse(tokens))
        }
        Ok(AssemblyProgram(res))
    }
}
