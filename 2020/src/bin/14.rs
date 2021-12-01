//! Day 14: Docking Data

use aoc2020::Input;
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
    let instructions: Vec<Instruction> = input
        .lines()
        .unwrap()
        .iter()
        .map(|ln| Instruction::from_str(ln).unwrap())
        .collect();

    let mut program = Program::default();
    program.run(&instructions);
    println!("Part 1: {}", program.sum());

    Ok(())
}

struct Program {
    mem: Vec<u64>,
    mask: u64,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            mem: Vec::new(),
            mask: 0,
        }
    }
}

impl Program {
    fn run(&mut self, instructions: &[Instruction]) {
        for i in instructions {
            match i {
                Instruction::Mask(m) => {
                    self.set_mask(*m);
                }
                Instruction::Mem(pos, val) => {
                    self.set_mem(*pos, *val);
                }
            }
        }
    }

    fn set_mask(&mut self, mask: u64) {
        self.mask = mask;
    }

    fn set_mem(&mut self, pos: usize, value: u64) {
        if self.mem.len() < pos {
            self.mem.resize(pos, 0);
        }
        self.mem[pos] = value & self.mask;
    }

    fn sum(&self) -> u64 {
        self.mem.iter().fold(0, |acc, x| acc + x)
    }
}

enum Instruction {
    Mask(u64),
    Mem(usize, u64),
}

use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "=").map(|s| s.trim()).collect();
        eprintln!("s={:?} parts={:?}", s, parts);
        match parts[0] {
            "mask" => {
                let n: u64 = parts[1].parse().unwrap();
                Ok(Self::Mask(n))
            }
            _ => {
                // "mem[POS]"
                let pos: usize = parts[0]
                    .splitn(2, "[")
                    .nth(1)
                    .unwrap()
                    .strip_suffix("]")
                    .unwrap()
                    .parse()
                    .unwrap();
                let n: u64 = parts[1].parse().unwrap();
                Ok(Self::Mem(pos, n))
            }
        }
    }
}
