//! Day 9: Encoding Error

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
    let numbers: Vec<isize> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| match s.parse::<isize>() {
            Ok(n) => n,
            Err(e) => {
                panic!("parse {}: {}", s, e);
            }
        })
        .collect();

    // Part 1:
    // Find the first number in the input, which cannot be the result of
    // adding any two of the previous 25 numbers.
    // Note that the two numbers cannot be the same.
    let preamble = 25;
    let (num, pos) = find_invalid_number(numbers.clone(), preamble).unwrap();
    println!("Part 1: {}", num);

    // Part 2:
    // Find a contiguous set of >=2  numbers from 0..index that add up to num.
    // Sum the largest and smallest numbers.
    let sum = break_encryption(numbers, pos).unwrap();
    println!("Part 2: {}", sum);

    Ok(())
}

/// Returns the first number in `numbers` that is not the sum of any two of
/// the previous `preamble` numbers,
/// along with its index in `numbers`.
fn find_invalid_number(numbers: Vec<isize>, preamble: usize) -> Option<(isize, usize)> {
    'outer: for i in preamble..numbers.len() {
        let target = numbers[i];
        for j in i - preamble..i - 1 {
            for k in j + 1..i {
                let n = numbers[j];
                let m = numbers[k];

                if n + m == target && n != m {
                    continue 'outer;
                }

                if j == i - 2 && k == i - 1 && n + m != target {
                    return Some((target, i));
                }
            }
        }
    }
    None
}

/// Returns the sum of the smallest and largest numbers in a contiguous set
/// of numbers that add up to `numbers[pos]`.
fn break_encryption(numbers: Vec<isize>, pos: usize) -> Option<isize> {
    let target = numbers[pos];
    'outer: for i in 0..pos {
        let mut sum = numbers[i];
        for j in i + 1..pos - 1 {
            sum += numbers[j];
            if sum == target {
                // Find the smallest and largest numbers in the range,
                // and add them together.
                let mut set: Vec<isize> = numbers[i..=j].iter().map(|n| *n).collect();
                set.sort_unstable();
                let sum = set[0] + set[set.len() - 1];
                return Some(sum);
            } else if sum > target {
                // If our sum has jumped past the target value,
                // continue on with the next iteration of the loop.
                continue 'outer;
            }
        }
    }
    None
}

#[test]
fn test_break_xmas_encryption() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let input = Input::from(input);
    let numbers: Vec<isize> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| match s.parse::<isize>() {
            Ok(n) => n,
            Err(e) => {
                panic!("parse {}: {}", s, e);
            }
        })
        .collect();

    let preamble = 5;

    // Part 1.
    let (num, pos) = find_invalid_number(numbers.clone(), preamble).unwrap();
    assert_eq!(num, 127);

    // Part 2.
    assert_eq!(break_encryption(numbers, pos), Some(62));
}
