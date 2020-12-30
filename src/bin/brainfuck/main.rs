use brainfucklib::{
    app::{
        get_app,
        AppOptions,
        ProgramOptions,
    },
    code::{BfProgram, BfRunner},
    debug::{Debugger, BfDebugger},
    parser::{NewParser, NormalParser, BfParser},
    vm::Memory,
};

use std::{
    cell::RefCell,
    io::{Error},
    process::exit,
    rc::Rc,
};

const VERSION_NO: &'static str = "0.3.0a";

macro_rules! someerror {
    ($last: expr, $code: expr) => {
        {
            if let Some(event) = $last {
                if let Err(error) = event {
                    return Some(($code, std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{}", error)
                    )));
                }
            }
        }
    }
}

fn run() -> Option<(i32, Error)> {
    let mut app = get_app("Brainfuck");
    app = app
        .version(VERSION_NO)
        .about("Brainfuck interpreter.")
        .author("GrayChrysTea <gray.chrysanthemum@gmail.com>")
    ;
    let matches = app.get_matches();
    let options = match AppOptions::from_matches(&matches) {
        Ok(o) => o,
        Err(error) => return Some((1, error)),
    };
    options.with_verbosity(1, |options| {
        println!("Processing arguments.");
        println!("Verbosity: {}", options.verbosity);
        println!("Input type: {:?}", options.program);
        println!("Memory Options: {:#?}", options.memory_options);
        println!("Use new parser: {}", options.new_parser);
    });
    let debugger = Rc::new(RefCell::new(BfDebugger::new()));
    let mut program = Box::new(BfProgram::new());
    let event = match options.program {
        ProgramOptions::Path(ref path) => {
            if options.new_parser {
                NewParser::parse_file(path, &mut program)
            } else {
                NormalParser::parse_file(path, &mut program)
            }
        },
        ProgramOptions::Raw(ref prog) => {
            if options.new_parser {
                NewParser::parse_string(prog, &mut program)
            } else {
                NormalParser::parse_string(prog, &mut program)
            }
        }
    };
    options.with_verbosity(1, |_options| {
        println!("Parsing ok.");
    });
    options.with_verbosity(3, {
        let program = &program;
        move |_options| {
            println!("Parsed program:");
            println!("{:#?}", program);
        }
    });
    debugger.borrow_mut().push(event);
    someerror!(debugger.borrow_mut().last_event(), 2);
    options.with_verbosity(1, |_options| {
        println!("Trying to populate bracket map.");
    });
    debugger.borrow_mut().push(program.populate_map());
    someerror!(debugger.borrow_mut().last_event(), 3);
    options.with_verbosity(1, |_options| {
        println!("Trying to calculate bracket map.");
    });
    debugger.borrow_mut().push(program.calculate_map());
    someerror!(debugger.borrow_mut().last_event(), 4);

    let memory: Box<Memory> = Box::new(options.memory_options.clone().into());
    let mut runner = BfRunner::new(
        program,
        memory,
        Some(Rc::clone(&debugger)),
    );

    options.with_verbosity(1, |_options| {
        println!("Runner created.");
    });

    while let Some(_) = runner.run_once() {
        options.with_verbosity(2, |_options| {
            println!("Running once.");
        });
        if let Some(event) = debugger.borrow_mut().last_event() {
            options.with_verbosity(2, |_options| {
                println!("\n{:?}", event);
            });
        }
        options.with_verbosity(3, {
            let runner = &runner;
            move |_options| {
                runner.print_pointer();
                runner.print_cell();
            }
        });
    }

    println!();
    options.with_verbosity(1, |_options| {
        println!("All OK.");
    });
    return None;
}

fn main() {
    let code = match run() {
        Some((code, error)) => {
            println!("{}\nExited with code: {}", error, code);
            code
        },
        None => 0
    };
    exit(code);
}