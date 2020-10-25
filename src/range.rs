use std::fmt;
use std::str::FromStr;

use regex::Regex;

/// A struct to represent a range starting and ending at specific values
/// and increasing by a given step size.
///
/// The `end` value is exclusive.
///
/// Can be represented as a string `start..end,step_size` with the `,step_size`
/// part being option and can be parsed from a `Str` using `FromStr`.
pub struct Range {
    pub start: usize,
    pub end: usize,
    pub step_size: usize,
}

impl Range {
    pub fn new(start: usize, end: usize, step_size: usize) -> Self {
        Self {
            start,
            end,
            step_size,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns `start` if it is the only value in the `Range`.
    ///
    /// A `Range` contains only one value if the `end` value is one greater
    /// than the `start` value because the `end` value is exclusive.
    pub fn single(&self) -> Option<usize> {
        if self.end == self.start + 1 {
            Some(self.start)
        } else {
            None
        }
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
        // Only compile Regex once
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<start>[0-9]+)\.\.(?P<end>[0-9]+)(?:,(?P<step_size>[0-9]+))?$")
                    .unwrap();
        }

        if RE.is_match(s) {
            let caps = RE.captures(s).unwrap();

            let start = caps
                .name("start")
                .map_or("0", |c| c.as_str())
                .parse::<usize>()
                .map_err(|_| ParseRangeError)?;
            let end = caps
                .name("end")
                .map_or("0", |c| c.as_str())
                .parse::<usize>()
                .map_err(|_| ParseRangeError)?;
            let step_size = caps
                .name("step_size")
                .map_or("1", |c| c.as_str()) // Default step size is 1
                .parse::<usize>()
                .map_err(|_| ParseRangeError)?;

            Ok(Range::new(start, end, step_size))
        } else {
            return Err(ParseRangeError);
        }
    }
}
