//! Day 12: Rain Risk

use aoc2020::Input;
use std::{
    io::{Error, ErrorKind, Result},
    num::ParseIntError,
    str::FromStr,
};

fn main() -> Result<()> {
    let infile: String = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(infile)?;
    let instructions = parse_instructions(input)?;

    // Part 1:
    // Get the Manhattan distance the ferry has traveled.
    // The ferry always starts facing east.
    let mut ferry = Ferry::default();
    for i in &instructions {
        ferry.travel(i);
    }
    println!(
        "Part 1: {} loc={:?}",
        ferry.manhattan_distance(),
        ferry.location()
    );

    // Part 2:
    // Get the Manhattan distance the ferry has traveled,
    // but using the `travel_relative` method instead,
    // so that it travels relative to a waypoint.
    let mut ferry = Ferry::default();
    for i in &instructions {
        ferry.travel_relative(i);
    }
    println!(
        "Part 2: {} loc={:?}",
        ferry.manhattan_distance(),
        ferry.location()
    );

    Ok(())
}

struct Ferry {
    loc: (i32, i32),
    dir: Direction,
    waypoint: (i32, i32),
}

impl Default for Ferry {
    fn default() -> Self {
        Self {
            loc: (0, 0),
            dir: Direction::East,
            waypoint: (10, 1),
        }
    }
}

impl Ferry {
    // Move the ferry according to its current position.
    //
    // This implementation is for Part 1 of the puzzle.
    // For moving the ferry relative to the "waypoint" (part 2),
    // use `travel_relative`.
    fn travel(&mut self, inst: &Instruction) {
        match *inst {
            Instruction::North(n) => {
                self.loc.1 += n;
            }
            Instruction::South(n) => {
                self.loc.1 -= n;
            }
            Instruction::East(n) => {
                self.loc.0 += n;
            }
            Instruction::West(n) => {
                self.loc.1 -= n;
            }

            Instruction::Forward(n) => {
                // Move the ship forward in the direction it is currently
                // pointing.
                match self.dir {
                    Direction::East => {
                        self.loc.0 += n;
                    }
                    Direction::West => {
                        self.loc.0 -= n;
                    }
                    Direction::North => {
                        self.loc.1 += n;
                    }
                    Direction::South => {
                        self.loc.1 -= n;
                    }
                }
            }

            Instruction::Left(_) | Instruction::Right(_) => {
                self.dir = self.dir.rotate(inst);
            }
        }
    }

    fn manhattan_distance(&self) -> i32 {
        self.loc.0.abs() + self.loc.1.abs()
    }

    // Move the boat according to the given `instruction`,
    // about the waypoint.
    //
    // Only `Instruction::Forward` instructions will move the boat.
    //
    // Any `N`, `E`, `S`, or `W` instructions change the location of the
    // waypoint,
    // and `L` and `R` instructions rotate the waypoint about the ferry.
    fn travel_relative(&mut self, instruction: &Instruction) {
        match *instruction {
            Instruction::North(n) => {
                self.waypoint.1 += n;
            }
            Instruction::South(n) => {
                self.waypoint.1 -= n;
            }
            Instruction::East(n) => {
                self.waypoint.0 += n;
            }
            Instruction::West(n) => {
                self.waypoint.0 -= n;
            }

            Instruction::Left(n) | Instruction::Right(n) => {
                // Rotate the waypoint about the ferry.
                let wp = self.rotate_waypoint(self.waypoint, n);
                self.waypoint = wp;
            }

            Instruction::Forward(n) => {
                // Move forward towards the waypoint `n` times.
                //
                // Get the slope between the ferry and the waypoint.
                let d = (self.waypoint.0 - self.loc.0, self.waypoint.1 - self.loc.1);
                let offset = (d.0 * n, d.1 * n);
                self.loc = (self.loc.0 + offset.0, self.loc.1 + offset.1);
                self.waypoint = (self.loc.0 + d.0, self.loc.1 + d.1);
            }
        }
        eprintln!(
            "instruction={:?} loc={:?} waypoint={:?}",
            instruction, self.loc, self.waypoint
        );
    }

    // Rotate the waypoint about the ferry.
    fn rotate_waypoint(&self, waypoint: (i32, i32), degrees: i32) -> (i32, i32) {
        let d = (waypoint.0 - self.loc.0, waypoint.1 - self.loc.1);
        let d = match degrees {
            90 => (d.1, d.0 * -1),
            180 => (d.0 * -1, d.1 * -1),
            270 => (d.1 * -1, d.0),
            _ => {
                panic!("invalid degrees: {}", degrees);
            }
        };
        let wp = (self.loc.0 + d.0, self.loc.1 + d.1);

        eprintln!(
            "loc={:?} waypoint={:?} degrees={:?} d={:?} new_waypoint={:?}",
            self.loc, waypoint, degrees, d, wp
        );
        wp
    }

    fn location(&self) -> (i32, i32) {
        self.loc
    }
}

fn parse_instructions(input: Input) -> Result<Vec<Instruction>> {
    let mut v: Vec<Instruction> = vec![];
    for line in input.lines().unwrap() {
        let inst = Instruction::from_str(&line).unwrap();
        v.push(inst);
    }
    Ok(v)
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let n = s[1..].parse::<i32>()?;
        match s.chars().nth(0).unwrap() {
            'N' => Ok(Self::North(n)),
            'E' => Ok(Self::East(n)),
            'S' => Ok(Self::South(n)),
            'W' => Ok(Self::West(n)),
            'F' => Ok(Self::Forward(n)),
            'L' => Ok(Self::Left(n)),
            'R' => Ok(Self::Right(n)),
            _ => {
                panic!("bad instruction: {}", s);
            }
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate(&self, instruction: &Instruction) -> Self {
        let deg: i32 = match self {
            Self::East => 0,
            Self::South => 90,
            Self::West => 180,
            Self::North => 270,
        };

        let d: i32 = match *instruction {
            Instruction::Left(n) => (deg - n) % 360,
            Instruction::Right(n) => (deg + n) % 360,
            _ => {
                panic!("cannot rotate on non L or R instruction: {:?}", instruction);
            }
        };

        match d {
            0 => Self::East,
            90 | -270 => Self::South,
            180 | -180 => Self::West,
            270 | -90 => Self::North,
            _ => {
                panic!("unexpected number of degrees: {}", d);
            }
        }
    }
}

#[test]
fn test_travel() {
    let raw = "F10
N3
F7
R90
F11";
    let input = Input::from(raw);
    let instructions = parse_instructions(input).unwrap();

    let mut ferry = Ferry::default();
    for i in instructions {
        ferry.travel(&i);
    }
    assert_eq!(ferry.manhattan_distance(), 25);
}

#[test]
fn test_travel_relative() {
    let raw = "F10
N3
F7
R90
F11";
    let input = Input::from(raw);
    let instructions = parse_instructions(input).unwrap();

    let mut ferry = Ferry::default();
    for i in instructions {
        ferry.travel_relative(&i);
    }
    assert_eq!(ferry.manhattan_distance(), 286);
}

#[test]
fn test_relative_travel_return_to_start() {
    let raw = "F10
R180
F20
L180
F20
R90
R90
F20
L90
L90
F10";
    let input = Input::from(raw);
    let instructions = parse_instructions(input).unwrap();
    let mut ferry = Ferry::default();
    for i in instructions {
        ferry.travel_relative(&i);
    }
    assert_eq!(ferry.manhattan_distance(), 0);
}
