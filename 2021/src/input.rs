use std::fs::File;
use std::io::{prelude::*, Error, ErrorKind, Result};
use std::path::Path;

/// Input can be used to hold, or load puzzle input.
pub struct Input {
    data: Option<String>,
}

impl Input {
    /// Return a new Input holding the data in `s`.
    ///
    /// Refer to the other methods, such as `lines` for getting at the data.
    pub fn new(s: String) -> Result<Self> {
        if s.len() == 0 {
            return Err(Error::new(ErrorKind::Other, "zero-length input"));
        }
        Ok(Self { data: Some(s) })
    }

    pub fn new_from_str(s: &str) -> Result<Self> {
        Self::new(s.to_string())
    }

    /// Load the puzzle input from the file at the specified `path`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut buffer = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buffer)?;
        Ok(Self { data: Some(buffer) })
    }

    /// Returns each line of the puzzle input, without removing any empty
    /// lines.
    pub fn raw_lines(&self) -> Option<Vec<String>> {
        if let Some(d) = &self.data {
            let v: Vec<String> = d.split("\n").map(|s| s.to_string()).collect();
            return Some(v);
        }
        None
    }

    /// Returns the puzzle input, split into lines with the newline characters
    /// removed.
    /// You may need to do additional parsing on the elements in the returned
    /// `Vec<String>` to make the puzzle input useful.
    pub fn lines(&self) -> Option<Vec<String>> {
        if let Some(d) = &self.data {
            let v: Vec<String> = d
                .split("\n")
                .filter(|s| s.len() != 0)
                .map(|s| s.to_string())
                .collect();
            return Some(v);
        }
        None
    }

    /// Returns the puzzle input split on the given pattern `pat`, after it has
    /// been split into lines.
    pub fn split_lines(&self, pat: &str) -> Option<Vec<Vec<String>>> {
        let mut v: Vec<Vec<String>> = vec![];
        for line in self.lines()? {
            v.push(line.split(pat).map(|s| s.to_string()).collect());
        }
        Some(v)
    }

    /// Returns the input data as a `String`.
    pub fn to_string(&self) -> Option<String> {
        if let Some(d) = &self.data {
            return Some(String::from(d.trim()));
        }
        None
    }

    /// Returns the input data as a `Vec<isize>`, assuming there is one
    /// number per line.
    pub fn numbers(&self) -> Option<Vec<isize>> {
        if let Some(lines) = &self.lines() {
            let v: Vec<isize> = lines
                .iter()
                .map(|ln| match ln.parse::<isize>() {
                    Ok(n) => n,
                    Err(e) => {
                        panic!("parse {:?}: {}", ln, e);
                    }
                })
                .collect();
            return Some(v);
        }
        None
    }
}

impl From<&str> for Input {
    fn from(s: &str) -> Self {
        Self {
            data: Some(s.to_string()),
        }
    }
}

impl From<String> for Input {
    fn from(s: String) -> Self {
        Self { data: Some(s) }
    }
}
