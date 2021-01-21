pub mod error_tolerant_shift_and;
pub mod ukkonen;

use std::time::SystemTime;

use crate::cli::CLIParams;
use crate::match_algorithm::ApproximativeAlgorithm;
use crate::measure::{Measure, SingleMeasurement};

impl Measure for ApproximativeAlgorithm {
    /// A function to measure the runtime of an approximative algorithm.
    ///
    /// It takes the maximum allowed error from the given CLI parameters.
    #[cfg(not(tarpaulin_include))]
    fn measure(pattern: &[u8], text: &[u8], f: &Self, cli_params: &CLIParams) -> SingleMeasurement {
        let before = SystemTime::now();

        // Unwrapping the `maximum_error` CLI parameter is valid here
        // because it can't be None as checked in `cli::valid()`
        let matches = f(pattern, text, cli_params.maximum_error.unwrap()).len();

        let duration = before.elapsed();

        // Because these algorithms do not have a preparation phase the runtime
        // of which could be measured, the first value is simply None
        (None, duration.expect("Could not measure time."), matches)
    }
}
