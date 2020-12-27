//! [`brainfucklib::vm::cell`]
//! 
//! This module defines a struct called [`MemoryCell`] which stores a [`i128`]
//! value inside it. This represents (as its name implies) a memory cell in
//! a Brainfuck virtual machine. The struct provides the necessary functions
//! to check inputs such as incrementing and decrementing in addition to
//! accepting [`char`]acters as arguments.

use std::fmt;
use crate::debug::{Event, OkEvent, Status};

/// This is a wrapper around [`i128`], the number that is stored in a
/// [`MemoryCell`].
pub type CellNumber = i128;

/// The default upper bound for the value in the [`MemoryCell`].
pub fn default_upper() -> CellNumber {
    return 0xFF;
}

/// The default lower bound for the value in the [`MemoryCell`].
pub fn default_lower() -> CellNumber {
    return 0x00;
}

/// A memory cell in a Brainfuck virtual machine.
/// 
/// # Fields
/// 1. `number`: [`CellNumber`]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemoryCell {
    number: CellNumber,
}

impl MemoryCell {
    /// Creates a [`MemoryCell`]. By default, `number` should be the lower
    /// limit (usually [`default_lower`]) of your virtual machine.
    pub fn new(number: CellNumber) -> Self {
        return Self {number};
    }

    /// Returns true if the cell is equal to the lower bound of the virtual
    /// machine.
    pub fn at_lowest(&self, lowest: CellNumber) -> bool {
        return self.number() == lowest;
    }

    /// Returns true if the cell is smaller than the lower bound of the
    /// virtual machine.
    pub fn below_lowest(&self, lowest: CellNumber) -> bool {
        return self.number() < lowest;
    }

    /// Returns true if the cell is equal to the upper bound of the virtual
    /// machine.
    pub fn at_highest(&self, highest: CellNumber) -> bool {
        return self.number() == highest;
    }

    /// Returns true if the cell is greater than the upper bound of the virtual
    /// machine.
    pub fn above_highest(&self, highest: CellNumber) -> bool {
        return self.number() > highest;
    }

    /// Wraps the value of the memory cell if it is not between `lowest` and
    /// `highest` (inclusive between).
    pub fn wrap(&mut self, lowest: CellNumber, highest: CellNumber) {
        let original = self.number();
        if self.below_lowest(lowest) || self.at_highest(highest) {
            *self.number_mut() = original % highest + lowest;
        }
    }

    /// Increments the value of the cell, while checking if it is within the
    /// lower and upper bounds of the virtual machine.
    pub fn increment(
        &mut self,
        lowest: CellNumber,
        highest: CellNumber
    ) -> Event {
        if self.number() == CellNumber::MAX {
            *self.number_mut() = lowest;
        } else {
            *self.number_mut() += 1;
        }
        self.wrap(lowest, highest);

        return Ok(OkEvent::Status(Status::new("Increment memory cell: OK")));
    }

    /// Decrements the value of the cell, while checking if it is within the
    /// lower and upper bounds of the virtual machine.
    pub fn decrement(
        &mut self,
        lowest: CellNumber,
        highest: CellNumber
    ) -> Event {
        if self.below_lowest(lowest) {
            self.wrap(lowest, highest);
        } else if self.at_lowest(lowest) {
            *self.number_mut() = highest;
        } else {
            *self.number_mut() -= 1;
        }

        return Ok(OkEvent::Status(Status::new("Decrement memory cell: OK")));
    }

    /// Sets the cell to `lowest` (lower bound).
    pub fn flatten(&mut self, lowest: CellNumber) {
        self.number = lowest;
    }

    /// Outputs the value of the memory cell as a character via `output`.
    pub fn to_char(&self, output: &mut char) -> Event {
        //println!("{}", self.number());
        let out = std::char::from_u32(self.number() as u32).unwrap_or('�');
        *output = out;
        return Ok(OkEvent::Status(Status::new("Output char: OK")));
    }

    /// Get the unicode number of an `input` character and store that value
    /// as the value of the [`MemoryCell`]. Of course, if the value exceeds
    /// the bounds of the virtual machine, the value is wrapped.
    pub fn from_char(
        &mut self,
        input: char,
        lowest: CellNumber,
        highest: CellNumber
    ) -> Event {
        let mut value = input as CellNumber;
        if lowest <= value && value < highest {
            *self.number_mut() = value;
        } else {
            value = value % highest + lowest;
            *self.number_mut() = value;
        }
        return Ok(OkEvent::Status(Status::new("Input char: OK")));
    }

    /// Gets a mutable reference to the value inside the [`MemoryCell`].
    /// 
    /// This function is private.
    fn number_mut(&mut self) -> &mut CellNumber {
        return &mut self.number;
    }

    /// Gets the value in the [`MemoryCell`]. This value is only a copy of the
    /// value when the method was called, and is not a reference to the value
    /// inside the cell.
    pub fn number(&self) -> CellNumber {
        return self.number;
    }

    /// Gets the minimum possible value of a [`MemoryCell`].
    pub fn minimum() -> Self {
        return Self::new(CellNumber::MIN);
    }

    /// Gets the maximum possible value of a [`MemoryCell`].
    pub fn maximum() -> Self {
        return Self::new(CellNumber::MAX);
    }
}

impl Into<CellNumber> for MemoryCell {
    /// Converts the [`MemoryCell`] into a [`CellNumber`]. As [`MemoryCell`]
    /// implements [`Copy`], it means that the original [`MemoryCell`] will
    /// not be destroyed.
    fn into(self) -> CellNumber {
        return self.number();
    }
}

impl From<CellNumber> for MemoryCell {
    /// Creates a [`MemoryCell`] from a [`CellNumber`].
    fn from(number: CellNumber) -> Self {
        return Self::new(number);
    }
}

impl fmt::Display for MemoryCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.number());
    }
}

impl Default for MemoryCell {
    /// Returns a [`MemoryCell`] initialized with a value of [`default_lower`].
    fn default() -> Self {
        return Self::new(default_lower());
    }
}