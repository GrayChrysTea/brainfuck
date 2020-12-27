//! [`brainfucklib::debug::event`]
//! 
//! This module defines various debugging events, such as [`Status`]es,
//! [`Warning`]s and [`BfError`]s. These can be passed into a custom [`Result`]
//! called [`Event`] which is logged into a [`crate::debug::BfDebugger`].

pub mod error;
pub mod event;
pub mod status;
pub mod warning;

pub use self::{
    error::{BfError, BfErrorKind},
    event::{Event, OkEvent, ErrEvent},
    status::Status,
    warning::Warning,
};

/// Custom [`Result`] for certain operations used by [`brainfucklib`].
pub type BfResult<T> = Result<T, self::error::BfError>;
