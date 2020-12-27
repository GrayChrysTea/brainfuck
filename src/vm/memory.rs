//! [`brainfucklib::vm::memory`]
//! 
//! This module contains the struct called [`Memory`] which acts as the
//! virtual machine for running Brainfuck code. It has a tape which acts as
//! the storage chute for [`crate::vm::MemoryCell`]s and can be customized
//! with [`MemoryOptions`].

use std::{
    io::{Error, ErrorKind}
};

use crate::{
    code::CommandRunner,
    debug::{Event, OkEvent, Status},
    unwrapcell,
};
use super::{
    CellNumber,
    default_lower,
    default_range,
    default_upper,
    MemoryCell,
    MemoryRange,
    MemoryPointer,
};

/// Customization for [`Memory`].
#[derive(Clone, Copy, Debug)]
pub struct MemoryOptions {
    variable_length: bool,
    wrap: bool,
    lower_bound: CellNumber,
    upper_bound: CellNumber,
    initial_length: MemoryRange,
}

impl MemoryOptions {
    /// Creates a new set of [`MemoryOptions`] with default values.
    pub fn new() -> Self {
        return Self {
            variable_length: false,
            wrap: true,
            lower_bound: default_lower(),
            upper_bound: default_upper(),
            initial_length: default_range(),
        };
    }

    /// Sets whether the memory tape should increase in size when necessary.
    /// By default, this field is `false`, which means that the pointer to
    /// the tape wraps back to 0 when it reaches the end of the tape.
    pub fn variable_length(&mut self, setting: bool) -> &mut Self {
        self.variable_length = setting;
        return self;
    }

    /// Sets whether each memory cell should wrap around if it is not within
    /// lower_bound and upper_bound. I haven't implemented the functionality
    /// for when `wrap` is `false` so this method is currently private.
    #[allow(dead_code)]
    fn wrap(&mut self, setting: bool) -> &mut Self {
        self.wrap = setting;
        return self;
    }

    /// Sets the lower bound of a [`MemoryCell`].
    pub fn lower_bound(&mut self, setting: CellNumber) -> &mut Self {
        self.lower_bound = setting;
        return self;
    }

    /// Gets the lower bound of a [`MemoryCell`].
    pub fn lowest(&self) -> CellNumber {
        return self.lower_bound;
    }

    /// Sets the upper bound of a [`MemoryCell`].
    pub fn upper_bound(&mut self, setting: CellNumber) -> &mut Self {
        self.upper_bound = setting;
        return self;
    }

    /// Gets the modulo limit of a [`MemoryCell`].
    /// 
    /// This modulo limit is the number used for wrapping and is 1 higher than
    /// the actual highest value of a [`MemoryCell`].
    pub fn highest(&self) -> CellNumber {
        return self.upper_bound+1;
    }

    /// Sets the initial length of the [`Memory`] tape.
    pub fn initial_length(&mut self, setting: MemoryRange) -> &mut Self {
        self.initial_length = setting;
        return self;
    }

    /// Checks if the set of options here is valid.
    pub fn is_valid(&self) -> bool {
        if ((self.upper_bound - self.lower_bound) <= 0 ||
            self.initial_length < 1) &&
            self.upper_bound != CellNumber::MAX
        {
            return false;
        }
        return true;
    }

    /// Validates the set of options and panics if it is not valid.
    pub fn validate(&self) {
        if !self.is_valid() {
            panic!("This set of MemoryOptions is invalid: {:?}", self);
        }
    }

    /// Generates an instance of [`Memory`], if the set of options provides
    /// is invalid, [`std::io::Error`] is returned instead.
    pub fn generate(&self) -> Result<Memory, Error> {
        return match self.is_valid() {
            false => Err(Error::new(
                ErrorKind::InvalidData,
                format!("This set of MemoryOptions is invalid: {:?}", self)
            )),
            true => Ok(Memory::new(self.clone()))
        };
    }

    /// Assume that the set of options is valid and panics if it is not.
    pub fn assume_and_generate(&self) -> Memory {
        self.validate();
        let copy = self.clone();
        return Memory::new(copy);
    }
}

impl Default for MemoryOptions {
    /// Creates a default set of options.
    fn default() -> Self {
        return Self::new();
    }
}

impl Into<Memory> for MemoryOptions {
    /// Generates a new [`Memory`].
    fn into(self) -> Memory {
        return self.assume_and_generate();
    }
}

