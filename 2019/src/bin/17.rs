use aoc::{intcode::Program, Input};
use ascii::AsciiStr;
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

    // Part 1
    let mut program = Program::from(code.as_str());
    let map = parse_output(program.execute()?);

    let mut intersections: Vec<(usize, usize)> = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '#' && y > 0 && x > 0 && y + 1 < map.len() - 1 && x + 1 < row.len() - 1 {
                let above = map[y - 1][x] == '#';
                let below = map[y + 1][x] == '#';
                let before = map[y][x - 1] == '#';
                let after = map[y][x + 1] == '#';

                if above && below && before && after {
                    intersections.push((x, y));
                    eprint!("O");
                } else {
                    eprint!("{}", c);
                }
            } else {
                eprint!("{}", c);
            }
        }
        eprintln!("");
    }

    let mut sum: usize = 0;
    for i in intersections {
        //eprintln!("{: >3?} => {}", i, i.0 * i.1);
        sum += i.0 * i.1;
    }
    println!("Sum of alignment parameters: {}", sum);

    // Part 2
    //
    // NOTE: I figured this out on pen and paper.
    let mut program = Program::from(code.as_str());
    assert_eq!(program.peek_mem(0), 1);
    program.set_mem(0, 2);

    let newline = 10;
    let movement_routine = AsciiStr::from_ascii(b"A,B,A,C,A,B,C,B,C,A\n").unwrap();
    let functions = vec![
        AsciiStr::from_ascii(b"L,12,R,4,R,4,L,6\n").unwrap(), // a
        AsciiStr::from_ascii(b"L,12,R,4,R,4,R,12\n").unwrap(), //b
        AsciiStr::from_ascii(b"L,10,L,6,R,4\n").unwrap(),     //c
    ];

    // Provide the movement routine.
    let _out = program.execute()?;
    for &c in movement_routine.as_bytes() {
        program.input(c as i64);
    }

    // Provide each movement function.
    let _out = program.execute()?;
    for func in functions {
        for &c in func.as_bytes() {
            program.input(c as i64);
        }
    }

    // Do we want to see a live video feed?
    let _out = program.execute()?;
    let watch_feed = AsciiStr::from_ascii(b"n\n").unwrap();
    for &n in watch_feed.as_bytes() {
        program.input(n as i64);
    }

    let output = program.execute()?;
    match output.len() {
        1 => {
            println!("Space dust collected: {:?}", output[0]);
        }
        _ => {
            let mut s = String::new();
            for n in output {
                s.push((n as u8) as char);
            }
            eprintln!("{}", s);
            eprintln!("{}", program.peek_mem(438));
        }
    }

    Ok(())
}

fn parse_output(output: Vec<i64>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];
    let mut row: Vec<char> = vec![];
    for n in output {
        match n {
            35 => {
                row.push('#');
            }
            46 => {
                row.push('.');
            }
            10 => {
                map.push(row);
                row = vec![];
            }
            60 => {
                row.push('<');
            }
            62 => {
                row.push('>');
            }
            118 => {
                row.push('v');
            }
            94 => {
                row.push('^');
            }
            _ => {
                panic!("unexpected ascii code: {}", n);
            }
        }
    }
    map
}
