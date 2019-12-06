//! # Day 4: Secure Container
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a
//! password.
//! The Elves had written the password on a sticky note, but someone threw it
//! out.
//!
//! However, they do remember a few key facts about the password:
//!
//! *    It is a six-digit number.
//! *    The value is within the range given in your puzzle input.
//! *    Two adjacent digits are the same (like 22 in 122345).
//! *    Going from left to right, the digits never decrease; they only ever
//!      increase or stay the same (like 111123 or 135679).
//!
//! Other than the range rule, the following are true:
//!
//! *    111111 meets these criteria (double 11, never decreases).
//! *    223450 does not meet these criteria (decreasing pair of digits 50).
//! *    123789 does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input
//! meet these criteria?

use aoc::Input;
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
    let input_string = input.to_string().unwrap();
    let sep = match input_string.find("-") {
        Some(n) => n,
        None => {
            return Err(Error::new(ErrorKind::Other, "Cannot find range separator"));
        }
    };
    let (rstart, rend) = input_string.split_at(sep);
    let start: i32 = rstart.parse().unwrap();
    let end: i32 = rend[1..].parse().unwrap();

    // Part 1
    let passwords = part1(start, end)?;
    println!("{}", passwords.len());

    // Part 2
    let passwords = part2(passwords)?;
    println!("{}", passwords.len());

    Ok(())
}

fn part1(start: i32, end: i32) -> Result<Vec<String>> {
    let mut passwords: Vec<String> = vec![];
    for n in start..end + 1 {
        let p = n.to_string();
        if valid(&p) {
            passwords.push(p);
            continue;
        }
    }
    Ok(passwords)
}

fn valid(p: &str) -> bool {
    has_double(p) && increments(p)
}

#[test]
fn test_valid() {
    assert_eq!(valid("111111"), true); // ok
    assert_eq!(valid("223450"), false); // fail - decreasing
    assert_eq!(valid("123789"), false); // fail - no double
    assert_eq!(valid("624566"), false); // fail - decreasing
}

fn part2(passwords: Vec<String>) -> Result<Vec<String>> {
    let v: Vec<String> = passwords
        .iter()
        .filter(|p| check(p))
        .map(|p| String::from(p))
        .collect();
    Ok(v)
}

fn check(p: &str) -> bool {
    increments(p) && has_run(p)
}

#[test]
fn test_check() {
    assert_eq!(check("112233"), true);
    assert_eq!(check("123444"), false);
    assert_eq!(check("111122"), true);

    assert_eq!(check("123445"), true);
}

fn has_double(p: &str) -> bool {
    for i in 0..p.len() - 1 {
        let a = p.chars().nth(i).unwrap_or('1');
        let b = p.chars().nth(i + 1).unwrap_or('0');
        if a == b {
            return true;
        }
    }
    false
}

fn increments(p: &str) -> bool {
    for i in 0..p.len() - 1 {
        let a = p.chars().nth(i).unwrap_or('1');
        let b = p.chars().nth(i + 1).unwrap_or('0');
        if a > b {
            return false;
        }
    }
    true
}

fn has_run(p: &str) -> bool {
    let mut count = 1;
    for i in 1..p.len() {
        let a = p.chars().nth(i - 1).unwrap();
        let b = p.chars().nth(i).unwrap();
        if b == a {
            count += 1;
        } else {
            if count == 2 {
                return true;
            }
            count = 1;
        }
    }
    count == 2
}
