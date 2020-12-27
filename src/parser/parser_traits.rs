//use pest::{Parser, RuleType};
use std::{
    fs::read_to_string,
    path::Path
};

use crate::{
    code::BfProgram,
    debug::{Event, ErrEvent, BfError, BfErrorKind},
};

pub trait BfParser//<R>
//where
//     Self: Parser<R>,
//     R: RuleType
{
    fn parse_string<S>(script: S, program: &mut BfProgram) -> Event
    where
        S: AsRef<str>;

    fn parse_file<P>(file_path: P, program: &mut BfProgram) -> Event
    where
        P: AsRef<Path>
    {
        return Self::parse_string(match read_to_string(&file_path) {
            Ok(s) => s,
            Err(error) => return Err(ErrEvent::Error(BfError::new(
                BfErrorKind::Other,
                format!(
                    "Could not open file with path: {:?}\n\
                    Error by `read_to_string`: {}",
                    file_path.as_ref(),
                    error
                )
            ))),
        }, program);
    }
}