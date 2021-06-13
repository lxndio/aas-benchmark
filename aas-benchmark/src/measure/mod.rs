pub mod csv_record;
pub mod measurement;
pub mod measurement_result;

use std::time::Duration;

use benchmark_lists::aslice::ASlice;

use self::measurement::SingleMeasurement;
use crate::cli::CLIParams;

/// Trait for implementing a measurement.
///
/// Some algorithms may only be measurable differently from the standard
/// algorithms which `fn measure` can measure. They may, for example,
/// require to measure a preparation and a matching function or generate
/// some other data first of which the time should not be measured.
pub trait Measure {
    fn measure(
        &self,
        pattern: &[u8],
        text: &mut ASlice<u8>,
        cli_params: &CLIParams,
    ) -> SingleMeasurement;
}

/// Trait for implementing a measurement that takes multiple patterns.
///
/// This is used for algorithms taking multiple patterns. Because of how this
/// program is structured, this was the easiest way to handle this, although not
/// the nicest one.
pub trait MultiplePatternMeasure {
    fn measure(
        &self,
        patterns: &[Vec<u8>],
        text: &[u8],
        cli_params: &CLIParams,
    ) -> SingleMeasurement;
}

/// A function to calculate the average duration of a `Vec<Duration>`
/// in milliseconds.
///
/// It takes a `Vec<Duration>` of multiple `Duration`s.
///
/// It returns the average duration in milliseconds as an `f64`.
pub fn calculate_avg_duration(durations: &[Duration]) -> f64 {
    if durations.is_empty() {
        return 0f64;
    }

    let sum: Duration = durations.iter().sum();
    let sum_nanos = sum.as_nanos() as f64;

    (sum_nanos / durations.len() as f64) / 1_000_000f64
}
