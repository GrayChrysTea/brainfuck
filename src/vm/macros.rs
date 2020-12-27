//! [`brainfucklib::vm::macros`]
//! 
//! This module defines macros used by the virtual machine.

/// This macro unwraps an [`Option`] containing a [`crate::vm::MemoryCell`]
/// from a pointer. If the option is [`None`], a [`crate::debug::ErrEvent`] is
/// returned.
#[macro_export]
macro_rules! unwrapcell {
    ($cell_opt: expr) => {
        {
            match $cell_opt {
                Some(c) => c,
                None => return Err($crate::debug::ErrEvent::Error(
                    $crate::debug::BfError::new(
                        $crate::debug::BfErrorKind::OutOfBounds,
                        format!("Could not get cell with pointer")
                    )
                ))
            }
        }
    };
}