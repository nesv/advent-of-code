//! Day 1
//!
//! After saving Christmas five years in a row, you've decided to take a
//! vacation at a nice resort on a tropical island.
//! Surely, Christmas will go on without you.
//!
//! The tropical island has its own currency and is entirely cash-only.
//! The gold coins used there have a little picture of a starfish; the locals
//! just call them stars.
//! None of the currency exchanges seem to have heard of them, but somehow,
//! you'll need to find fifty of these coins by the time you arrive so you
//! can pay the deposit on your room.
//!
//! To save your vacation, you need to get all fifty stars by December 25th.
//!
//! Collect stars by solving puzzles.
//! Two puzzles will be made available on each day in the Advent calendar;
//! the second puzzle is unlocked when you complete the first.
//! Each puzzle grants one star. Good luck!
//!
//! Before you leave, the Elves in accounting just need you to fix your
//! expense report (your puzzle input); apparently, something isn't quite
//! adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020
//! and then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299.
//! Multiplying them together produces 1721 * 299 = 514579,
//! so the correct answer is 514579.
//!
//! Of course, your expense report is much larger.
//! Find the two entries that sum to 2020;
//! what do you get if you multiply them together?
//!
//! The Elves in accounting are thankful for your help;
//! one of them even offers you a starfish coin they had left over from a past
//! vacation.
//! They offer you a second one if you can find three numbers in your expense
//! report that meet the same criteria.
//!
//! Using the above example again, the three entries that sum to 2020 are
//! 979, 366, and 675.
//! Multiplying them together produces the answer, 241861950.
//!
//! In your expense report, what is the product of the three entries that sum
//! to 2020?

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

    let nums: Vec<i32> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Part 1: Find two numbers that add up to 2020,
    // and multiply them together for the answer.
    let (a, b) = find_two(2020, &nums)?;
    println!("{} * {} = {}", a, b, a * b);

    // Part 2: Find three numbers that add up to 2020,
    // and multiple them together for the answer.
    let (a, b, c) = find_three(2020, &nums)?;
    println!("{} * {} * {} = {}", a, b, c, a * b * c);

    Ok(())
}

/// Find two numbers that add to `target`.
#[inline]
fn find_two(target: i32, nums: &Vec<i32>) -> Result<(i32, i32)> {
    for i in 0..nums.len() - 2 {
        for j in i + 1..nums.len() - 1 {
            if nums[i] + nums[j] == target {
                return Ok((nums[i], nums[j]));
            }
        }
    }
    Err(Error::new(
        ErrorKind::InvalidInput,
        "no two numbers add to 2020",
    ))
}

#[test]
fn test_find_two() {
    let raw_input = "1721\n979\n366\n299\n675\n1456";
    let input = Input::from(raw_input);
    let nums: Vec<i32> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let (a, b) = find_two(2020, &nums).unwrap();
    assert_eq!((a, b), (1721, 299));
}

/// Find three numbers that add to `target`.
#[inline]
fn find_three(target: i32, nums: &Vec<i32>) -> Result<(i32, i32, i32)> {
    for i in 0..nums.len() - 3 {
        for j in i + 1..nums.len() - 2 {
            for k in i + 2..nums.len() - 1 {
                if nums[i] + nums[j] + nums[k] == target {
                    return Ok((nums[i], nums[j], nums[k]));
                }
            }
        }
    }
    Err(Error::new(
        ErrorKind::InvalidInput,
        "no three numbers add to 2020",
    ))
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
