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

    // Part 1.
    let mut program = Program::from(code.as_str());
    program.set_mem(1, 12)?;
    program.set_mem(2, 2)?;
    program.execute()?;
    println!("{}", program.peek_mem(0));

    // Part 2.
    let mut program = Program::from(code.as_str());
    let want = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            program.set_mem(1, noun)?;
            program.set_mem(2, verb)?;
            program.execute()?;
            if program.peek_mem(0) == want {
                println!("{}", (100 * noun) + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}
