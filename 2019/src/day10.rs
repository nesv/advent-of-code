use std::fmt;
use std::iter::successors;

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
        for row in &self.elements {
            for e in row {
                write!(f, "{}", e)?;
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

    /// Returns a vector containing only the `Element::Asteroid` elements from
    /// the map.
    pub fn asteroids(&self) -> Vec<Element> {
        let mut v: Vec<Element> = vec![];
        for line in &self.elements {
            for elt in line {
                match elt {
                    Element::Asteroid(_, _) => {
                        v.push(*elt);
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
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq, PartialOrd)]
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
