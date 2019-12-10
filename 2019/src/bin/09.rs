use aoc::{intcode::Program, Input};
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let in_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(in_file)?;

    let code = input.to_string().unwrap();
    let mut program = Program::from(code.as_str());

    // Part 1.
    let (_mem, out) = program.input(1).execute()?;
    for line in out {
        println!("{}", line);
    }

    // Part 2.
    let (_mem, out) = program.input(2).execute()?;
    for line in out {
        println!("{}", line);
    }

    Ok(())
}
