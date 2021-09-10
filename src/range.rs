use std::fmt;
use std::str::FromStr;

use regex::Regex;

/// A struct to represent a range starting and ending at specific values
/// and increasing by a given step size.
///
/// The `end` value is inclusive.
///
/// Can be represented as a string `start..end,step_size` with the `,step_size`
/// part being option and can be parsed from a `Str` using `FromStr`.
#[derive(Debug)]
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
        self.start > self.end
    }

    /// Returns `start` if it is the only value in the `Range`.
    pub fn single(&self) -> Option<usize> {
        if self.end == self.start {
            Some(self.start)
        } else {
            None
        }
    }

    pub fn iter(&self) -> RangeIterator {
        RangeIterator::from_range(&self)
    }

    pub fn is_valid(&self) -> bool {
        if self.is_empty() {
            return false;
        }

        if let Some(length) = self.single() {
            if length == 0 {
                return false;
            }
        }

        if self.step_size == 0 {
            return false;
        }

        true
    }
}

impl Default for Range {
    #[cfg(not(tarpaulin_include))]
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end && self.step_size == other.step_size
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
            static ref RE_SINGLE: Regex = Regex::new(r"^(?P<single>[0-9]+)?$").unwrap();
        }

        // Try parsing the input as a Range, if that failed try parsing it
        // as a positive integer to make a single value range,
        // otherwise return an error
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
        } else if RE_SINGLE.is_match(s) {
            let caps = RE_SINGLE.captures(s).unwrap();

            let single = caps
                .name("single")
                .map_or("0", |c| c.as_str())
                .parse::<usize>()
                .map_err(|_| ParseRangeError)?;

            Ok(Range::new(single, single + 1, 1))
        } else {
            Err(ParseRangeError)
        }
    }
}

pub struct RangeIterator {
    curr: usize,
    next: usize,
    step_size: usize,
    end: usize,
}

impl RangeIterator {
    pub fn from_range(range: &Range) -> Self {
        RangeIterator {
            curr: range.start,
            next: range.start,
            step_size: range.step_size,
            end: range.end,
        }
    }
}

impl Iterator for RangeIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.next + self.step_size;

        self.curr = self.next;
        self.next = new_next;

        if self.curr <= self.end {
            Some(self.curr)
        } else {
            None
        }
    }
}

#[allow(unused)]
macro_rules! range {
    ($left:literal..$right:literal) => {
        Range::new($left, $right, 1)
    };
    ($left:literal..$right:literal, $step_size:literal) => {
        Range::new($left, $right, $step_size)
    };
    ($single:literal) => {
        Range::new($single, $single + 1, 1)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_is_empty() {
        let empty_range1 = Range::new(5, 4, 5);
        let empty_range2 = Range::new(10, 5, 5);

        let nonempty_range1 = Range::new(0, 5, 2);
        let nonempty_range2 = Range::new(5, 10, 0);
        let nonempty_range3 = Range::new(5, 5, 1);

        assert_eq!(empty_range1.is_empty(), true);
        assert_eq!(empty_range2.is_empty(), true);

        assert_eq!(nonempty_range1.is_empty(), false);
        assert_eq!(nonempty_range2.is_empty(), false);
        assert_eq!(nonempty_range3.is_empty(), false);
    }

    #[test]
    fn test_range_is_single() {
        let single_range = Range::new(1, 1, 1);

        let nonsingle_range = Range::new(1, 2, 1);

        assert_eq!(single_range.single(), Some(1));

        assert_eq!(nonsingle_range.single(), None);
    }

    #[test]
    fn test_range_iterator() {
        let range_single_step = Range::new(5, 10, 1);
        let range_multiple_step = Range::new(5, 50, 5);

        assert_eq!(
            range_single_step.iter().collect::<Vec<usize>>(),
            vec![5, 6, 7, 8, 9, 10]
        );
        assert_eq!(
            range_multiple_step.iter().collect::<Vec<usize>>(),
            vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]
        );
    }

    #[test]
    fn test_range_eq() {
        let range1 = Range::new(0, 10, 1);
        let range2 = Range::new(0, 10, 1);
        let range3 = Range::new(5, 10, 1);

        assert_eq!(range1, range2);
        assert_eq!(range2, range1);

        assert_ne!(range1, range3);
        assert_ne!(range3, range1);
        assert_ne!(range1, range3);
        assert_ne!(range3, range2);
    }

    #[test]
    fn test_parse_range_from_str() {
        let valid_strs = vec![
            "1..2",
            "1..15",
            "10..25",
            "10..5",
            "1..2,5",
            "10..25,253",
            "42",
        ];
        let invalid_strs = vec!["-5..-20", "7..", "..10", ",15", "20,25", "-42"];

        let valid_ranges = vec![
            range!(1..2),
            range!(1..15),
            range!(10..25),
            range!(10..5),
            range!(1..2, 5),
            range!(10..25, 253),
            range!(42),
        ];

        for (valid_str, valid_range) in valid_strs.iter().zip(valid_ranges) {
            assert_eq!(Range::from_str(valid_str).unwrap(), valid_range);
        }

        for invalid_str in invalid_strs.iter() {
            assert!(Range::from_str(invalid_str).is_err());
        }
    }
}
