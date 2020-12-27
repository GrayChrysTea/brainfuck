//! [`brainfucklib::code::program`]
//! 
//! A [`BfProgram`] is a representation of a Brainfuck program.

use crate::debug::{Event, OkEvent, Status, ErrEvent, BfError};
use super::{BfToken, BracketMap};

/// A [`BfProgram`] is a representation of a Brainfuck program.
#[derive(Clone, Debug)]
pub struct BfProgram {
    commands: Box<Vec<BfToken>>,
    bracket_map: Box<BracketMap>
}

impl BfProgram {
    /// Create a new [`BfProgram`].
    pub fn new() -> Self {
        let commands: Box<Vec<BfToken>> = Box::new(Vec::new());
        let bracket_map: Box<BracketMap> = Box::new(BracketMap::new());
        return Self {commands, bracket_map};
    }

    /// Push a [`BfToken`] into the program.
    pub fn push(&mut self, command: BfToken) {
        self.commands.push(command);
    }

    /// Populate the bracket map with all the brackets in the program.
    pub fn populate_map(&mut self) -> Event {
        let mut map: BracketMap = BracketMap::new();
        for (location, command) in self.commands.iter().enumerate() {
            let bracket = match command.command().bracket() {
                Some(b) => b,
                None => continue,
            };
            map.insert(bracket, location);
        }
        *self.bracket_map = map;
        return Ok(OkEvent::Status(Status::new("Populate bracket map: OK")));
    }

    /// Check whether all the brackets are paired up. If they cannot be paired
    /// up, an error is returned.
    pub fn calculate_map(&mut self) -> Event {
        let result = self.bracket_map.pair_up();
        return match result {
            Ok(_) => Ok(
                OkEvent::Status(Status::new("Calculate bracket map: OK"))
            ),
            Err((kind, location)) => {
                let description = format!(
                    "Unmatched {} at {}",
                    self.commands
                        .get(location)
                        .unwrap()
                        .command(),
                    location
                );
                Err(ErrEvent::Error(BfError::new(
                    kind,
                    description
                )))
            },
        }
    }

    /// Clear the program.
    pub fn clear(&mut self) -> Event {
        self.commands.clear();
        *self.bracket_map = BracketMap::new();
        return Ok(OkEvent::Status(Status::new("Clear BfProgram: OK")));
    }

    /// Get the current command.
    pub fn current_command(&self, index: usize) -> Option<BfToken> {
        return Some(
            self.commands
                .get(index)?
                .clone()
        );
    }

    pub fn get_counterpart(&self, index: usize) -> Option<usize> {
        return self.bracket_map.get_counterpart(index);
    }
}

impl AsRef<Vec<BfToken>> for BfProgram {
    fn as_ref(&self) -> &Vec<BfToken> {
        return &*self.commands;
    }
}

impl AsMut<Vec<BfToken>> for BfProgram {
    fn as_mut(&mut self) -> &mut Vec<BfToken> {
        return &mut *self.commands;
    }
}

impl AsRef<BracketMap> for BfProgram {
    fn as_ref(&self) -> &BracketMap {
        return &*self.bracket_map;
    }
}