use aoc::Input;
use std::fmt;
use std::io::{Error, ErrorKind, Result};
use std::iter;
use std::str::FromStr;

fn main() -> Result<()> {
    let in_file = match std::env::args().nth(1) {
        Some(v) => v,
        None => {
            return Err(Error::new(ErrorKind::InvalidInput, "no input file specified"));
        }
    };
    let input = Input::from_file(in_file)?;
    let moons: Vec<Moon> = input.lines()
        .unwrap()
        .iter()
        .map(|ln| Moon::from_str(ln).expect("failed to parse moon"))
        .collect();

    let n = 1000;
    let total_energy = run_simulation(moons, n);
    println!("Total energy after {} steps: {}", n, total_energy);

    Ok(())
}

/// Run the simulation `n` times, given the lunar positions.
/// Returns the total energy in the system after `n` iterations.
fn run_simulation(moons: Vec<Moon>, n: usize) -> i32 {
    let mut positions = moons.clone();
    let mut velocities: Vec<Velocity> = moons.iter().map(|_| Velocity { x: 0, y: 0, z: 0 }).collect();

    /*
    eprintln!("After 0 steps:");
    for (_i, (pos, vel)) in positions.iter().zip(velocities.iter()).enumerate() {
        eprintln!("pos={}, vel={}", pos, vel);
    }
    eprintln!("");
    */

    let mut step = 0;
    loop {
        eprintln!("After {} steps:", &step);

        let current_positions = &positions.clone();
        let current_velocities = &velocities.clone();
        let mut energy = vec![];

        let mut it = current_positions.iter().zip(current_velocities.iter());
        for (i, (pos, &vel)) in it.enumerate() {
            eprintln!("pos={}, vel={}", pos, vel);

            let gravity = positions.iter()
                .filter(|p| *p != pos)
                .map(|p| pos.gravity(p))
                .sum();

            let mut new_pos = pos.clone();
            new_pos.add_velocity(vel);
            positions[i] = new_pos;

            velocities[i] = v;
            energy.push(pos.potential_energy() * vel.kinetic_energy());
        }

        if step == n {
            return energy.iter().sum();
        }
        step += 1;

        eprintln!("");
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
}

impl Moon {
    fn add_velocity(&mut self, v: Velocity) {
        *self = Moon {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        };
    }

    fn gravity(&self, other: &Moon) -> Velocity {
        if self == other {
            return Velocity { x: 0, y: 0, z: 0 };
        }

        use std::cmp::Ordering;
        Velocity {
            x: match self.x.cmp(&other.x) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
            y: match self.y.cmp(&other.y) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
            z: match self.z.cmp(&other.z) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            },
        }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl FromStr for Moon {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '<' || p == '>')
            .split(", ")
            .collect();

        let (mut x, mut y, mut z) = (0, 0, 0);
        for c in coords {
            let sep = c.find('=').expect("cannot find seperator");
            let dim: &str = &c[..sep];
            match dim {
                "x" => { x = c[sep+1..].parse::<i32>().unwrap(); }
                "y" => { y = c[sep+1..].parse::<i32>().unwrap(); }
                "z" => { z = c[sep+1..].parse::<i32>().unwrap(); }
                _ => { panic!("unexpected dimension: {}", dim); }
            }
        }

        Ok(Self{ x, y, z })
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Velocity {
    x: i32,
    y: i32,
    z: i32,
}

impl Velocity {
    fn kinetic_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x={}, y={}, z={}>", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Velocity {
    type Output = Velocity;

    fn add(self, other: Velocity) -> Velocity {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Velocity {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl iter::Sum<Velocity> for Velocity {
    fn sum<I>(iter: I) -> Self
    where
        I: iter::Iterator<Item = Velocity>,
    {
        let mut result = Velocity { x: 0, y: 0, z: 0 };
        for v in iter {
            result += v;
        }
        result
    }
}

#[test]
fn test_simulation_1() {
    let input = Input::from(r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#);
    let moons: Vec<Moon> = input.lines()
        .unwrap()
        .iter()
        .map(|ln| Moon::from_str(ln).expect("failed to parse moon"))
        .collect();

    assert_eq!(run_simulation(moons, 10), 179);
}

#[test]
fn test_simulation_2() {
    let input = Input::from(r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#);
    let moons: Vec<Moon> = input.lines().unwrap().iter()
        .map(|ln| Moon::from_str(ln).expect("failed to parse moon"))
        .collect();
    assert_eq!(run_simulation(moons, 1000), 1940);
}
