use aoc::{
    intcode::{Program, Stop},
    Input,
};
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
    println!(
        "Number of points affected by the tractor beam: {}",
        map_tractor_beam(code.as_str(), 50, 50)?
    );

    // Part 2
    println!("Closest point: {}", part2(code.as_str())?);

    Ok(())
}

// Runs the given intcode program `code`, and probes each coordinate in the
// given `ax, ay` area to calculate how many points are affected by the tractor
// beam.
fn map_tractor_beam(code: &str, ax: isize, ay: isize) -> Result<isize> {
    let mut program = Program::from(code);
    let mut output: Vec<isize> = vec![];
    for y in 0..ay {
        for x in 0..ax {
            program.input(x);
            program.input(y);
            let out = program.execute()?;
            match program.reason_for_stop().unwrap() {
                Stop::WaitingForInput => {
                    panic!("not enough input");
                }
                Stop::HCF => {
                    output.extend(out.iter());
                    program.reset(code);
                }
            };
        }
    }
    let num_points: isize = output.iter().filter(|&&n| n == 1).sum();
    Ok(num_points)
}

// Find the point closest to the tractor beam emitter that would allow for a
// 100x100 ship to fit completely inside of it, then return those coordinates.
fn part2(code: &str) -> Result<isize> {
    let mut x = 0;
    for y in 100..isize::max_value() {
        // Advance the X-position until we are within the beam.
        // NOTE: This tracks the bottom-left coordinate of the 100x100 square.
        while !point_in_beam(code, &Point { x, y })? {
            x += 1;
        }

        // Check to see if the top-right corner of the square is also within
        // the beam.
        if point_in_beam(
            code,
            &Point {
                x: x + 99,
                y: y - 99,
            },
        )? {
            return Ok(x * 10000 + (y - 99));
        }
    }
    Err(Error::new(ErrorKind::Other, "map not large enough"))
}

fn point_in_beam(code: &str, point: &Point) -> Result<bool> {
    let mut program = Program::from(code);
    program.input(point.x);
    program.input(point.y);
    let out = program.execute()?;
    if out.len() != 1 {
        return Err(Error::new(ErrorKind::Other, "expected 1 output value"));
    }
    Ok(out[0] == 1)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn debug_location_program() {
    let input = Input::from_file("input/19").unwrap();
    let code = input.to_string().unwrap();
    let mut program = Program::from(code.as_str());
}
