pub mod aho_corasick;
pub mod naive;

use std::time::SystemTime;

use crate::cli::CLIParams;
use crate::count_accesses::{reset_text_access_counter, text_access_counter};
use crate::count_comparisons::{comparison_counter, reset_comparison_counter};
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
    fn measure(&self, patterns: &[Vec<u8>], text: &[u8], _: &CLIParams) -> SingleMeasurement {
        reset_comparison_counter();
        reset_text_access_counter();
        let before = SystemTime::now();

        let matches = self(patterns, text).len();

        let duration = before.elapsed();
        let comparisons = comparison_counter();
        let text_accesses = text_access_counter();

        // Because these algorithms do not have a preparation phase the runtime
        // of which could be measured, the first value is simply None
        SingleMeasurement::new(
            None,
            duration.expect("Could not measure time."),
            matches,
            hashmap! {
                "comparisons" => comparisons,
                "text_accesses" => text_accesses,
            },
        )
    }
}
