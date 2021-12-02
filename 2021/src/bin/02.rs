//! Day 2: Dive!

use aoc2021::Input;
use std::io::{Error, ErrorKind, Result};

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

    let directions: Vec<Direction> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| Direction::from(s.as_str()))
        .collect();

    // Part 1: Calculate the horizontal position and depth after following the
    // planned course.
    // Multiply the final horizontal position by the final depth.
    let mut sub = Submarine::default();
    sub.follow(&directions);
    println!("{}", sub.distance());

    // Part 2: The same as Part 1, but accounting for "aim".
    let mut sub = Submarine::default();
    sub.follow_with_aim(&directions);
    println!("{}", sub.distance());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        let mut split = s.split_whitespace();
        let direction = if let Some(d) = split.next() {
            d
        } else {
            panic!("{}", s);
        };
        let distance: usize = if let Some(d) = split.next() {
            d.parse().unwrap()
        } else {
            panic!("{}", s);
        };

        match direction {
            "forward" => Self::Forward(distance),
            "up" => Self::Up(distance),
            "down" => Self::Down(distance),
            _ => {
                panic!("unknown direction: {}", s);
            }
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward(d) => write!(f, "forward {}", d),
            Self::Up(d) => write!(f, "up {}", d),
            Self::Down(d) => write!(f, "down {}", d),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    depth: usize,
    hpos: usize,
    aim: usize,
}

impl Submarine {
    fn follow(&mut self, directions: &[Direction]) {
        for dir in directions {
            match dir {
                Direction::Forward(n) => {
                    self.hpos += n;
                }
                Direction::Up(n) => {
                    self.depth -= n;
                }
                Direction::Down(n) => {
                    self.depth += n;
                }
            }
        }
    }

    fn distance(&self) -> usize {
        self.depth * self.hpos
    }

    fn follow_with_aim(&mut self, directions: &[Direction]) {
        for dir in directions {
            match dir {
                Direction::Down(n) => {
                    self.aim += n;
                }
                Direction::Up(n) => {
                    self.aim -= n;
                }
                Direction::Forward(n) => {
                    self.hpos += n;
                    self.depth += n * self.aim;
                }
            }
        }
    }
}
