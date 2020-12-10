//! Day 10: Adapter Array

use aoc2020::Input;
use std::{
    convert::TryInto,
    io::{Error, ErrorKind, Result},
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
    let adapters = input.numbers().unwrap();

    // Part 1: Using all of the adapters,
    // multiply the number of adapters with a joltage difference of 1,
    // by the number of joltage adapters with a difference of 3.
    let diffs = differences(adapters.clone());
    let ones = diffs.iter().fold(0, |acc, d| match d {
        Difference::One => acc + 1,
        Difference::Other(_) => {
            panic!("unexpected difference: {:?}", d);
        }
        _ => acc,
    });
    let threes = diffs.iter().fold(0, |acc, d| match d {
        Difference::Three => acc + 1,
        _ => acc,
    });

    println!("Part 1: {}", ones * threes);

    // Part 2: How many different arrangements can be made from the joltage
    // adapters?
    let arrs = arrangements(adapters.clone());
    println!("Part 2: {}", arrs);

    Ok(())
}

fn differences(mut adapters: Vec<isize>) -> Vec<Difference> {
    adapters.sort_unstable();

    let mut diffs = vec![];

    // Initial difference, between the outlet and the first adapter.
    diffs.push(Difference::from(adapters[0]));

    for i in 1..adapters.len() {
        diffs.push(Difference::from(adapters[i] - adapters[i - 1]));
    }

    // Final difference, which is always three.
    diffs.push(Difference::Three);

    diffs
}

#[derive(Debug, PartialEq)]
enum Difference {
    One,
    Three,
    Other(isize),
}

impl From<isize> for Difference {
    fn from(n: isize) -> Self {
        match n {
            1 => Self::One,
            3 => Self::Three,
            _ => Self::Other(n),
        }
    }
}

fn arrangements(mut adapters: Vec<isize>) -> usize {
    adapters.sort_unstable();

    // Start with the outlet (0).
    let mut ads: Vec<isize> = vec![0];

    // Include our joltage adapters.
    ads.append(&mut adapters);

    // Finish with our device.
    let max = ads.iter().max().unwrap().clone();
    ads.push(max + 3);

    let mut p2: i32 = 0;
    let mut p7: i32 = 0;
    for i in 1..ads.len() - 1 {
        let n3 = if i >= 3 { ads[i - 3] } else { -9999 };
        if ads[i + 1] - n3 == 4 {
            p2 -= 2;
            p7 += 1;
        } else if ads[i + 1] - ads[i - 1] == 2 {
            p2 += 1;
        }
    }
    (2_usize.pow(p2.try_into().unwrap()) * 7_usize.pow(p7.try_into().unwrap())) as usize
}

#[test]
fn test_differences() {
    let adapters: Vec<isize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let diffs = differences(adapters);
    let ones = diffs.iter().fold(0, |acc, d| match d {
        Difference::One => acc + 1,
        _ => acc,
    });
    assert_eq!(ones, 7);

    let threes = diffs.iter().fold(0, |acc, d| match d {
        Difference::Three => acc + 1,
        _ => acc,
    });
    assert_eq!(threes, 5);
}

#[test]
fn test_arrangements() {
    let adapters: Vec<isize> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    assert_eq!(arrangements(adapters), 8);

    let adapters: Vec<isize> = vec![
        28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8,
        17, 7, 9, 4, 2, 34, 10, 3,
    ];
    assert_eq!(arrangements(adapters), 19208);
}
