//! [`brainfucklib::parser`]
//! 
//! This module allows you to parse Brainfuck programs. There are 2 parsers
//! that you can use from this library. They are:
//! 1. [`nparser`], and
//! 2. [`sparser`]
//! 
//! [`sparser`] is the simple parser. All it does is read all the characters
//! which represents Brainfuck commands and ignores everything else. This is
//! faster but it does not allow for commenting out code and putting snippets
//! aside.
//! 
//! To avoid this problem, you can use [`nparser`], which allows you to
//! comment any part of a line after `#`, allowing the parser to ignore
//! Brainfuck code after that token.

pub mod nparser;
pub mod parser_traits;
pub mod sparser;

pub use self::{
    nparser::NewParser,
    sparser::NormalParser,
    parser_traits::BfParser,
};