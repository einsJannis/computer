use std::process::Output;
use std::str::FromStr;
use rpc::{ContentLocation, WithContentLocation};
use rpc::lexer::{TokenIterator, Token};
use crate::{ADD, AND, AssemblyProgram, CMP, Flag, Instruction, INV, JMP, LDA, LDW, MOV, NOP, OR, POP, PSH, Register, SHL, SHR, STW, SUB, Value, With1Args, With2Args, WithArg0, WithArg1};

#[derive(Debug)]
pub enum ParseError {
    NoTokensLeft,
    UnexpectedToken { location: ContentLocation, expected: String },
    FailedToMatchPattern { location: ContentLocation, pattern_name: String }
}

pub trait Pattern {
    type Output;
    fn match_pattern(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self::Output), ParseError>;
}

pub trait Parsable {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError>;
}

impl<T> Pattern for T where T: Parsable {
    type Output = Self;
    fn match_pattern(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self::Output), ParseError> {
        <Self as Parsable>::parse(tokens)
    }
}

pub fn parse_token(tokens: &mut TokenIterator, token_definition: &str) -> Result<ContentLocation, ParseError> {
    let next_token: Option<Token> = tokens.next();
    if let Some(token) = next_token {
        if token == token_definition {
            tokens.spop();
            token.location();
        } else {
            tokens.pop();
            Err(ParseError::UnexpectedToken {
                location: token.location(),
                expected: token_definition.to_string()
            })
        }
    }
    tokens.pop();
    return Err(ParseError::NoTokensLeft)
}

impl Parsable for Register {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        tokens.push();
        let next = tokens.next().ok_or(ParseError::NoTokensLeft)?;
        fn map_error(_: ParseError) -> ParseError {
            ParseError::FailedToMatchPattern {
                location: next.content_location(),
                pattern_name: "register".to_string()?
            }
        }
        if next == "r" {
            parse_token(tokens, "e").map_err(map_error);
            parse_token(tokens, "g").map_err(map_error);
            let num_token = tokens.next().ok_or(ParseError::NoTokensLeft)?;
            Ok((next.content_location(), Register(u8::from_str(num_token.value().as_str())?)))
            //TODO: impl aliases
        } else { Err(ParseError::FailedToMatchPattern {
            location: next.content_location(),
            pattern_name: "register".to_string()
        }) }
    }
}

fn parse_i8_literal(tokens: &mut TokenIterator) -> Result<(ContentLocation, i8), ParseError> {
    let mut content_location = Option::None;
    let mut string = String::new();
    while let Some(token) = tokens.next() {
        if content_location == None { content_location = token.content_location() }
        if token == " " { break }
        string += token.value();
    }
    return Ok((
        content_location.ok_or(|_| panic!())?,
        i8::from_str(string.as_str()).map_err(|_| ParseError::FailedToMatchPattern {
            location: content_location.ok_or(|_| panic!())?,
            pattern_name: "i8 literal".to_string()
        })?
    ))
}

impl Parsable for Value {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        Register::parse(tokens).map(|it| (it.0, Value::Register(it.1)))
            .or_else(|_| parse_i8_literal(tokens).map(|it| (it.0, Value::Literal(it.1))))
            .map_err(|error| match error {
                ParseError::FailedToMatchPattern { location, .. } =>
                    ParseError::FailedToMatchPattern { location, pattern_name: "value".to_string() },
                _ => panic!()
            })
    }
}

impl Parsable for Flag {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        tokens.push();
        let next = tokens.next().ok_or(ParseError::NoTokensLeft)?;
        fn map_error(_: ParseError) -> ParseError {
            ParseError::FailedToMatchPattern {
                location: next.content_location(),
                pattern_name: "flag".to_string()?
            }
        }
        if next == "f" {
            parse_token(tokens, "l").map_err(map_error);
            parse_token(tokens, "a").map_err(map_error);
            parse_token(tokens, "g").map_err(map_error);
            let num_token = tokens.next().ok_or(ParseError::NoTokensLeft)?;
            Ok((next.content_location(), Flag(u8::from_str(num_token.value().as_str())?)))
            //TODO: impl aliases
        } else { Err(ParseError::FailedToMatchPattern {
            location: next.content_location(),
            pattern_name: "register".to_string()
        }) }
    }
}