/// Wrapper type around a vector of [`crate::vm::MemoryCell`]s stored on the
/// heap.
type Tape = Box<Vec<MemoryCell>>;

/// A virtual machine with a memory [`Tape`], a [`MemoryPointer`] and some
/// [`MemoryOptions`].
#[derive(Clone, Debug)]
pub struct Memory {
    tape: Tape,
    pointer: MemoryPointer,
    options: MemoryOptions,
}

impl Memory {
    /// Creates a new instance of [`Memory`].
    pub fn new(options: MemoryOptions) -> Self {
        options.validate();
        let tape: Tape = Box::new(Vec::new());
        let pointer = MemoryPointer::default();
        let mut memory = Self {tape, pointer, options};
        memory.init();
        return memory;
    }

    /// Same as new but validates the set of options without panicking.
    pub fn with_validation(options: MemoryOptions) -> Result<Self, Error> {
        return match options.is_valid() {
            false => Err(Error::new(
                ErrorKind::InvalidData,
                format!("This set of MemoryOptions is invalid: {:?}", options)
            )),
            true => Ok(Memory::new(options)),
        };
    }

    /// Initializes the memory tape.
    fn init(&mut self) {
        self.tape.resize(
            self.options.initial_length,
            MemoryCell::new(self.options.lowest())
        );
        self.pointer.to_zero();
    }

    /// Flattens the value of all cells in the tape to the lower bound defined
    /// in `options`.
    pub fn flatten(&mut self) {
        for cell in &mut self.tape.iter_mut() {
            cell.flatten(self.options.lowest());
        }
    }

    /// Get the index the pointer is pointing at.
    pub fn pointer(&self) -> MemoryRange {
        return self.pointer.pointer();
    }

    /// Get a copy of the [`MemoryCell`] indexed by the pointer.
    pub fn get(&self) -> Option<MemoryCell> {
        return match self.tape.get(self.pointer()) {
            Some(mc) => Some(mc.clone()),
            None => None,
        };
    }
}

impl CommandRunner for Memory {
    fn increment(&mut self) -> Event {
        let pointer = self.pointer();
        let cell = self.tape
            .get_mut(pointer);
        let cell = unwrapcell!(cell);
        let _ = cell.increment(self.options.lowest(), self.options.highest());
        return Ok(OkEvent::Status(Status::new("Increment successful")));
    }

    fn decrement(&mut self) -> Event {
        let pointer = self.pointer();
        let cell = self.tape
            .get_mut(pointer);
        let cell = unwrapcell!(cell);
        let _ = cell.decrement(self.options.lowest(), self.options.highest());
        return Ok(OkEvent::Status(Status::new("Decrement successful")));
    }

    fn next(&mut self) -> Event {
        let resize = self.pointer
            .increment(self.tape.len(), !self.options.variable_length);
        if resize {
            self.tape.push(MemoryCell::new(self.options.lowest()));
        }
        return Ok(OkEvent::Status(Status::new(
            "Successfully moved pointer to the next cell."
        )));
    }

    fn previous(&mut self) -> Event {
        self.pointer
            .decrement(self.tape.len());
        return Ok(OkEvent::Status(Status::new(
            "Successfully moved pointer to the previous cell."
        )));
    }

    fn read_out(&self, output: &mut char) -> Event {
        let pointer = self.pointer();
        let cell = self.tape
            .get(pointer);
        let cell = unwrapcell!(cell);
        return cell.to_char(output);
    }

    fn write_in(&mut self, input: char) -> Event {
        let pointer = self.pointer();
        let cell = self.tape
            .get_mut(pointer);
        let cell = unwrapcell!(cell);
        return cell.from_char(
            input,
            self.options.lowest(),
            self.options.highest()
        );
    }

    fn is_zero(&self, output: &mut bool) -> Event {
        let pointer = self.pointer();
        let cell = self.tape
            .get(pointer);
        let cell = unwrapcell!(cell);
        *output = cell.number() == 0;
        return Ok(OkEvent::Status(Status::new(
            "Successfully moved pointer to the previous cell."
        )));
    }
}

impl From<&MemoryOptions> for Memory {
    /// Creates a [`Memory`] instance from [`MemoryOptions`].
    fn from(options: &MemoryOptions) -> Self {
        return options.assume_and_generate();
    }
}