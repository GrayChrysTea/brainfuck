//! [`brainfucklib::debug`]
//!
//! This module provides debugging assets to log and monitor your Brainfuck
//! program and virtual machine.

pub mod debugger;
pub mod event;

pub use self::{
    debugger::{
        BfDebugger,
        Debugger,
        ErrorLog,
    },
    event::*,
};

/// This type is a wrapper around [`bool`] and is used as a result type for
/// an action. If [`CurrentStatus`] is `true`, then the action has been
/// successful, otherwise, an error has occurred.
/// 
/// Due to the nature of [`CurrentStatus`], you will not be able to detect
/// warnings or what kind of errors have occurred if any at all.
pub type CurrentStatus = bool;