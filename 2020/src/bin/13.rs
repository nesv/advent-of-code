//! Day 13: Shuttle Search

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
    let lines = input.lines().unwrap();
    let departure_time: usize = lines.iter().nth(0).unwrap().parse().unwrap();
    let schedule: Vec<String> = lines
        .iter()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.to_string())
        .collect();

    println!("Part 1: {}", part1(departure_time, &schedule));
    println!("Part 2: {}", part2(100000000000000, &schedule));
    Ok(())
}

// What is the ID of the earliest bus you can take from the airport,
// multiplied by the number of minutes you'll need to wait for that bus?
fn part1(departure_time: usize, schedule: &Vec<String>) -> usize {
    let buses: Vec<usize> = schedule
        .iter()
        .filter(|&s| s != "x")
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    let bus_times: Vec<usize> = buses
        .iter()
        .map(|n| (n * (departure_time / n)) + n)
        .collect();
    let waiting_times: Vec<usize> = bus_times.iter().map(|n| n - departure_time).collect();
    let min_waiting_time: usize = *waiting_times.iter().min().unwrap();
    let min_idx: usize = waiting_times
        .iter()
        .position(|&n| n == min_waiting_time)
        .unwrap();
    buses[min_idx] * min_waiting_time
}

// Given the list of buses (input line #2),
// what is the earliest timestamp that all listed buses depart at offsets
// matching their position in the schedule?
fn part2(start_at: usize, schedule: &[String]) -> usize {
    // Collect the IDs of the buses that are in service,
    // along with their position in the schedule.
    let mut buses: Vec<(usize, usize)> = vec![];
    for (i, s) in schedule.iter().enumerate() {
        if s == "x" {
            continue;
        }
        let bus: usize = s.parse().unwrap();
        buses.push((bus, i));
    }

    let mut n = 0;
    let mut step = 1;
    for bus in buses {
        for t in (n..usize::MAX).step_by(step) {
            if (t + bus.1) % bus.0 == 0 {
                n = t;
                step *= bus.0;
                //eprintln!("n={} step={}", n, step);
                break;
            }
        }
    }
    n
}

// Returns the earliest departure time for the `bus` at,
// or after,
// the given timestamp `t`.
fn earliest_departure(t: usize, bus: usize) -> usize {
    if t % bus == 0 {
        t
    } else {
        (bus * (t / bus)) + bus
    }
}

#[test]
fn test_part2() {
    let schedule: Vec<String> = "17,x,13,19".split(",").map(|s| s.to_string()).collect();
    assert_eq!(part2(0, &schedule), 3417);

    let schedule: Vec<String> = "67,7,59,61".split(",").map(|s| s.to_string()).collect();
    assert_eq!(part2(754000, &schedule), 754018);

    let schedule: Vec<String> = "67,x,7,59,61".split(",").map(|s| s.to_string()).collect();
    assert_eq!(part2(779200, &schedule), 779210);

    let schedule: Vec<String> = "67,7,x,59,61".split(",").map(|s| s.to_string()).collect();
    assert_eq!(part2(1261400, &schedule), 1261476);

    let schedule: Vec<String> = "1789,37,47,1889"
        .split(",")
        .map(|s| s.to_string())
        .collect();
    assert_eq!(part2(1202161400, &schedule), 1202161486);
}
