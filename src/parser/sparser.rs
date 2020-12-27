use pest::{Parser, iterators::Pairs};

use crate::{
    code::{BfProgram, BfCommand, BfToken, Span},
    debug::{Event, OkEvent, Status, ErrEvent, BfError, BfErrorKind}
};
use super::BfParser;

#[derive(Parser)]
#[grammar = "./parser/simple.pest"]
pub struct NormalParser;

impl NormalParser {
    pub fn parse_ast(ast: Pairs<Rule>, program: &mut BfProgram) -> Event {
        use Rule::*;
        use BfCommand::*;
        for pair in ast {
            let span = pair.as_span();
            let code = pair.as_str();
            let reserved = match pair.as_rule() {
                Rule::RESERVED => match pair.into_inner().next() {
                    Some(r) => r,
                    None => return Err(ErrEvent::Error(BfError::new(
                        BfErrorKind::ParsingError,
                        format!(
                            "Could not parse this: {} at {}",
                            code,
                            span.start()
                        )
                    )))
                }
                _ => continue,
            };
            let command: BfCommand = match reserved.as_rule() {
                INCREMENT => Increment,
                DECREMENT => Decrement,
                TO_PREV => Previous,
                TO_NEXT => Next,
                OUTPUT => Read,
                INPUT => Write,
                LEFT_BRACKET => IfZero,
                RIGHT_BRACKET => IfNotZero,
                _ => continue,
            };
            let token = BfToken::new(command, Span::from(span));
            program.push(token);
        }
        return Ok(OkEvent::Status(Status::new("Parse AST: OK")));
    }
}

impl BfParser for NormalParser {
    fn parse_string<S>(script: S, program: &mut BfProgram) -> Event
    where
        S: AsRef<str>
    {
        let script = script.as_ref();
        let ast = match Self::parse(Rule::FILE, script) {
            Ok(parsed) => parsed,
            Err(_) => return Err(ErrEvent::Error(BfError::new(
                BfErrorKind::BadProgram,
                "Could not parse string."
            ))),
        };
        return Self::parse_ast(ast, program);
    }
}