use std::time::SystemTime;

use crate::algorithms::full_text_indices::suffix_array::slow;
use crate::match_algorithm::SlowSuffixArrayAlgorithm;
use crate::measure::{Measurement, SingleMeasurement};

/// Measurement for slow suffix array algorithms as the preparation time,
/// i. e. the time it takes to generate the suffix array, should also
/// be measured.
///
/// The **slow** suffix array algorithms measurement uses the slow approach
/// to generating the suffix array.
impl Measurement for SlowSuffixArrayAlgorithm {
    fn measure(pattern: &[u8], text: &[u8], f: &Self) -> SingleMeasurement {
        // Add sentinel to text
        let mut text = text.iter().map(|x| *x).collect::<Vec<u8>>();
        text.push(0);
        let text = text.as_slice();

        // Measure time it takes to generate the suffix array
        let before = SystemTime::now();

        let pos = slow(text);

        let preparation_duration = before.elapsed();

        // Measure time it takes to run the actual algorithm
        let before = SystemTime::now();

        let matches = f(pos, pattern, text).len();

        let algorithm_duration = before.elapsed();

        (
            Some(preparation_duration.expect("Could not measure preparation time.")),
            algorithm_duration.expect("Could not measure time."),
            matches,
        )
    }
}

pub fn match_pattern(pos: Vec<usize>, pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();

    // let l: usize = pos.iter().position(|x| less_eq_m(pattern, &text[*x..], m)).unwrap_or(n);
    // let r: isize = pos.iter().rev().position(|x| greater_eq_m(pattern, &text[*x..], m)).map(|x| x as isize).unwrap_or(-1);

    let mut ls: Vec<usize> = (0..n)
        .filter(|r| less_eq_m(pattern, &text[pos[*r]..], m))
        .collect();
    ls.push(n);

    let mut rs: Vec<isize> = (0..n)
        .filter(|r| greater_eq_m(pattern, &text[pos[*r]..], m))
        .map(|x| x as isize)
        .collect();
    rs.push(-1);

    let l = *ls.iter().min().unwrap();
    let r = *rs.iter().max().unwrap();

    if r >= l as isize {
        (l..=r as usize).map(|x| pos[x]).collect()
    } else {
        Vec::new()
    }
}

fn less_eq_m(a: &[u8], b: &[u8], m: usize) -> bool {
    let mut m = m;
    if a.len().min(b.len()) < m {
        m = a.len().min(b.len());
    }

    a[0..m] <= b[0..m]
}

fn greater_eq_m(a: &[u8], b: &[u8], m: usize) -> bool {
    let mut m = m;
    if a.len().min(b.len()) < m {
        m = a.len().min(b.len());
    }

    a[0..m] >= b[0..m]
}
