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
    let (mem, _) = program.execute()?;
    println!("{}", mem[0]);

    // Part 2.
    let mut program = Program::from(code.as_str());
    let want = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            program.set_mem(1, noun)?;
            program.set_mem(2, verb)?;

            let (mem, _output) = program.execute()?;
            if mem[0] == want {
                println!("{}", (100 * noun) + verb);
                return Ok(());
            }
        }
    }

    /*
    let mem: Vec<usize> = input
        .trim_end()
        .split(",")
        .map(|i| match i.parse::<usize>() {
            Ok(v) => v,
            Err(err) => {
                eprintln!("{}: {}", err, i);
                std::process::exit(1);
            },
        })
    .collect();

    // Part 1.
    let mut p1 = mem.clone();
    p1[1] = 12;
    p1[2] = 2;
    println!("{}", compute(p1)[0]);

    // Part 2.
    let want = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = mem.clone();
            mem[1] = noun;
            mem[2] = verb;
            if compute(mem)[0] == want {
                println!("{}", (100 * noun) + verb);
                return Ok(());
            }
        }
    }
    */

    Ok(())
}

/*
fn compute(mem: Vec<usize>) -> Vec<usize> {
    let mut mem = mem.clone();
    let mut i: usize = 0;
    loop {
        // mem[i] == instruction pointer
        match mem[i] {
            // Addition.
            1 => {
                let noun = mem[i+1];
                let verb = mem[i+2];
                let output = mem[i+3];
                mem[output] = mem[noun] + mem[verb];
            },

            // Multiplication.
            2 => {
                let noun = mem[i+1];
                let verb = mem[i+2];
                let output = mem[i+3];
                mem[output] = mem[noun] * mem[verb];
            },

            // HCF.
            99 => { break; },

            _ => { eprintln!("Unexpected opcode at ip={}: {}", i, mem[i]); }
        }

        if i + 4 >= mem.len() {
            break;
        }
        i += 4;
    }
    mem
}
*/
