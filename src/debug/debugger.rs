//! [`brainfucklib::debug::debugger`]
//! 
//! This module defines a trait called [`Debugger`], which provides functions
//! to log events such as statuses and errors.

//use std::ops::Deref;
use super::{Event, ErrEvent};

/// A [`Log`] of [`Event`]s.
type Log = Vec<Event>;

/// This struct stores a [`Vec`]tor of [`ErrEvent`]s and the location of where
/// it occurred in a debugging log.
pub struct ErrorLog(Vec<(usize, ErrEvent)>);

impl AsMut<Vec<(usize, ErrEvent)>> for ErrorLog {
    /// Get the inner [`Vec`]tor in the [`ErrorLog`].
    fn as_mut(&mut self) -> &mut Vec<(usize, ErrEvent)> {
        return &mut self.0;
    }
}

/// [`Debugger`] serves as an interface for adding and reading logs to a
/// struct responsible for debugging by requiring that the struct exposes the
/// [`Log`] it uses to store events via [`AsMut<Log>`].
pub trait Debugger where Self: AsMut<Log> {
    /// Clear the log.
    fn clear(&mut self) {
        self.as_mut().clear();
    }

    /// Pushes an event into the log.
    fn push(&mut self, event: Event) {
        self.as_mut().push(event);
    }

    /// Get the number of events logged.
    fn total_events(&mut self) -> usize {
        return self.as_mut().len();
    }

    /// Get the number of [`crate::debug::OkEvent`]s.
    fn total_ok(&mut self) -> usize {
        let mut total: usize = 0;
        for event in self.as_mut().iter() {
            if event.is_ok() {
                total += 1;
            }
        }
        return total;
    }

    /// Get the number of [`ErrEvent`]s.
    fn total_err(&mut self) -> usize {
        return self.total_events() - self.total_ok();
    }

    /// Copy all [`ErrEvent`]s to an [`ErrorLog`].
    fn to_err_log(&mut self) -> ErrorLog {
        let mut log = ErrorLog(Vec::new());
        for (index, event) in self.as_mut().iter().enumerate() {
            if let Err(error) = event {
                log.as_mut().push((index, error.clone()));
            }
        }
        return log;
    }

    /// Get the last [`Event`] in the log.
    fn last_event(&mut self) -> Option<&Event> {
        let length = self.as_mut().len();
        if length == 0 {
            return None;
        }
        return self.as_mut().get(length-1);
    }

    /// Checks if the last [`Event`] in the log is an error. If the log is
    /// empty, `false` is returned.
    fn is_err(&mut self) -> bool {
        return match self.last_event() {
            Some(event) => event.is_err(),
            None => false,
        };
    }

    /// Checks if the last [`Event`] in the log is [`Ok`]. If the log is empty,
    /// `false` is returned.
    fn is_ok(&mut self) -> bool {
        return match self.last_event() {
            Some(event) => event.is_ok(),
            None => false,
        };
    }

    /// Checks if there are no [`ErrEvent`]s in the log.
    fn all_ok(&mut self) -> bool {
        for event in self.as_mut().iter() {
            if event.is_err() {
                return false;
            }
        }
        return true;
    }
}

/// [`BfDebugger`] stores a log of [`Event`]s as a [`Vec`]tor on the heap.
#[derive(Debug)]
pub struct BfDebugger {
    log: Box<Log>,
}

impl BfDebugger {
    /// Creates a new instance of [`BfDebugger`].
    pub fn new() -> Self {
        let log: Box<Log> = Box::new(Vec::new());
        return Self {log};
    }
}

impl AsMut<Log> for BfDebugger {
    fn as_mut(&mut self) -> &mut Log {
        return &mut *self.log;
    }
}

impl Debugger for BfDebugger {}