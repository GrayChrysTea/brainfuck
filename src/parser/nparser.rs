use pest::{Parser, iterators::{Pair, Pairs}};

use crate::{
    code::{BfProgram, BfCommand, BfToken, Span},
    debug::{Event, OkEvent, Status, ErrEvent, BfError, BfErrorKind}
};
use super::BfParser;

#[derive(Parser)]
#[grammar = "./parser/brainfuck.pest"]
pub struct NewParser;

impl NewParser {
    fn parse_suite(program: &mut BfProgram, suite: Pair<Rule>) -> Event {
        for command in suite.into_inner() {
            if let Rule::INDIVIDUALS = command.as_rule() {
                let span = command.as_span();
                let pest_token = command
                    .into_inner()
                    .next()
                    .unwrap();
                let com = match pest_token.as_rule() {
                    Rule::INCREMENT => BfCommand::Increment,
                    Rule::DECREMENT => BfCommand::Decrement,
                    Rule::TO_PREV => BfCommand::Previous,
                    Rule::TO_NEXT => BfCommand::Next,
                    Rule::OUTPUT => BfCommand::Read,
                    Rule::INPUT => BfCommand::Write,
                    _ => return Err(ErrEvent::Error(BfError::new(
                        BfErrorKind::ParsingError,
                        format!(
                            "The parser got an erroneous token ({}) here: {}",
                            pest_token.as_str(),
                            pest_token.as_span().start(),
                        )
                    )))
                };
                let token = BfToken::new(com, Span::from(span));
                program.push(token);
            } else if let Rule::SCOPE = command.as_rule() {
                Self::parse_scope(program, command)?;
            }
        }
        return Ok(OkEvent::Status(Status::new("Parse Suite: OK")));
    }

    fn parse_scope(program: &mut BfProgram, scope: Pair<Rule>) -> Event {
        for command in scope.into_inner() {
            if let Rule::LEFT_BRACKET = command.as_rule() {
                let com = BfCommand::IfZero;
                let token = BfToken::new(com, Span::from(command.as_span()));
                program.push(token);
            } else if let Rule::RIGHT_BRACKET = command.as_rule() {
                let com = BfCommand::IfNotZero;
                let token = BfToken::new(com, Span::from(command.as_span()));
                program.push(token);
            } else if let Rule::SUITE = command.as_rule() {
                Self::parse_suite(program, command)?;
            }
        }
        return Ok(OkEvent::Status(Status::new("Parse Scope: OK")));
    }

    pub fn parse_ast(ast: Pairs<Rule>, program: &mut BfProgram) -> Event {
        for pair in ast {
            if let Rule::SUITE = pair.as_rule() {
                Self::parse_suite(program, pair)?;
            }
        }
        return Ok(OkEvent::Status(Status::new("Parse AST: OK")));
    }
}

impl BfParser for NewParser {
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