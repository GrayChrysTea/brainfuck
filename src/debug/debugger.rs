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

impl ErrorLog {
    /// Creates a new [`ErrorLog`].
    pub fn new() -> Self {
        return Self(Vec::new());
    }

    /// Push an [`ErrEvent`] into the log with its corresponding location.
    pub fn push(&mut self, location: usize, error: ErrEvent) {
        self.0.push((location, error));
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

impl AsMut<Vec<(usize, ErrEvent)>> for ErrorLog {
    /// Get the inner [`Vec`]tor in the [`ErrorLog`].
    fn as_mut(&mut self) -> &mut Vec<(usize, ErrEvent)> {
        return &mut self.0;
    }
}

/// [`Debugger`] serves as an interface for adding and reading logs to a
/// struct responsible for debugging by requiring that the struct exposes the
/// [`Log`] it uses to store events via [`AsMut<Log>`].
pub trait Debugger {
    /// Clear the log.
    fn clear(&mut self);

    /// Pushes an event into the log.
    fn push(&mut self, event: Event);

    /// Get the number of events logged.
    fn total_events(&self) -> usize;

    /// Get the number of [`crate::debug::OkEvent`]s.
    fn total_ok(&self) -> usize;

    /// Get the number of [`ErrEvent`]s.
    fn total_err(&self) -> usize {
        return self.total_events() - self.total_ok();
    }

    /// Write all of the [`ErrEvent`]s stored into an [`ErrorLog`].
    fn to_err_log(&self, error_log: &mut ErrorLog);

    /// Get the last [`Event`] in the log.
    fn last_event(&self) -> Option<&Event>;

    /// Checks if the last [`Event`] in the log is an error. If the log is
    /// empty, `false` is returned.
    fn is_err(&self) -> bool {
        return !self.is_ok();
    }

    /// Checks if the last [`Event`] in the log is [`Ok`]. If the log is empty,
    /// `false` is returned.
    fn is_ok(&self) -> bool;

    /// Checks if there are no [`ErrEvent`]s in the log.
    fn all_ok(&self) -> bool {
        return self.total_events() == self.total_ok()
    }
}

/// [`BfDebugger`] stores a log of [`Event`]s as a [`Vec`]tor on the heap.
#[derive(Debug)]
pub struct BfDebugger {
    log: Box<Log>,
    max_length: usize,
    ok_before: usize,
    err_before: usize,
}

impl BfDebugger {
    /// Creates a new instance of [`BfDebugger`].
    pub fn new() -> Self {
        let log: Box<Log> = Box::new(Vec::new());
        let max_length = 5;
        let ok_before = 0;
        let err_before = 0;
        return Self {log, max_length, ok_before, err_before};
    }

    /// Set the maximum length of the logger.
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        let max_length = match max_length {
            0 => 1,
            n => n
        };
        self.max_length = max_length;
        return self;
    }

    /// Removes events if it exceeds `max_length`.
    pub fn castrate(&mut self) -> usize {
        let log_len = self.log.len();
        if log_len > self.max_length {
            let mut ok_removed: usize = 0;
            let mut err_removed: usize = 0;
            for _i in self.max_length..log_len {
                match self.log.remove(0) {
                    Ok(_) => ok_removed += 1,
                    Err(_) => err_removed += 1,
                }
            }
            self.ok_before += ok_removed;
            self.err_before += err_removed;
            return log_len - self.max_length;
        } else {
            return 0;
        }
    }
}

impl AsMut<Log> for BfDebugger {
    fn as_mut(&mut self) -> &mut Log {
        return &mut *self.log;
    }
}

impl Debugger for BfDebugger {
    fn clear(&mut self) {
        self.log.clear();
    }

    fn push(&mut self, event: Event) {
        self.log.push(event);
        self.castrate();
    }

    fn total_events(&self) -> usize {
        return self.ok_before + self.err_before + self.log.len();
    }

    fn total_ok(&self) -> usize {
        let mut ok: usize = 0;
        for event in self.log.iter() {
            if event.is_ok() {
                ok += 1;
            }
        }
        return ok + self.ok_before;
    }

    fn to_err_log(&self, error_log: &mut ErrorLog) {
        let total_events = self.total_events();
        for (index, event) in self.log.iter().enumerate() {
            let error = match event {
                Ok(_) => continue,
                Err(e) => e.clone()
            };
            error_log.push(index + total_events, error);
        }
    }

    fn last_event(&self) -> Option<&Event> {
        return self.log.last();
    }

    fn is_ok(&self) -> bool {
        return match self.log.last() {
            Some(event) => event.is_ok(),
            None => false,
        };
    }

    fn is_err(&self) -> bool {
        return match self.log.last() {
            Some(event) => event.is_err(),
            None => false,
        };
    }
}