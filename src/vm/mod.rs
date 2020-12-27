//! [`brainfucklib::vm`]
//! 
//! This module provides definition for a Brainfuck virtual machine. Amongst
//! the implementations defined here is a [`memory::Memory`], which allows you
//! to store memory in a Brainfuck program. You can customise it using
//! [`memory::MemoryOptions`] as well.

pub mod cell;
pub mod macros;
pub mod memory;
pub mod pointer;

pub use self::{
    cell::{CellNumber, default_lower, default_upper, MemoryCell},
    memory::{Memory, MemoryOptions},
    pointer::{MemoryRange, default_range, MemoryPointer}
};