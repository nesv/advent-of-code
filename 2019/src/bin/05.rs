// --- Day 5: Sunny with a Chance of Asteroids ---
use aoc::{
    Input,
    intcode::Program,
};
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let input_file: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(ErrorKind::InvalidInput, "no input file specified"));
        },
    };
    let input = Input::from_file(input_file)?;
    let code = input.to_string().unwrap();

    // Part 1.
    let mut p = Program::from(code.as_str());
    let output = p.input(1).execute()?;
    for line in output {
        println!("{}", line);
    }

    // Part 2.
    let output = p.reset(code.as_str()).input(5).execute()?;
    for line in output {
        println!("{}", line);
    }

    Ok(())
}
