//! [`brainfucklib`]
//! 
//! This library allows you to run Brainfuck programs using the grammar parser
//! and virtual machine provided.

extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod app;
pub mod code;
pub mod debug;
pub mod macros;
pub mod parser;
pub mod vm;