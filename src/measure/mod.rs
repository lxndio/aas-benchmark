pub mod csv_record;
pub mod measure_result;

use std::time::Duration;

use crate::cli::CLIParams;
use crate::match_algorithm::{
    ApproximativeAlgorithm, BWTAlgorithm, SinglePatternAlgorithm, SuffixArrayAlgorithm,
    TypedAlgorithm,
};
use crate::measure::measure_result::MeasureResult;

/// A single measurement containing an optional preparation runtime,
/// a mandatory execution runtime (of the actual pattern matching algorithm
/// itself) and the number of matches, i. e. how often the pattern has been
/// found in the text.
pub type SingleMeasurement = (Option<Duration>, Duration, usize);

/// Trait for implementing a measurement.
///
/// Some algorithms may only be measurable differently from the standard
/// algorithms which `fn measure` can measure. They may, for example,
/// require to measure a preparation and a matching function or generate
/// some other data first of which the time should not be measured.
pub trait Measure {
    fn measure(pattern: &[u8], text: &[u8], f: &Self, cli_params: &CLIParams) -> SingleMeasurement;
}

/// A function to measure the runtimes of multiple executions of an algorithm.
///
/// It takes a `pattern` and a `text` and executes a function `f` using
/// the standard signature of the pattern matching algorithms
/// `(&[u8], &[u8]) -> Vec<usize>`.
///
/// It returns a `(Vec<Duration>, usize)`, the runtimes of the exeuctions
/// of the given functions and the number of matches.
pub fn measure_multiple(
    pattern: &[u8],
    text: &[u8],
    f: &TypedAlgorithm,
    cli_params: &CLIParams,
) -> Vec<SingleMeasurement> {
    let mut single_measurements: Vec<SingleMeasurement> = Vec::new();

    for _ in 0..cli_params.executions {
        single_measurements.push(match f {
            TypedAlgorithm::SinglePatternAlgorithm(f) => {
                SinglePatternAlgorithm::measure(pattern, text, f, cli_params)
            }
            TypedAlgorithm::MultiplePatternAlgorithm(_) => {
                unimplemented!();
            }
            TypedAlgorithm::SuffixArrayAlgorithm(f) => {
                SuffixArrayAlgorithm::measure(pattern, text, f, cli_params)
            }
            TypedAlgorithm::BWTAlgorithm(f) => BWTAlgorithm::measure(pattern, text, f, cli_params),
            TypedAlgorithm::ApproximativeAlgorithm(f) => {
                ApproximativeAlgorithm::measure(pattern, text, f, cli_params)
            }
        });
    }

    single_measurements
}

/// Measures the runtimes of multiple executions of an algorithm
/// using a different patterns.
///
/// It measures the algorithm `n` times using each of the `patterns`.
///
/// It returns a `Vec<MeasureResult>` containing the results of this measurement.
pub fn measure_multiple_different_patterns(
    algorithm: &str,
    patterns: &[Vec<u8>],
    text: &[u8],
    f: &TypedAlgorithm,
    cli_params: &CLIParams,
) -> Vec<MeasureResult> {
    let mut measure_results: Vec<MeasureResult> = Vec::new();

    for pattern in patterns {
        let measurements = measure_multiple(pattern, text, f, cli_params);

        // TODO make this nicer in the future
        //let preparation_durations: Vec<Duration>;
        //if !measurements.iter().map(|m| m.0).filter(|d| d.is_none()).collect::<Vec<Option<Duration>>>().is_empty() {
        let preparation_durations: Vec<Option<Duration>> =
            measurements.iter().map(|m| m.0).collect();
        //} else {
        //    preparation_durations = vec![0; measurements.len()];
        //}

        let algorithm_durations: Vec<Duration> = measurements.iter().map(|m| m.1).collect();
        let matches: usize = measurements.get(0).unwrap().2;

        measure_results.push(MeasureResult::new(
            &algorithm,
            text.len(),
            pattern.len(),
            matches,
            preparation_durations,
            algorithm_durations,
        ));
    }

    measure_results
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
