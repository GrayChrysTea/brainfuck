//! [`brainfucklib::code::commands`]
//! 
//! This module provides representations of Brainfuck's 8 commands as an
//! enum.

use std::fmt;

use crate::debug::{BfResult, BfError, BfErrorKind};
use super::{Bracket, BType, Span};

/// This is a representation of Brainfuck commands, which can take any of the
/// 8 variants here.
#[derive(Clone, Copy, Debug)]
pub enum BfCommand {
    /// '+'
    Increment,
    /// '-'
    Decrement,
    /// '<'
    Previous,
    /// '>'
    Next,
    /// '.'
    Read,
    /// ','
    Write,
    /// '['
    IfZero,
    /// ']'
    IfNotZero,
}

impl BfCommand {
    /// Tries to create a command from a string such as "+".
    /// 
    /// If an invalid string is passed in, an error is returned.
    pub fn new(command: impl AsRef<str>) -> BfResult<Self> {
        use BfCommand::*;
        let command = command.as_ref();
        return Ok(match command {
            "+" => Increment,
            "-" => Decrement,
            "<" => Previous,
            ">" => Next,
            "." => Read,
            "," => Write,
            "[" => IfZero,
            "]" => IfNotZero,
            _ => return Err(BfError::new(
                BfErrorKind::UnrecognizedCommand,
                format!("{} is not a valid command.", command)
            )),
        });
    }

    pub fn bracket_kind(&self) -> Option<u8> {
        use BfCommand::*;
        return match self {
            IfZero => Some(1),
            IfNotZero => Some(1),
            _ => None,
        };
    }

    pub fn bracket_side(&self) -> Option<BType> {
        use BfCommand::*;
        return match self {
            IfZero => Some(BType::Left),
            IfNotZero => Some(BType::Right),
            _ => None,
        }
    }

    pub fn bracket(&self) -> Option<Bracket> {
        return Some(Bracket::new(self.bracket_side()?, self.bracket_kind()?));
    }
}

impl fmt::Display for BfCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BfCommand::*;
        return write!(f, "{}", match self {
            Increment => "+",
            Decrement => "-",
            Previous => "<",
            Next => ">",
            Read => ".",
            Write => ",",
            IfZero => "[",
            IfNotZero => "]",
        });
    }
}

/// A Brainfuck command with its context in the program.
#[derive(Clone, Debug)]
pub struct BfToken {
    command: BfCommand,
    span: Span,
}

impl BfToken {
    /// Creates a new [`BfToken`].
    pub fn new(command: BfCommand, span: Span) -> Self {
        return Self {command, span};
    }

    /// Get the command in the token.
    pub fn command(&self) -> BfCommand {
        return self.command;
    }

    /// Get the span in the token.
    pub fn span(&self) -> Span {
        return self.span;
    }
}