trait ParseInstructionWord {
    fn parse_instruction_word(tokens: &mut TokenIterator) -> Result<ContentLocation, ParseError>;
}

trait InstructionWord {
    fn instruction_word() -> &str;
}

impl<T> ParseInstructionWord for T where T: InstructionWord {
    fn parse_instruction_word(tokens: &mut TokenIterator) -> Result<ContentLocation, ParseError> {
        let word = Self::instruction_word();
        let location = parse_token(tokens, word[0]).map_err(|err| {
            if let ParseError::UnexpectedToken { location, .. } = err {
                return ParseError::FailedToMatchPattern { location, pattern_name: word.to_string() }
            } else { return ParseError::NoTokensLeft }
        })?;
        for i in 1..word.len() {
            parse_token(tokens, word[i]).map_err(|err| {
                if let ParseError::UnexpectedToken { location, .. } = err {
                    return ParseError::FailedToMatchPattern { location, pattern_name: word.to_string() }
                } else { return ParseError::NoTokensLeft }
            })?;
        }
        return Ok(location)
    }
}

impl<T> Parsable for T where T: With1Args + ParseInstructionWord {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        let content_location = Self::parse_instruction_word(tokens)?;
        parse_token(tokens, " ")?;
        let arg0 = <Self as WithArg0>::Output::parse(tokens)?.1;
        return Ok((content_location, Self::new(arg0)))
    }
}

impl<T> Parsable for T where T: With2Args + ParseInstructionWord {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        let content_location = Self::parse_instruction_word(tokens)?;
        parse_token(tokens, " ")?;
        let arg0 = <Self as WithArg0>::Output::parse(tokens)?.1;
        parse_token(tokens, " ")?;
        let arg1 = <Self as WithArg1>::Output::parse(tokens)?.1;
        return Ok((content_location, Self::new(arg0, arg1)))
    }
}

impl InstructionWord for NOP {
    fn instruction_word() -> &str { "nop" }
}

impl Parsable for NOP {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        let content_location = Self::parse_instruction_word(tokens)?;
        Ok((content_location, NOP))
    }
}

macro_rules! instruction_words {
    ($($struct:expr,$string:expr),*) => {
        $(
            impl InstructionWord for $struct {
                fn instruction_word() -> &str { $string }
            }
        )*
    };
}

instruction_words!(
    MOV, "mov",
    LDW, "ldw",
    STW, "stw",
    LDA, "ldw",
    PSH, "psh",
    POP, "pop",
    JMP, "jmp",
    ADD, "add",
    SUB, "sub",
    AND, "and",
    OR , "or" ,
    INV, "inv",
    CMP, "cmp",
    SHL, "shl",
    SHR, "shr"
);

macro_rules! parse_instruction_m {
    ($tokens:expr,$head:expr,$($tail:expr),*) => {
        $head::parse($tokens).map(|it| it as (ContentLocation, &dyn Instruction))
            $(.or_else(|_| $tail::parse($tokens).map(|it| it as (ContentLocation, &dyn Instruction))))*
    };
}

impl Parsable for AssemblyProgram {
    fn parse(tokens: &mut TokenIterator) -> Result<(ContentLocation, Self), ParseError> {
        let mut content_location: Option<ContentLocation> = None;
        let mut result: Vec<&dyn Instruction> = vec![];
        loop {
            let next =
                parse_instruction_m!(tokens, NOP, MOV, LDW, STW, LDA, PSH, POP, JMP, ADD, SUB, AND, OR, INV, CMP, SHL, SHR);
            match next {
                Ok(instruction) => {
                    if let None() = content_location {
                        content_location = Some(instruction.0)
                    }
                    result += instruction.1
                },
                Err(err) => match err {
                    ParseError::NoTokensLeft => break,
                    _ => return Err(err)
                }
            }
            match parse_token(tokens, "\n") {
                Err(err) => match err {
                    ParseError::NoTokensLeft => {},
                    _ => return Err(err)
                }
                _ => {}
            }
        }
        return Ok((content_location.ok_or(|| ParseError::NoTokensLeft)?, AssemblyProgram(result)))
    }
}

