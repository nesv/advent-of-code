use aoc::{
    day10::{Element, Map},
    Input,
};
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

    let map = Map::from(input.lines().unwrap());

    // Holds an Element::Asteroid, and how many other things it has direct
    // line of sight to.
    let mut hash: HashMap<Element, Vec<Vec<Element>>> = HashMap::new();
    for (i, a) in map.asteroids().iter().enumerate() {
        let rest = map.asteroids().to_vec();
        let paths: Vec<Vec<Element>> = vec![];
        for (j, b) in rest.iter().enumerate() {
            // Skip the current iteration if we are looking at the same element.
            if i == j {
                continue;
            }
            // Collect everything along the path from a->b.
            let elements: Vec<Element> = map.along_path(a, &b).iter().filter(|e| match {
                Element::Asteroid(_,_) => true,
                _ => false,
            }).collect();
            paths.push(elements);
        }
        hash.insert(a, paths);
    }

    for (asteroid, paths) in &hash {

    }

    Ok(())
}
