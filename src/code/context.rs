//! [`brainfucklib::code::context`]
//! 
//! This module provides implementations to give Brainfuck code a context.

/// A [`Span`] represents a range of locations a segment of bytes occur,
/// starting from `start` and ending at but not including `end`.
#[derive(Clone, Copy, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Creates a new [`Span`].
    pub fn new(start: usize, end: usize) -> Self {
        return Self {start, end};
    }

    /// Get the starting location of the segment of bytes.
    pub fn start(&self) -> usize {
        return self.start;
    }

    /// Get the ending location of the segment of bytes. (The last byte
    /// actually comes before this)
    pub fn end(&self) -> usize {
        return self.end;
    }
}

impl From<pest::Span<'_>> for Span {
    fn from(span: pest::Span) -> Self {
        return Self::new(span.start(), span.end());
    }
}