//! [`brainfucklib::debug::event::warning`]
//! 
//! This module defines [`Warning`], which tells the user that something wrong
//! may have happened but can be glossed over with/without error handling.

use std::fmt;

/// A [`Warning`] indicates that something has gone wrong, but can be ignored
/// unlike an error.
#[derive(Clone, Debug)]
pub struct Warning {
    description: String,
}

impl Warning {
    /// Creates a new [`Warning`].
    pub fn new(description: impl AsRef<str>) -> Self {
        let description = description.as_ref().to_string();
        return Self {description};
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "Warning: {}", self.description);
    }
}