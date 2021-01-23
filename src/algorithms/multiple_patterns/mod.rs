pub mod aho_corasick;
pub mod naive;

use std::time::SystemTime;

use crate::cli::CLIParams;
use crate::match_algorithm::MultiplePatternAlgorithm;
use crate::measure::measurement::SingleMeasurement;
use crate::measure::MultiplePatternMeasure;

impl MultiplePatternMeasure for MultiplePatternAlgorithm {
    /// A function to measure the runtime of an algorithm.
    ///
    /// It takes a `pattern` and a `text` and executes a function `f` using
    /// the standard signature of the pattern matching algorithms
    /// `(&[u8], &[u8]) -> Vec<usize>`.
    ///
    /// It returns a `Duration`, the runtime of the execution given function.
    #[cfg(not(tarpaulin_include))]
    fn measure(&self, patterns: &Vec<Vec<u8>>, text: &[u8], _: &CLIParams) -> SingleMeasurement {
        let before = SystemTime::now();

        let matches = self(patterns, text).len();

        let duration = before.elapsed();

        // Because these algorithms do not have a preparation phase the runtime
        // of which could be measured, the first value is simply None
        (None, duration.expect("Could not measure time."), matches)
    }
}
