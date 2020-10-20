use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
}

#[derive(Debug, Clone)]
pub struct ParseRangeError;

impl fmt::Display for ParseRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "could not parse to range")
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bounds: Vec<&str> = s.split("..").collect();

        if bounds.len() != 2 {
            return Err(ParseRangeError);
        }

        let start = match bounds[0].parse::<usize>() {
            Ok(start) => Some(start),
            Err(_) => None,
        };
        let end = match bounds[1].parse::<usize>() {
            Ok(end) => Some(end),
            Err(_) => None,
        };

        if start.is_none() || end.is_none() {
            return Err(ParseRangeError);
        }

        Ok(Range {
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}
