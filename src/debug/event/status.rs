//! [`brainfucklib::debug::event::status`]
//! 
//! This module defines [`Status`], a struct that tells the user that an
//! operation was ok.

use std::fmt;

/// A status for an operation involving the Brainfuck virtual machine and
/// the grammar parser.
#[derive(Clone, Debug)]
pub struct Status {
    description: String,
}

impl Status {
    /// Creates a new [`Status`].
    pub fn new(description: impl AsRef<str>) -> Self {
        let description = description.as_ref().to_string();
        return Self {description};
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Status: {}", self.description);
    }
}