//! [`brainfucklib::code::command_traits`]
//! 
//! This module defines traits for virtual machines which run Brainfuck
//! commands.

use crate::debug::Event;

/// A [`CommandRunner`] can run Brainfuck commands.
pub trait CommandRunner {
    /// Increment the memory cell.
    fn increment(&mut self) -> Event;
    
    /// Decrement the memory cell.
    fn decrement(&mut self) -> Event;

    /// Move the pointer to the left.
    fn previous(&mut self) -> Event;

    /// Move the pointer to the right.
    fn next(&mut self) -> Event;

    /// Read out the memory cell as a character.
    fn read_out(&self, output: &mut char) -> Event;

    /// Write a character into the memory cell.
    fn write_in(&mut self, input: char) -> Event;

    /// Checks if the current memory cell is zero.
    fn is_zero(&self, output: &mut bool) -> Event;

    /// Checks if the current memory cell is not zero.
    /// 
    /// This method has a default implementation because a cell which returns
    /// a `false` for [`CommandRunner::is_zero`] is by default not zero.
    fn is_not_zero(&self, output: &mut bool) -> Event {
        let mut out = false;
        let event = self.is_zero(&mut out);
        *output = !out;
        return event;
    }
}