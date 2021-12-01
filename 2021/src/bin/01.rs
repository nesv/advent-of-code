//! Day 1: Sonar Sweep

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

    let nums: Vec<u16> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| s.parse::<u16>().unwrap())
        .collect();

    // Part 1: Count the number of times a depth measurement increases from
    // the previous measurement.
    // The first depth measurement does not count, since there is no previous
    // measurement.
    let increases = depth_increases(&nums);
    println!("{}", increases);

    // Part 2: Count the number of times the sum of measurements in a sliding
    // window increases.
    // The window size is 3.
    let increases = windowed_depth_increases(&nums);
    println!("{}", increases);

    Ok(())
}

fn depth_increases(depths: &[u16]) -> usize {
    let mut n: usize = 0;
    for (i, v) in depths.iter().enumerate().skip(1) {
        if v > &depths[i - 1] {
            n += 1;
        }
    }
    n
}

fn windowed_depth_increases(depths: &[u16]) -> usize {
    let end = depths.iter().count() - 2;
    let mut increases: usize = 0;
    let mut prev_sum = u16::MAX;
    for i in 0..end {
        let sum = &depths[i] + &depths[i + 1] + &depths[i + 2];
        if sum > prev_sum {
            increases += 1;
        }
        prev_sum = sum;
    }
    increases
}

#[test]
fn test_find_three() {
    let raw_input = "1721\n979\n366\n299\n675\n1456";
    let input = Input::from(raw_input);
    let nums: Vec<i32> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (a, b, c) = find_three(2020, &nums).unwrap();
    assert_eq!((a, b, c), (979, 366, 675));
}
