use clap::{App, Arg};

pub fn get_app<'a, S>(name: S) -> App<'a, 'a>
where
    S: AsRef<str>
{
    let app = App::new(name.as_ref())
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .multiple(true)
                .help("How verbose the output should be.")
                .long_help(
                    "By default, the program only outputs what the program \
                    tells the virtual machine to output.
                    
                    However, you can show debugging information by increasing \
                    the number of occurrences of the verbosity flag."
                )
        )
        .arg(
            Arg::with_name("input")
                .takes_value(true)
                .multiple(false)
                .required_unless("raw")
                .help("The input program.")
                .long_help("The input program to be run. This input can be \
                an absolute or a relative path. If you want to input a raw \
                program (by inputting the full program that is not stored in \
                a file), use `-r`.")
        )
        .arg(
            Arg::with_name("raw")
                .short("r")
                .long("raw")
                .value_name("RAW-PROGRAM")
                .takes_value(true)
                .multiple(false)
                .required_unless("input")
                .help("The input program as a raw string in the command \
                line.")
                .long_help("The input program that is not stored as a file. \
                Instead it is a raw string that you can pass to the \
                interpreter to run.")
        )
        .arg(
            Arg::with_name("celllower")
                .short("c")
                .long("cell-lower")
                .value_name("CELL-LOWER")
                .takes_value(true)
                .multiple(false)
                .help("The lower bound of a cell.")
                .long_help("By default, the lower bound of each memory cell \
                is 0. However, you can edit this by using this option.
                
                An error is returned if this option is greater than \
                cellupper or if a non-integer is entered.")
        )
        .arg(
            Arg::with_name("cellupper")
                .short("C")
                .long("cell-upper")
                .value_name("CELL-UPPER")
                .takes_value(true)
                .multiple(false)
                .help("The upper bound of a cell.")
                .long_help("By default, the upper bound of each memory cell \
                is 256. However, you can edit this by using this option.
                
                An error is returned if this option is smaller than \
                celllower or if a non-integer is entered.")
        )
        .arg(
            Arg::with_name("memorysize")
                .short("m")
                .long("memory")
                .value_name("MEMORY-SIZE")
                .takes_value(true)
                .multiple(false)
                .help("The number of memory cell in the memory tape.")
                .long_help("By default, the lower bound of each memory cell \
                is 65535. However, you can edit this by using this option.
                
                An error is returned if this option is less than 1 or \
                a non-integer is passed.")
        )
        .arg(
            Arg::with_name("variablelength")
                .short("l")
                .long("variable-length")
                .value_name("VARIABLE-LENGTH")
                .takes_value(false)
                .multiple(false)
                .help("Whether the memory tape can increase in length.")
                .long_help("By default, the memory tape cannot increase in \
                length. When the memory pointer reaches the end of the tape \
                but is commanded to go to the next cell, it wraps around to \
                the beginning.
                
                If this flag is present, then the memory tape expands in \
                size when the pointer reaches the end of the tape.
                
                However, no matter the presence of this flag, when the \
                pointer is commanded to go to the previous cell when it is \
                at cell 0, it will still wrap to the last cell.")
        )
        .arg(
            Arg::with_name("newparser")
                .short("N")
                .long("new-parser")
                .takes_value(false)
                .multiple(false)
                .help("Whether to use the new parser.")
                .long_help("This implementation of brainfuck has 2 parsers. \
                The first one is the default brainfuck parser that should \
                work with any of the programs you see on `Wikipedia`.
                
                The second parser allows you to comment any part of a line \
                after `#`. This might break code if you use punctuation \
                in your comments.")
        )
    ;
    return app;
}