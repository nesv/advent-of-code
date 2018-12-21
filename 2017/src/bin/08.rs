extern crate aoc2017;

use aoc2017::input;

fn main() {
    if let Some(data) = input::load(8) {
        let instructions = parse_instructions(data.clone().into_bytes()).unwrap();

        let mut result = p1(&instructions).unwrap();
        println!("{}", result);

        result = p2(&instructions).unwrap();
        println!("{}", res2);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Error {
    InvalidCondition,
}

fn parse_input(raw: Vec<u8>) -> Result<Vec<Instruction>, Error> {
    let mut instructions: Vec<u8> = Vec::new();
    let mut line = String::new();
    loop {
    }
}

fn parse_instruction(input: &str) -> Result<Instruction, std::num::ParseIntError> {
    input
        .split_whitespace()
        .map(|token| token.parse())
        .collect()
}

struct Instruction {
    register: &str
    operation: &str
    value: i32
    test_register: &str
    condition: &str
    test_value: i32
}

fn p1(instructions: Vec<Instruction>) -> i32 {
    0
}

fn p2(instructions: Vec<Instruction>) -> i32 {
    0
}
