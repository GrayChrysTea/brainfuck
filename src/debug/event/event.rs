//! [`brainfucklib::debug::event::event`]
//! 
//! This module defines [`Event`], a [`Result`] whose [`Ok`] and [`Err`] value
//! are [`OkEvent`] and [`ErrEvent`] respectively.

use std::fmt;
use super::{BfError, Status, Warning};

/// A custom [`Result`] returned by a Brainfuck virtual machine and a parser.
pub type Event = Result<OkEvent, ErrEvent>;

/// A [`OkEvent`] is an event that is not an error, such that no error
/// handling would be required for it.
#[derive(Clone, Debug)]
pub enum OkEvent {
    /// A status, alerting the user that an operation was successful.
    Status(Status),
    /// A warning, alerting the user that something unsafe might have
    /// happened which does not warrant a crash.
    Warning(Warning),
}

impl fmt::Display for OkEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", match self {
            OkEvent::Status(status) => format!("{}", status),
            OkEvent::Warning(warning) => format!("{}", warning),
        });
    }
}

/// An [`ErrEvent`] is an error which requires error handling. This can happen
/// when a pointer in a memory tape in [`crate::vm::Memory`] tries to access
/// a wrong
#[derive(Clone, Debug)]
pub enum ErrEvent {
    /// A warning which requires error handling, not as bad as
    /// [`ErrEvent::Error`].
    Warning(Warning),
    /// A generic error which might require crashing the program.
    Error(BfError),
}

impl fmt::Display for ErrEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", match self {
            ErrEvent::Error(error) => format!("{}", error),
            ErrEvent::Warning(warning) => format!("{}", warning),
        });
    }
}