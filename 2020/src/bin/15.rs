//! Day 15: Rambunctious Recitation

use aoc2020::Input;
use std::io::{Error, ErrorKind, Result};

fn main() -> Result<()> {
    let infile = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "no input file specified",
            ));
        }
    };
    let input = Input::from_file(infile)?;
    let nums: Vec<usize> = input
        .lines()
        .unwrap()
        .iter()
        .map(|ln| ln.split(",").map(|s| s.parse::<usize>().unwrap()))
        .flatten()
        .collect();

    println!("Part 1: {}", findn(2020, &nums));

    Ok(())
}

// If this is the first time a number has been "said",
// then say "0".
// If this isn't the first time a number has been said,
// then say the difference in turns from when the last time it was said.
//
// What is the 2020th number?
fn findn(index: usize, nums: &[usize]) -> usize {
    let mut said: Vec<usize> = nums.to_vec().clone();
    for turn in said.len()..index + 1 {
        let prev = said[turn - 1];
	let p = match said.iter().rev().skip(1).position(|&n|n==prev) {
	    Some(i) => turn-i,
	    None=> 0,
	};
        if let Some(i) = said.iter().rev().skip(1).position(|&n| n == prev) {
	    eprintln!("i={:?}",i);
            said.push(turn - (said.len() - 1 - i));
        } else {
            said.push(0);
        }
    }
    eprintln!("said={:?}", said);
    *said.iter().last().unwrap()
}

#[test]
fn test_part1() {
    let nums = [0, 3, 6];
    assert_eq!(findn(2020, &nums), 436);

    let nums = [1, 3, 2];
    assert_eq!(findn(2020, &nums), 1);

    let nums = [2, 1, 3];
    assert_eq!(findn(2020, &nums), 10);

    let nums = [1, 2, 3];
    assert_eq!(findn(2020, &nums), 27);

    let nums = [2, 3, 1];
    assert_eq!(findn(2020, &nums), 78);

    let nums = [3, 2, 1];
    assert_eq!(findn(2020, &nums), 438);

    let nums = [3, 1, 2];
    assert_eq!(findn(2020, &nums), 1836);
}
