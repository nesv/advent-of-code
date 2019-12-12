use std::fmt;
use std::iter::successors;

#[derive(Clone)]
pub struct Map {
    elements: Vec<Vec<Element>>,
}

impl From<Vec<String>> for Map {
    fn from(v: Vec<String>) -> Self {
        let mut elements: Vec<Vec<Element>> = vec![];
        for (y, line) in v.iter().enumerate() {
            let mut elts: Vec<Element> = vec![];
            for (x, c) in line.chars().enumerate() {
                elts.push(match c {
                    '.' => Element::Nothing(x, y),
                    '#' => Element::Asteroid(x, y),
                    _ => Element::Unknown(x, y),
                });
            }
            elements.push(elts);
        }
        Self { elements }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.elements.iter().enumerate() {
            for e in row {
                write!(f, "{}", e)?;
            }
            if i == self.elements.len() - 1 {
                break;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Map {
    /// Returns the size of the map, as an `(x, y)` tuple.
    pub fn size(&self) -> (usize, usize) {
        let y = self.elements.len();
        if y > 0 {
            return (self.elements[0].len(), y);
        }
        (0, 0)
    }

    /// Returns a vector containing the `Position`s of each asteroid on the map.
    pub fn asteroids(&self) -> Vec<Position> {
        let mut v: Vec<Position> = vec![];
        for line in &self.elements {
            for elt in line {
                match elt {
                    Element::Asteroid(x, y) => {
                        v.push(Position{ x: *x as i32, y: *y as i32 });
                    }
                    _ => {}
                }
            }
        }
        v
    }

    /// Returns an `Element` at the given `x` and `y` positions, on the map.
    /// If `x` or `y` extend past the outer boundaries of the map, `at` will
    /// return `None`.
    pub fn at(&self, x: usize, y: usize) -> Option<&Element> {
        if y >= self.elements.len() {
            return None;
        }
        if x >= self.elements[y].len() {
            return None;
        }
        Some(&self.elements[y][x])
    }

    /// Takes the path from `first` to `second`, and continues walking along
    /// that path, collecting all `Element`s along the way, including `second`.
    pub fn along_path(&self, first: &Element, second: &Element) -> Vec<Element> {
        let (dx, dy) = first.distance_from(second);
        let elements: Vec<Element> = successors(Some(second), |next| {
            let (x, y) = match next {
                Element::Unknown(x, y) => ((*x as i32 + dx) as usize, (*y as i32 + dy) as usize),
                Element::Nothing(x, y) => ((*x as i32 + dx) as usize, (*y as i32 + dy) as usize),
                Element::Asteroid(x, y) => ((*x as i32 + dx) as usize, (*y as i32 + dy) as usize),
            };
            self.at(x, y)
        })
        .map(|e| e.clone())
        .collect();
        elements
    }

    /// Cast a ray in a particular direction, from `start`, and returns the
    /// first asteroid on that path.
    /// If there are no asteroids on that path, `cast_ray` will return `None`.
    pub fn cast_ray(&self, start: &Element, dx: i32, dy: i32) -> Option<Element> {
        // Never cast past these boundaries.
        let (min_x, min_y) = (0, 0);
        let (max_x, max_y) = self.size();
        let max_x = max_x as i32;
        let max_y = max_y as i32;

        let (ax, ay) = start.coords();
        let mut ax = ax as i32;
        let mut ay = ay as i32;

        loop {
            // Calculate the next point we are going to look at, which is
            // effectively calculating the slope of our line/ray.
            //
            // If our next point is going to reach outside of our boundaries,
            // return early.
            let bx = ax + dx;
            if bx < min_x || bx >= max_x {
                return None;
            }

            let by = ay + dy;
            if by < min_y || by >= max_y {
                return None;
            }

            match self.at(bx as usize, by as usize).unwrap() {
                Element::Asteroid(x, y) => { return Some(Element::Asteroid(*x, *y)); }
                Element::Nothing(x, y) => {
                    ax = *x as i32;
                    ay = *y as i32;
                }
                _ => { panic!("unexpected element"); }
            }
        }
    }
}

#[derive(Copy, Clone, Hash, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Element {
    Unknown(usize, usize),
    Nothing(usize, usize),
    Asteroid(usize, usize),
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl Element {
    /// Returns the relative distance from another element.
    pub fn distance_from(&self, other: &Element) -> (i32, i32) {
        let (x, y) = match self {
            Self::Asteroid(x, y) => (*x as i32, *y as i32),
            Self::Nothing(x, y) => (*x as i32, *y as i32),
            Self::Unknown(x, y) => (*x as i32, *y as i32),
        };
        let (ox, oy) = match other {
            Self::Asteroid(x, y) => (*x as i32, *y as i32),
            Self::Nothing(x, y) => (*x as i32, *y as i32),
            Self::Unknown(x, y) => (*x as i32, *y as i32),
        };
        (ox - x, oy - y)
    }

    pub fn as_char(&self) -> char {
        match self {
            Self::Asteroid(_, _) => '#',
            Self::Nothing(_, _) => '.',
            Self::Unknown(_, _) => '?',
        }
    }

    pub fn coords(&self) -> (usize, usize) {
        match self {
            Self::Asteroid(x, y) => (*x, *y),
            Self::Nothing(x, y) => (*x, *y),
            Self::Unknown(x, y) => (*x, *y),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Angle {
    xsign: i32,
    ratio: num::rational::Ratio<i32>,
}

impl Angle {
    pub fn new(x: i32, y: i32) -> Self {
        let xsign = x.signum();
        let ysign = y.signum();
        let ratio = if xsign == 0 {
            num::rational::Ratio::new(ysign, 1)
        } else {
            num::rational::Ratio::new(y, x)
        };

        Angle { xsign, ratio }
    }
}


impl std::cmp::PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

use std::cmp::Ordering;

impl std::cmp::Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        if self.xsign == 0 && self.ratio.numer() < &0 {
            return Ordering::Less;
        }
        if other.xsign == 0 && other.ratio.numer() < &0 {
            return Ordering::Greater;
        }
        if self.xsign > 0 && other.xsign <= 0 {
            return Ordering::Less;
        }
        if other.xsign > 0 && self.xsign <= 0 {
            return Ordering::Greater;
        }
        if self.xsign > 0 && other.xsign > 0 {
            return self.ratio.cmp(&other.ratio);
        }
        if self.xsign == 0 {
            return Ordering::Less;
        }
        if other.xsign == 0 {
            return Ordering::Greater;
        }
        self.ratio.cmp(&other.ratio)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position{ x, y }
    }
}
