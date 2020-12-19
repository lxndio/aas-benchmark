use std::time::SystemTime;

use crate::cli::CLIParams;
use crate::match_algorithm::ApproximativeAlgorithm;
use crate::measure::{Measurement, SingleMeasurement};

pub mod ukkonen;

impl Measurement for ApproximativeAlgorithm {
    /// A function to measure the runtime of an approximative algorithm.
    ///
    /// It takes the allowed maximum error from the given CLI parameters.
    #[cfg(not(tarpaulin_include))]
    fn measure(pattern: &[u8], text: &[u8], f: &Self, _: &CLIParams) -> SingleMeasurement {
        let before = SystemTime::now();

        let matches = f(pattern, text, 1).len();

        let duration = before.elapsed();

        // Because these algorithms do not have a preparation phase the runtime
        // of which could be measured, the first value is simply None
        (None, duration.expect("Could not measure time."), matches)
    }
}
