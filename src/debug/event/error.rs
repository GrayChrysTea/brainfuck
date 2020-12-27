//! [`brainfucklib::debug::event::error`]
//! 
//! This module defines [`BfError`] and [`BfErrorKind`], which are custom
//! [`Error`] types used by [`brainfucklib`].

use std::{
    error::Error,
    fmt
};

/// The kind of error that occurred. This enum can be compared to
/// [`std::io::Error`].
#[derive(Clone, Copy, Debug)]
pub enum BfErrorKind {
    UnrecognizedCommand,
    UnmatchedLeftBracket,
    UnmatchedRightBracket,
    BadProgram,
    ParsingError,

    OutOfBounds,
    PointerError,
    CellOverflow,

    Other,
}

impl fmt::Display for BfErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BfErrorKind::*;
        return write!(f, "{}", match self {
            UnrecognizedCommand => "UnrecognizedCommand",
            UnmatchedLeftBracket => "UnmatchedLeftBracket",
            UnmatchedRightBracket => "UnmatchedRightBracket",
            BadProgram => "BadProgram",
            ParsingError => "ParsingError",
            OutOfBounds => "OutOfBounds",
            PointerError => "PointerError",
            CellOverflow => "CellOverflow",
            Other => "Other"
        });
    }
}

/// A [`BfError`] is an error which has a [`BfErrorKind`] and a description
/// which tells you what has gone wrong.
#[derive(Clone, Debug)]
pub struct BfError {
    kind: BfErrorKind,
    description: String,
}

impl BfError {
    /// Create a new instance of [`BfError`]. You need to input a
    /// [`BfErrorKind`] and a description of any type that can be converted
    /// into a [`&str`] (using [`AsRef<str>`]).
    pub fn new(kind: BfErrorKind, description: impl AsRef<str>) -> Self {
        let description = description
            .as_ref()
            .to_string();
        return Self {kind, description};
    }
}

impl fmt::Display for BfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}: {}", self.kind, self.description);
    }
}

impl Error for BfError {}