use aoc::{
    day10::{Angle, Map, Position},
    Input,
};
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
    let asteroids = map.asteroids();

    let (num_asteroids, asteroid) = part1(&asteroids);
    println!("{:?} => {}", &asteroid, num_asteroids);

    let stop_at = 200;
    let destroyed = part2(&asteroid, &asteroids, stop_at);
    println!("{}", destroyed);

    Ok(())
}


fn part1(asteroids: &Vec<Position>) -> (usize, Position) {
    use std::collections::HashSet;

    asteroids
        .iter()
        .map(|pos| {
            let z: HashSet<_> = asteroids
                .iter()
                .filter(|&p| p != pos)
                .map(|p| {
                    let x = pos.x - p.x;
                    let y = pos.y - p.y;
                    Angle::new(x, y)
                })
                .collect();
            (z.len(), *pos)
        })
        .max_by_key(|(count, _)| *count)
        .unwrap_or_default()
}

fn part2(station: &Position, asteroids: &Vec<Position>, stop_at: usize) -> usize {
    use std::collections::BTreeMap;

    let mut directions: BTreeMap<Angle, Vec<Position>> = BTreeMap::new();
    let positions = asteroids
        .iter()
        .filter(|&&p| p != *station)
        .map(|p| (Angle::new(p.x - station.x, p.y - station.y), p));
    for (angle, position) in positions {
        directions.entry(angle).or_default().push(*position);
    }

    for line in directions.values_mut() {
        line.sort_by_key(|p| -(p.x - station.x).abs() - (p.y - station.y).abs());
    }

    let mut i = 0;
    loop {
        for line in directions.values_mut() {
            if let Some(pos) = line.pop() {
                i += 1;
                if i == stop_at {
                    return (pos.x * 100 + pos.y) as usize;
                }
            }
        }
    }
}

#[test]
fn test_part1_a() {
    let input = Input::new_from_str(r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#).unwrap();
    let map = Map::from(input.lines().unwrap());
    println!("{}", &map);

    let (x, y) = map.size();
    println!("Size: {} x {}", x, y);

    let asteroids = map.asteroids();
    let (visible, pos) = part1(&asteroids);
    assert_eq!(visible, 33);
    assert_eq!(pos, Position { x: 5, y: 8});
}

#[test]
fn test_part1_b() {
    let input = Input::new_from_str(r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#).unwrap();
    let map = Map::from(input.lines().unwrap());
    let asteroids = map.asteroids();
    let (visible, position) = part1(&asteroids);
    assert_eq!(visible, 35);
    assert_eq!(position, Position{ x: 1, y: 2 });
}
