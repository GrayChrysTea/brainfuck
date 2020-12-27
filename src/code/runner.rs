use std::{
    cell::RefCell,
    io::{stdin, stdout, prelude::*},
    iter::Iterator,
    rc::Rc,
};
use crate::{
    debug::{Debugger, BfDebugger, ErrEvent, BfError, BfErrorKind},
    vm::Memory
};
use super::{BfCommand, BfProgram, CommandRunner};

#[derive(Debug)]
pub struct BfRunner<D = BfDebugger, M = Memory>
where
    D: Debugger,
    M: CommandRunner,
{
    program: Box<BfProgram>,
    program_pointer: usize,
    memory: Box<M>,
    debugger: Option<Rc<RefCell<D>>>,
}

impl<D, M> BfRunner<D, M>
where
    D: Debugger,
    M: CommandRunner,
{
    pub fn new(
        program: Box<BfProgram>,
        memory: Box<M>,
        debugger: Option<Rc<RefCell<D>>>,
    ) -> Self {
        let program_pointer = 0;
        return Self {program, program_pointer, memory, debugger};
    }

    pub fn run_once(&mut self) -> Option<()> {
        use BfCommand::*;
        let token = self.program
            .current_command(self.program_pointer)?;
        //let span = command.span();
        let command = token.command();
        let event = match command {
            Increment => self.memory.increment(),
            Decrement => self.memory.decrement(),
            Previous => self.memory.previous(),
            Next => self.memory.next(),
            Read => {
                let mut output: char = 'f';
                let event = self.memory.read_out(&mut output);
                print!("{}", output);
                let _ = stdout().flush();
                event
            },
            Write => {
                let mut input = String::new();
                let _ = stdout().flush();
                let inres = stdin().read_line(&mut input);
                if let Err(error) = inres {
                    Err(ErrEvent::Error(BfError::new(
                        BfErrorKind::Other,
                        format!("Could not read user input.
                        stdin Error: {}", error)
                    )))
                } else {
                    self.memory.write_in(
                        input.chars()
                            .collect::<Vec<char>>()
                            .get(0)
                            .unwrap()
                            .clone()
                    )
                }
            },
            IfZero => {
                let mut is_zero = false;
                let event = self.memory.is_zero(&mut is_zero);
                match event {
                    Err(_) => event,
                    Ok(_) => {
                        let counterpart = self.program.get_counterpart(
                            self.program_pointer
                        );
                        match counterpart {
                            Some(counterpart) => {
                                if is_zero {
                                    self.program_pointer = counterpart;
                                }
                                event
                            },
                            None => Err(ErrEvent::Error(BfError::new(
                                BfErrorKind::UnmatchedLeftBracket,
                                format!(
                                    "Could not get matching right bracket for \
                                    {}",
                                    token.span().start()
                                )
                            )))
                        }
                    }
                }
            },
            IfNotZero => {
                let mut is_zero = false;
                let event = self.memory.is_zero(&mut is_zero);
                match event {
                    Err(_) => event,
                    Ok(_) => {
                        let counterpart = self.program.get_counterpart(
                            self.program_pointer
                        );
                        match counterpart {
                            Some(counterpart) => {
                                if !is_zero {
                                    self.program_pointer = counterpart;
                                }
                                event
                            },
                            None => Err(ErrEvent::Error(BfError::new(
                                BfErrorKind::UnmatchedRightBracket,
                                format!(
                                    "Could not get matching left bracket for \
                                    {}",
                                    token.span().start()
                                )
                            )))
                        }
                    }
                }
            }
        };
        self.program_pointer += 1;
        let output = event.is_ok();
        if let Some(ref debugger) = self.debugger {
            debugger.borrow_mut().push(event);
        }
        return match output {
            true => Some(()),
            false => None,
        };
    }

    pub fn get_debugger(&mut self) -> Option<Rc<RefCell<D>>> {
        return match self.debugger {
            Some(ref mut d) => Some(Rc::clone(d)),
            None => None,
        };
    }
}

impl<D> BfRunner<D, Memory>
where
    D: Debugger,
{
    pub fn print_pointer(&self) {
        println!("{:?}", self.memory.pointer());
    }

    pub fn print_cell(&self) {
        println!("{:?}", self.memory.get());
    }
}

impl<D, M> Iterator for BfRunner<D, M>
where
    D: Debugger,
    M: CommandRunner,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        return self.run_once();
    }
}