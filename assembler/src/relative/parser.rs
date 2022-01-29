use crate::{generator::Generable, parser::{parse_token, Parsable}, With2Args, WithArg0, WithArg1};

use super::{LabeledInstruction, RelativeInstruction};

impl<T> Parsable for LabeledInstruction<T> where T: RelativeInstruction {
    fn parse(tokens: &mut rpc::lexer::TokenIterator) -> Result<(rpc::ContentLocation, Self), crate::parser::ParseError> {
        tokens.push();
        let location = parse_token(tokens, "@").map_err(|err| { tokens.pop(); err })?;
        let mut string = String::new();
        loop {
            let next = tokens.next().map_err(|err| { tokens.pop(); err })?;
            if (next == ":") {
                break;
            }
            string += next;
        }
        parse_token(tokens, " ").or_else(|_| parse_token(tokens, "\n")).map_err(|err| { tokens.pop(); err })?;
        let instruction = parse_relative_instruction(tokens).map_err(|err| { tokens.pop(); err })?;
        tokens.spop();
        return Ok((location, LabeledInstruction(string, instruction)));
    }
}

impl Parsable for LabelName {
    fn parse(tokens: &mut rpc::lexer::TokenIterator) -> Result<(rpc::ContentLocation, Self), crate::parser::ParseError> {
        tokens.push();
        let location = parse_token(tokens, "@")?;
        let mut string = String::new();
        loop {
            let next = tokens.next().map_err(|err| { tokens.pop(); err })?;
            if next == " " || next == "\n" {
                break;
            }
            string += next;
        }
        tokens.spop();
        return Ok((location, string));
    }
}

