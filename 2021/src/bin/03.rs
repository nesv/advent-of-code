//! Day 3: Binary Diagnostic

use aoc2021::Input;
use itertools::partition;
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

    let values: Vec<usize> = input
        .lines()
        .unwrap()
        .iter()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();

    // Part 1: Power consumption.
    println!("power consumption = {}", power_consumption(&values));

    // Part 2: Life support rating.
    println!("life support rating = {}", life_support_rating(&values));

    Ok(())
}

fn power_consumption(readings: &[usize]) -> usize {
    let gamma = gamma_rate(readings);
    let epsilon = epsilon_rate(gamma);
    gamma * epsilon
}

fn gamma_rate(readings: &[usize]) -> usize {
    let num_readings = readings.len();
    let mut gamma = 0;
    for i in 0..12 {
        let mask = 1 << i;
        let mut ones = 0;
        for r in readings {
            if r & mask == mask {
                ones += 1;
            }
        }
        if ones >= num_readings / 2 {
            gamma |= mask;
        }
    }
    gamma
}

fn epsilon_rate(gamma: usize) -> usize {
    !gamma & 0b1111_1111_1111
}

fn life_support_rating(readings: &[usize]) -> usize {
    let oxygen = oxygen_generator_rating(readings);
    let co2_scrub = co2_scrubber_rating(readings);
    oxygen * co2_scrub
}

fn oxygen_generator_rating(readings: &[usize]) -> usize {
    let mut readings: Vec<usize> = readings.to_vec();
    for i in (0..12).rev() {
        let mask = 1 << i;
        let split_index = partition(&mut readings, |&v| v & mask == mask);
        readings = if split_index >= readings.len() / 2 {
            readings[0..split_index].to_vec()
        } else {
            readings[split_index..].to_vec()
        };
        if readings.len() == 1 {
            break;
        }
    }
    readings[0]
}

fn co2_scrubber_rating(readings: &[usize]) -> usize {
    let mut readings: Vec<usize> = readings.to_vec();
    for i in (0..12).rev() {
        let mask = 1 << i;
        let split_index = partition(&mut readings, |&v| v & mask == mask);
        readings = if split_index < readings.len() / 2 {
            readings[0..split_index].to_vec()
        } else {
            readings[split_index..].to_vec()
        };
        if readings.len() == 1 {
            break;
        }
    }
    readings[0]
}
