//! # Day 8: Handheld Halting
//!
//! Your flight to the major airline hub reaches cruising altitude
//! without incident.
//! While you consider checking the in-flight menu for one of those drinks that
//! come with a little umbrella,
//! you are interrupted by the kid sitting next to you.
//!
//! Their handheld game console won't turn on! They ask if you can take a look.
//!
//! You narrow the problem down to a strange infinite loop in the boot code
//! (your puzzle input) of the device.
//! You should be able to fix it, but first you need to be able to run the code
//! in isolation.
//!
//! The boot code is represented as a text file with one instruction
//! per line of text.
//! Each instruction consists of an operation
//! (`acc`, `jmp`, or `nop`)
//! and an argument
//! (a signed number like `+4` or `-20`).
//!
//! - `acc` increases or decreases a single global value called the accumulator
//!   by the value given in the argument.
//!   For example, `acc +7` would increase the accumulator by 7.
//!   The accumulator starts at 0.
//!   After an `acc` instruction, the instruction immediately below it is
//!   executed next.
//! - `jmp` jumps to a new instruction relative to itself.
//!   The next instruction to execute is found using the argument as an offset
//!   from the jmp instruction;
//!   for example, `jmp +2` would skip the next instruction,
//!   `jmp +1` would continue to the instruction immediately below it,
//!   and `jmp -20` would cause the instruction 20 lines above to be executed
//!   next.
//! - `nop` stands for No OPeration - it does nothing.
//!   The instruction immediately below it is executed next.
//!
//! For example, consider the following program:
//! ```
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! These instructions are visited in this order:
//!
//! ```
//! nop +0  | 1
//! acc +1  | 2, 8(!)
//! jmp +4  | 3
//! acc +3  | 6
//! jmp -3  | 7
//! acc -99 |
//! acc +1  | 4
//! jmp -4  | 5
//! acc +6  |
//! ```
//!
//! First,
//! the `nop +0` does nothing.
//! Then,
//! the accumulator is increased from 0 to 1 (`acc +1`) and `jmp +4` sets the next
//! instruction to the other `acc +1` near the bottom.
//! After it increases the accumulator from 1 to 2,
//! `jmp -4` executes,
//! setting the next instruction to the only `acc +3`.
//! It sets the accumulator to 5,
//! and `jmp -3` causes the program to continue back at the first `acc +1`.
//!
//! This is an infinite loop:
//! with this sequence of jumps,
//! the program will run forever.
//! The moment the program tries to run any instruction a second time,
//! you know it will never terminate.
//!
//! Immediately before the program would run an instruction a second time,
//! the value in the accumulator is 5.
//!
//! ## Part 1
//!
//! Run your copy of the boot code.
//! Immediately before any instruction is executed a second time,
//! what value is in the accumulator?
//!
//! ## Part 2
//!
//! After some careful analysis,
//! you believe that exactly one instruction is corrupted.
//!
//! Somewhere in the program,
//! either a `jmp` is supposed to be a `nop`,
//! or a `nop` is supposed to be a `jmp`.
//! (No `acc` instructions were harmed in the corruption of this boot code.)
//!
//! The program is supposed to terminate by attempting to execute an instruction
//! immediately after the last instruction in the file.
//! By changing exactly one `jmp` or `nop`,
//! you can repair the boot code and make it terminate correctly.
//!
//! For example,
//! consider the same program from above:
//!
//! ```
//! nop +0
//! acc +1
//! jmp +4
//! acc +3
//! jmp -3
//! acc -99
//! acc +1
//! jmp -4
//! acc +6
//! ```
//!
//! If you change the first instruction from `nop +0` to `jmp +0`,
//! it would create a single-instruction infinite loop,
//! never leaving that instruction.
//! If you change almost any of the `jmp` instructions,
//! the program will still eventually find another `jmp` instruction and loop
//! forever.
//!
//! However,
//! if you change the second-to-last instruction
//! (from `jmp -4` to `nop -4`),
//! the program terminates!
//! The instructions are visited in this order:
//!
//! ```
//! nop +0  | 1
//! acc +1  | 2
//! jmp +4  | 3
//! acc +3  |
//! jmp -3  |
//! acc -99 |
//! acc +1  | 4
//! nop -4  | 5
//! acc +6  | 6
//! ```
//!
//! After the last instruction
//! (`acc +6`),
//! the program terminates by attempting to run the instruction below the last
//! instruction in the file.
//! With this change,
//! after the program terminates,
//! the accumulator contains the value 8
//! (`acc +1`, `acc +1`, `acc +6`).
//!
//! Fix the program so that it terminates normally by changing exactly one
//! `jmp` (to `nop`) or `nop` (to `jmp`).
//! What is the value of the accumulator after the program terminates?

use aoc2020::Input;
use std::{
    collections::HashSet,
    fmt,
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

    let instructions = parse_instructions(&input)?;

    // Part 1: Before the program would execute the same instruction a second
    // time, print the accumulator value.
    let mut program = Program::from(instructions.clone());
    let acc = match program.execute() {
        Termination::Ok => {
            panic!("program should not have terminated normally");
        }
        Termination::Loop => program.acc(),
    };
    println!("Part 1: {}", acc);

    // Part 2: Change 1 Jmp instruction to a Nop instruction
    // (or one Nop instruction to a Jmp instruction), and run the program.
    // If the program terminates successfully, print the accumulator value.
    let instructions_to_swap: Vec<usize> = instructions
        .iter()
        .enumerate()
        .map(|(n, i)| match i {
            Instruction::Nop => Some(n),
            Instruction::Jmp(_) => Some(n),
            _ => None,
        })
        .filter(|i| i.is_some())
        .map(|o| o.unwrap())
        .collect();
    for i in instructions_to_swap {
        let mut instructions = instructions.clone();
        instructions[i] = match instructions[i] {
            Instruction::Jmp(_) => Instruction::Nop,
            Instruction::Nop => Instruction::Jmp(0),
            _ => instructions[i],
        };
        let mut program = Program::from(instructions);
        match program.execute() {
            Termination::Ok => {
                println!("Part 2: {}", program.acc());
                break;
            }
            Termination::Loop => {}
        }
    }

    Ok(())
}

