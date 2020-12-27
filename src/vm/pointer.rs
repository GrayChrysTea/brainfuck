//! [`brainfucklib::vm::pointer`]
//! 
//! This module defines [`MemoryPointer`], which is a special wrapper struct
//! around [`usize`]. Unlike a normal [`usize`], you can check if the pointer
//! is within the range of the length of the memory tape in a Brainfuck
//! virtual machine.

/// Wrapper around [`usize`].
pub type MemoryRange = usize;

/// Default maximum length of a memory tape in a Brainfuck virtual machine.
pub fn default_range() -> MemoryRange {
    return 0xFFFF;
}

/// A pointer which stores the location of the [`crate::vm::MemoryCell`]
/// it is pointing to.
/// 
/// # Fields
/// 1. `pointer`: [`MemoryRange`]
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct MemoryPointer {
    pointer: MemoryRange,
}

impl MemoryPointer {
    /// Creates a new [`MemoryPointer`] pointing to cell 0.
    pub fn new() -> Self {
        return Self {pointer: 0};
    }

    /// Gets the index the [`MemoryPointer`] is pointing at.
    pub fn pointer(&self) -> MemoryRange {
        return self.pointer;
    }

    /// Gets the mutable reference of the index stored in the
    /// [`MemoryPointer`].
    fn pointer_mut(&mut self) -> &mut MemoryRange {
        return &mut self.pointer;
    }

    /// Increments the pointer and checks to see if it exceeds the length
    /// of the memory tape (passed in as `highest`). If it does exceed, this
    /// method will check if the virtual machine wraps the pointer back
    /// around to 0 or simply increases in size. This option is passed in as
    /// `wrap`.
    /// 
    /// By default this method returns a [`bool`]ean value of false. This
    /// tells the virtual machine not to increase in size. However, if `wrap`
    /// is `false` and the pointer exceeds the length of the tape, this
    /// method returns `true`, telling the virtual machine to increase in
    /// length.
    pub fn increment(&mut self, highest: MemoryRange, wrap: bool) -> bool {
        *self.pointer_mut() += 1;
        if self.pointer() >= highest {
            if wrap {
                *self.pointer_mut() = 0;
            } else {
                *self.pointer_mut() = highest;
                return true;
            }
        }
        return false;
    }

    /// Decrements the pointer. If the pointer is already at 0, it wraps
    /// around to `highest` (the length of the tape), unlike
    /// [`MemoryPointer::increment`] which allows you to increase the size of
    /// the tape when necessary. This is so that the tape does not have to
    /// work with negative indices.
    pub fn decrement(&mut self, highest: MemoryRange) {
        if highest <= 0 {
            panic!("MemoryPointer::decrement: highest cannot be 0 or below.");
        }
        let pointer = self.pointer();
        if pointer <= 0 || pointer >= highest {
            *self.pointer_mut() = highest - 1;
        } else {
            *self.pointer_mut() -= 1;
        }
    }

    /// Resets the pointer to 0.
    pub fn to_zero(&mut self) {
        *self.pointer_mut() = 0;
    }
}

impl Default for MemoryPointer {
    /// Creates a [`MemoryPointer`].
    fn default() -> Self {
        return Self::new();
    }
}