//! Day 5: Binary Boarding

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
    let boarding_passes: Vec<String> = input.lines().unwrap();
    let mut seat_ids: Vec<usize> = boarding_passes.iter().map(|s| seat_id(&s)).collect();
    seat_ids.sort_unstable();

    // Part 1: What is the highest seat ID on a boarding pass?
    let sid: usize = *seat_ids.iter().max().unwrap();
    println!("Highest seat ID: {}", sid);

    // Part 2: Find your seat.
    // It will be the missing one from the sorted range of seat IDs.
    for i in 1..seat_ids.len() {
        if seat_ids[i] - seat_ids[i - 1] == 2 {
            println!("Your seat ID: {}", seat_ids[i] - 1);
        }
    }

    Ok(())
}

// Returns the `(row, column)` of the seat for the given boarding pass.
fn find_seat(s: &str) -> (usize, usize) {
    let mut row: Vec<usize> = (0..=127).collect();
    let mut col: Vec<usize> = (0..=7).collect();

    for c in s.chars() {
        match c {
            'F' => {
                // Keep the lower-half of the row range.
                let n = row.len() / 2;
                row = row[..n].to_vec();
            }
            'B' => {
                // Keep the upper-half of the row range.
                let n = row.len() / 2;
                row = row[n..].to_vec();
            }
            'R' => {
                // Keep the upper-half of the col range.
                let n = col.len() / 2;
                col = col[n..].to_vec();
            }
            'L' => {
                // Keep the lower-half of the col range.
                let n = col.len() / 2;
                col = col[..n].to_vec();
            }
            _ => {
                panic!("unexpected character in boarding pass {:?}: {:?}", s, c);
            }
        }
    }

    (row[0], col[0])
}

#[inline]
// Calculates the ID for the seat with the given boarding pass.
fn seat_id(pass: &str) -> usize {
    let loc = find_seat(pass);
    loc.0 * 8 + loc.1
}

#[test]
fn test_find_seat() {
    let tests = vec![
        ("FBFBBFFRLR", (44, 5)),
        ("BFFFBBFRRR", (70, 7)),
        ("FFFBBBFRRR", (14, 7)),
        ("BBFFBBFRLL", (102, 4)),
    ];
    for test in tests {
        let result = find_seat(test.0);
        assert_eq!(result, test.1);
    }
}

#[test]
fn test_seat_id() {
    let tests = vec![
        ("FBFBBFFRLR", 357),
        ("BFFFBBFRRR", 567),
        ("FFFBBBFRRR", 119),
        ("BBFFBBFRLL", 820),
    ];
    for test in tests {
        let result = seat_id(test.0);
        assert_eq!(result, test.1);
    }
}
