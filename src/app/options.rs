use clap::ArgMatches;
use std::{
    io::{Error, ErrorKind},
    path::PathBuf
};

use crate::vm::MemoryOptions;

macro_rules! unwrapparse {
    ($result: expr) => {
        {
            match $result {
                Ok(p) => p,
                Err(error) => return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("{}", error)
                ))
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum ProgramOptions {
    Path(PathBuf),
    Raw(String),
}

#[derive(Clone, Debug)]
pub struct AppOptions {
    pub program: ProgramOptions,
    pub verbosity: u8,
    pub memory_options: MemoryOptions,
    pub new_parser: bool,
}

impl AppOptions {
    pub fn from_matches(matches: &ArgMatches) -> Result<Self, Error> {
        let program = if let Some(thing) = matches.value_of("input") {
            ProgramOptions::Path(PathBuf::from(thing))
        } else if let Some(thing) = matches.value_of("raw") {
            ProgramOptions::Raw(thing.to_string())
        } else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No input program detected."
            ));
        };
        let verbosity = matches.occurrences_of("verbose") as u8;
        let mut memory_options = MemoryOptions::default();
        if let Some(thing) = matches.value_of("celllower") {
            memory_options.lower_bound(
                unwrapparse!(thing.parse())
            );
        }
        if let Some(thing) = matches.value_of("cellupper") {
            memory_options.upper_bound(
                unwrapparse!(thing.parse())
            );
        }
        if let Some(thing) = matches.value_of("memorysize") {
            memory_options.initial_length(
                unwrapparse!(thing.parse())
            );
        }
        if let Some(thing) = matches.value_of("variablelength") {
            memory_options.variable_length(
                unwrapparse!(thing.parse())
            );
        }
        let new_parser = matches.is_present("newparser");
        return Ok(Self {program, verbosity, memory_options, new_parser});
    }

    pub fn with_verbosity<F>(&self, minimum: u8, callback: F)
    where
        F: FnOnce(&Self) -> ()
    {
        if self.verbosity >= minimum {
            callback(self);
        }
    }
}