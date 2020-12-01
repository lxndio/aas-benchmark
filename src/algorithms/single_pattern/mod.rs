pub mod all;
pub mod bndm;
pub mod horspool;
pub mod kmp;
pub mod naive;
pub mod shift_and;

use std::time::SystemTime;

use crate::match_algorithm::SinglePatternAlgorithm;
use crate::measure::{Measurement, SingleMeasurement};

impl Measurement for SinglePatternAlgorithm {
    /// A function to measure the runtime of an algorithm.
    ///
    /// It takes a `pattern` and a `text` and executes a function `f` using
    /// the standard signature of the pattern matching algorithms
    /// `(&[u8], &[u8]) -> Vec<usize>`.
    ///
    /// It returns a `Duration`, the runtime of the execution given function.
    #[cfg(not(tarpaulin_include))]
    fn measure(pattern: &[u8], text: &[u8], f: &SinglePatternAlgorithm) -> SingleMeasurement {
        let before = SystemTime::now();

        let matches = f(pattern, text).len();

        let duration = before.elapsed();

        // Because these algorithms do not have a preparation phase the runtime
        // of which could be measured, the first value is simply None
        (None, duration.expect("Could not measure time."), matches)
    }
}