/// Contains the puzzle input,
/// and provides methods for stepping through its execution.
///
/// # Examples
///
/// ```
/// use aoc2020::Input;
///
/// let raw_input = "nop +0
/// acc +1
/// jmp +4
/// acc +3
/// jmp -3
/// acc -99
/// acc +1
/// jmp -4
/// acc +6";
///
/// let input = Input::from(raw_input);
/// let instructions = parse_instructions(&input);
///
/// let program = Program::from(instructions);
/// ```
struct Program {
    /// The program's instructions.
    instructions: Vec<Instruction>,

    /// The instructions the program has run.
    steps: Vec<usize>,

    /// The accumulator value.
    acc: isize,

    /// The position of the _next_ instruction to execute.
    /// The instruction at this position will be executed the next time `step`
    /// is called.
    pos: usize,
}

impl From<Vec<Instruction>> for Program {
    fn from(v: Vec<Instruction>) -> Self {
        Self {
            instructions: v,
            steps: vec![],
            acc: 0,
            pos: 0,
        }
    }
}

impl FromStr for Program {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut instructions = vec![];
        for line in s.lines() {
            let i = Instruction::from_str(line)?;
            instructions.push(i);
        }
        let acc = 0;
        let pos = 0;
        let steps = vec![];
        Ok(Self {
            instructions,
            steps,
            acc,
            pos,
        })
    }
}

impl Program {
    /// The current value of the accumulator.
    fn acc(&self) -> isize {
        self.acc
    }

    /// Executes the program, returning the reason execution stopped.
    fn execute(&mut self) -> Termination {
        let mut steps = HashSet::new();
        loop {
            self.steps.push(self.pos);
            steps.insert(self.pos);

            let next = self.step();
            if next >= self.instructions.len() {
                return Termination::Ok;
            } else if steps.contains(&next) {
                return Termination::Loop;
            }
        }
    }

    /// Execute the current instruction,
    /// and update the instruction pointer.
    /// `step` will return the address of the next instruction to execute.
    ///
    /// # Panics
    ///
    /// `step` will panic if a `jmp` instruction attempts to jump to an
    /// instruction at a position < 0.
    fn step(&mut self) -> usize {
        match self.instructions[self.pos] {
            Instruction::Nop => {
                // No-op.
                // Just increment the instruction pointer, and move on.
                self.pos += 1;
            }

            Instruction::Acc(n) => {
                // Add `n` to the accumulator value.
                self.acc += n;
                self.pos += 1;
            }

            Instruction::Jmp(n) => {
                // Jump to the instruction at the relative offset `n`.
                let pos = self.pos as isize + n;
                if pos < 0 {
                    panic!("jump to an instruction < 0");
                }

                self.pos = pos as usize;
            }
        }
        self.pos
    }
}

/// Returns a `Vec` of `Instruction`s parsed from the puzzle input.
fn parse_instructions(input: &Input) -> Result<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in input.lines().unwrap() {
        match Instruction::from_str(&line) {
            Ok(i) => {
                instructions.push(i);
            }
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidInput, err));
            }
        }
    }
    Ok(instructions)
}

/// An instruction read from the puzzle input.
#[derive(Copy, Clone)]
enum Instruction {
    /// A `nop` instruction.
    Nop,

    /// An `acc` instruction,
    /// holding a signed integer.
    Acc(isize),

    /// A `jmp` instruction,
    /// with a signed integer indicating which instruction to jump to next,
    /// relative to the current instruction.
    Jmp(isize),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nop => write!(f, "nop +0"),
            Self::Acc(n) => {
                if n < 0 {
                    write!(f, "acc -{}", n)
                } else {
                    write!(f, "acc +{}", n)
                }
            }
            Self::Jmp(n) => {
                if n < 0 {
                    write!(f, "jmp -{}", n)
                } else {
                    write!(f, "jmp +{}", n)
                }
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_ascii_whitespace().collect();
        if parts.len() != 2 {
            panic!(
                "wrong number of parts after splitting instruction {:?}: want={} got={}",
                s,
                2,
                parts.len()
            );
        }

        match parts[0] {
            "nop" => Ok(Self::Nop),
            "acc" => {
                let n: isize = parts[1].parse().unwrap();
                Ok(Self::Acc(n))
            }
            "jmp" => {
                let n: isize = parts[1].parse().unwrap();
                Ok(Self::Jmp(n))
            }
            _ => {
                panic!("unexpected instruction: {:?}", parts[0]);
            }
        }
    }
}

/// Indicates the reason a `Program` terminated.
#[derive(Eq, PartialEq, Debug)]
enum Termination {
    /// The program completed successfully.
    Ok,

    /// Indicates the program detected a potentially infinite loop,
    /// and has returned early.
    Loop,
}

#[test]
fn test_loop_detection() {
    let instructions = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    let mut p = Program::from_str(instructions).unwrap();
    let term = p.execute();
    assert_ne!(term, Termination::Ok);
    assert_eq!(p.acc(), 5);
}
