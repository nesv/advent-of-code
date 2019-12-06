//! # Day 6: Universal Orbit Map
//!
//! You've landed at the Universal Orbit Map facility on Mercury.
//! Because navigation in space often involves transferring between orbits,
//! the orbit maps here are useful for finding efficient routes between,
//! for example, you and Santa.
//! You download a map of the local orbits (your puzzle input).
//!
//! Except for the universal Center of Mass (COM), every object in space is in
//! orbit around exactly one other object.
//! An [orbit](https://en.wikipedia.org/wiki/Orbit) looks roughly like this:
//!
//! ```ignore
//!                   \
//!                    \
//!                     |
//!                     |
//! AAA--> o            o <--BBB
//!                     |
//!                     |
//!                    /
//!                   /
//! ```
//!
//! In this diagram, the object `BBB` is in orbit around `AAA`.
//! The path that `BBB` takes around `AAA` (drawn with lines) is only partly
//! shown.
//! In the map data, this orbital relationship is written `AAA)BBB`, which means
//! "`BBB` is in orbit around `AAA`".
//!
//! Before you use your map data to plot a course, you need to make sure it
//! wasn't corrupted during the download.
//! To verify maps, the Universal Orbit Map facility uses orbit count
//! checksums - the total number of direct orbits (like the one shown above)
//! and indirect orbits.
//!
//! Whenever `A` orbits `B` and `B` orbits `C`, then `A` indirectly orbits `C`.
//! This chain can be any number of objects long:
//! if `A` orbits `B`, `B` orbits `C`, and `C` orbits `D`, then `A` indirectly
//! orbits `D`.
//!
//! For example, suppose you have the following map:
//!
//! ```ignore
//! COM)B
//! B)C
//! C)D
//! D)E
//! E)F
//! B)G
//! G)H
//! D)I
//! E)J
//! J)K
//! K)L
//! ```
//!
//! Visually, the above map of orbits looks like this:
//!
//! ```ignore
//!         G - H       J - K - L
//!        /           /
//! COM - B - C - D - E - F
//!                \
//!                 I
//! ```
//!
//! In this visual representation, when two objects are connected by a line,
//! the one on the right directly orbits the one on the left.
//!
//! Here, we can count the total number of orbits as follows:
//!
//! `D` directly orbits `C` and indirectly orbits `B` and `COM`,
//! a total of 3 orbits.
//! `L` directly orbits `K` and indirectly orbits `J`, `E`, `D`, `C`, `B`,
//! and `COM`, a total of 7 orbits.
//! `COM` orbits nothing.
//! The total number of direct and indirect orbits in this example is 42.
//!
//! **What is the total number of direct and indirect orbits in your map data?**

use aoc::Input;
use std::collections::HashMap;
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
    let lines = input.lines().unwrap();

    // Collect the orbits from the map as a series of associations, A->B.
    let assocs: Vec<Assoc<&str>> = lines
        .iter()
        .map(|ln| {
            let (planet, orbited_by) = parse_orbit(ln);
            Assoc(planet, orbited_by)
        })
        .collect();

    // Store the associations in a hashmap. (A->B)
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();
    for orbit in &assocs {
        if let Some(rels) = orbits.get_mut(orbit.0) {
            rels.push(orbit.1);
        } else {
            let v: Vec<&str> = vec![orbit.1];
            orbits.insert(orbit.0, v);
        }
    }
    // Count the number of direct, and indirect orbits by walking all
    // associations.
    let mut indirect = 0;
    let keys: Vec<&str> = orbits.keys().map(|s| *s).collect();
    for k in keys.iter() {
        indirect += follow(&orbits, k);
    }

    println!("{}", indirect);

    // Figure out how many orbital transfers are required between
    // "YOU" and "SAN".
    //
    // Start by storing the orbits in reverse association.
    let mut orbits_rev: HashMap<&str, &str> = HashMap::new();
    for orbit in &assocs {
        if let Some(o) = orbits_rev.insert(orbit.1, orbit.0) {
            panic!("{} is already orbiting {}", orbit.1, o)
        }
    }

    /*
     * Adapted from
     *  https://github.com/humantree/advent-of-code-2019/blob/master/orbit-map/src/main.rs
    let you_orbits: Vec<&str> = std::iter::successors(Some(&"YOU"), |next| orbits_rev.get(*next))
        .skip(1)
        .map(|v| *v)
        .collect();
    let san_orbits: Vec<&str> = std::iter::successors(Some(&"SAN"), |next| orbits_rev.get(*next))
        .skip(1)
        .map(|v| *v)
        .collect();
    let mut intersection = "";
    for o in &you_orbits {
        if san_orbits.contains(&o) {
            intersection = o;
        }
    }

    let you_xfers = &you_orbits.iter().position(|p| p == &intersection).unwrap();
    let san_xfers = &san_orbits.iter().position(|p| p == &intersection).unwrap();
    let transfers = you_xfers + san_xfers;
    */

    // Start at the planet we are orbiting.
    //
    // NOTE(nesv): This works on the example input, but not my actual puzzle
    // input.
    let start = orbits_rev.get("YOU").unwrap();
    let transfers = find_path(&orbits_rev, start, "SAN").expect("YOU -> SAN not found");
    println!("{}", transfers);

    Ok(())
}

/// Parses an orbit string of the form `A)B` (`B` orbits `A`), and returns a
/// tuple `("A", "B")`.
fn parse_orbit(s: &str) -> (&str, &str) {
    let sep = match s.find(")") {
        Some(n) => n,
        None => {
            panic!("no separator ')' in {}", s);
        }
    };
    (&s[..sep], &s[sep + 1..])
}

/// An association between two planets.
/// The first one is orbited by the second one.
#[derive(Debug)]
struct Assoc<T: Eq>(T, T);

fn follow(m: &HashMap<&str, Vec<&str>>, v: &str) -> usize {
    let vs: Vec<&str> = m.get(v).unwrap_or(&Vec::new()).to_vec();
    let n = vs.len();
    match n {
        0 => 0,
        1 => n + follow(m, vs[0]),
        _ => {
            let mut total = 0;
            for i in vs {
                total += follow(m, i) + 1;
            }
            total
        }
    }
}

/// Walk `m`, starting at `start` until a path to `until` is found.
/// Returns the shortest number of steps from start -> until.
fn find_path(m: &HashMap<&str, &str>, from: &str, to: &str) -> Option<i32> {
    if let Some(next) = m.get(from) {
        // If the next node is `to`, then we have found the endpoint.
        // We do not need to add anything to this, since we do not count the
        // last jump.
        if *next == to {
            return Some(0);
        }

        // We need to go one level further.
        if let Some(i) = find_path(m, next, to) {
            return Some(i + 1);
        }

        // We have reached a dead end, and need to backtrack.
        return Some(-1);
    }
    // We have reached a dead end.
    None
}
