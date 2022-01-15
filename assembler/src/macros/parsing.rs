use crate::macros::{ExpandableInstruction, MacroArgument, MacroInstruction, MacroMapping};
use crate::parsing::{parse_label_call, Parseable, ParseError, ParseResult};
use crate::{Address, Flag, Instruction, Register, Value};

fn parse_macro_argument_call(content: &str, &mut i: usize) -> ParseResult<String> {
    if content[i..].starts_with("$") {
        i += 1;
        Ok(String::parse(content, i).map_err(|it| {i -= 1; it})?)
    } else { Err(ParseError { content: content.to_string(), index: i, name: "MacroArgumentCall".to_string() }) }
}

impl<T> Parseable for MacroMapping<T> where T: Parseable {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        parse_macro_argument_call(content, i).map(|it| MacroMapping::MacroArgument(it))
            .or_else(|_| Self::parse(content, i).map(|it| MacroMapping::Literal(it)))
    }
}

impl Parseable for MacroArgument {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        ExpandableInstruction::parse(content, i).map(|it| MacroArgument::Instruction(it))
            .or_else(|_| Register::parse(content, i).map(|it| MacroArgument::Register(it)))
            .or_else(|_| Value::parse(content, i).map(|it| MacroArgument::Value(it)))
            .or_else(|_| Address::parse(content, i).map(|it| MacroArgument::Address(it)))
            .or_else(|_| Flag::parse(content, i).map(|it| MacroArgument::Flag(it)))
            .or_else(|_| String::parse(content, i).map(|it| MacroArgument::Identifier(it)))
    }
}

fn parse_macro_macro_call(content: &str, &mut i: usize) -> ParseResult<MacroInstruction> {
    if content[i..].starts_with("!") {
        i += 1;
        let name = String::parse(content, i).map_err(|it| {i -= 1; it})?;
        let mut args = vec![];
        loop {
            let result = MacroMapping::<MacroArgument>::parse(content, i)
            match result {
                Ok(it) => args += it,
                Err(_) => break
            }
        }
        Ok(MacroInstruction::MacroCall(name, args))
    } else { Err(ParseError { content: content.to_string(), index: i, name: "MacroCall".to_string() }) }
}

impl Parseable for MacroInstruction {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        parse_macro_argument_call(content, i).map(|it| MacroInstruction::MacroArgument(it))
            .or_else(parse_macro_macro_call(content, i))
            .or_else(parse_label_call(content, i))
            .or_else()
    }
}

impl Parseable for ExpandableInstruction {
    fn parse(content: &str, &mut i: usize) -> ParseResult<Self> {
        todo!()
    }
}
