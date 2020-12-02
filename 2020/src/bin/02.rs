//! Day 2
//!
//! Your flight departs in a few days from the coastal airport;
//! the easiest way down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
//! "Something's wrong with our computers; we can't log in!"
//! You ask if you can take a look.
//!
//! Their password database seems to be a little corrupted:
//! some of the passwords wouldn't have been allowed by the Official Toboggan
//! Corporate Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input)
//! of passwords (according to the corrupted database) and the corporate policy
//! when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//!
//! Each line gives the password policy and then the password.
//! The password policy indicates the lowest and highest number of times a
//! given letter must appear for the password to be valid.
//! For example, 1-3 a means that the password must contain a at least 1 time
//! and at most 3 times.
//!
//! In the above example, 2 passwords are valid.
//! The middle password, cdefg, is not; it contains no instances of b,
//! but needs at least 1.
//! The first and third passwords are valid:
//! they contain one a or nine c,
//! both within the limits of their respective policies.
//!
//! How many passwords are valid according to their policies?

#[macro_use]
extern crate scan_fmt;

use aoc2020::Input;
use scan_fmt::parse::ScanError;
use std::io::{Error, ErrorKind, Result};
use std::str::FromStr;

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

    // Part 1: How many passwords are valid?
    let mut num_valid = 0;
    for line in input.lines().unwrap() {
        let entry = match Entry::from_str(&line) {
            Ok(e) => e,
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidInput, err.to_string()));
            }
        };
        if entry.valid() {
            num_valid += 1;
        }
    }
    println!("Valid passwords: {}", num_valid);

    // Part 2:
    // Use the valid2 method, instead.
    let mut n = 0;
    for line in input.lines().unwrap() {
        let e = match Entry::from_str(&line) {
            Ok(e) => e,
            Err(err) => {
                return Err(Error::new(ErrorKind::InvalidInput, err.to_string()));
            }
        };
        if e.valid2() {
            n += 1;
        }
    }
    println!("Valid passwords: {}", n);

    Ok(())
}

struct Entry {
    min: usize,
    max: usize,
    letter: String,
    password: String,
}

impl Entry {
    fn valid(&self) -> bool {
        let occurrences = self.password.matches(&self.letter).count();
        occurrences >= self.min && occurrences <= self.max
    }

    fn valid2(&self) -> bool {
        let a =
            self.password.chars().nth(self.min - 1).unwrap() == self.letter.chars().nth(0).unwrap();
        let b =
            self.password.chars().nth(self.max - 1).unwrap() == self.letter.chars().nth(0).unwrap();
        a ^ b
    }
}

impl FromStr for Entry {
    type Err = ScanError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (min, max, letter, password) =
            scan_fmt!(s, "{}-{} {}: {}", usize, usize, String, String)?;
        Ok(Self {
            min,
            max,
            letter,
            password,
        })
    }
}

#[test]
fn test_entry_valid() {
    let raw_input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let input = Input::from(raw_input);

    let mut n: usize = 0;
    for line in input.lines().unwrap() {
        let entry = Entry::from_str(&line).unwrap();
        if entry.valid() {
            n += 1;
        }
    }

    assert_eq!(n, 2);
}

#[test]
fn test_entry_valid2() {
    let raw_input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
    let input = Input::from(raw_input);

    let mut n: usize = 0;
    for line in input.lines().unwrap() {
        let entry = Entry::from_str(&line).unwrap();
        if entry.valid2() {
            println!("{:?}: valid", line);
            n += 1;
        } else {
            println!("{:?}: invalid", line);
        }
    }

    assert_eq!(n, 1);
}
