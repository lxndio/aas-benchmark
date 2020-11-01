pub mod csv_record;
pub mod measure_result;

use std::time::{Duration, SystemTime};

use crate::measure::measure_result::MeasureResult;

/// A function to measure the runtime of an algorithm.
///
/// It takes a `pattern` and a `text` and executes a function `f` using
/// the standard signature of the pattern matching algorithms
/// `(&[u8], &[u8]) -> Vec<usize>`.
///
/// It returns a `Duration`, the runtime of the execution given function.
pub fn measure(
    pattern: &[u8],
    text: &[u8],
    f: fn(&[u8], &[u8]) -> Vec<usize>,
) -> (Duration, usize) {
    let before = SystemTime::now();

    let matches = f(pattern, text).len();

    let duration = before.elapsed();

    (duration.expect("Could not measure time."), matches)
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
    f: fn(&[u8], &[u8]) -> Vec<usize>,
    n: usize,
) -> (Vec<Duration>, usize) {
    let mut durations: Vec<Duration> = Vec::new();

    for _ in 0..n {
        durations.push(measure(pattern, text, f).0);
    }

    // Measure once again to get number of matches
    let matches = measure(pattern, text, f).1;

    (durations, matches)
}

/// Measures the runtimes of multiple executions of an algorithm
/// using a different patterns.
///
/// It measures the algorithm `n` times using each of the `patterns`.
///
/// It returns a `Vec<MeasureResult>` containing the results of this measurement.
pub fn measure_multiple_different_patterns(
    algorithm: &str,
    patterns: &Vec<Vec<u8>>,
    text: &[u8],
    f: fn(&[u8], &[u8]) -> Vec<usize>,
    n: usize,
) -> Vec<MeasureResult> {
    let mut measure_results: Vec<MeasureResult> = Vec::new();

    for pattern in patterns {
        let (durations, matches) = measure_multiple(pattern, text, f, n);

        measure_results.push(MeasureResult::new(
            &algorithm,
            text.len(),
            pattern.len(),
            matches,
            durations,
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
pub fn calculate_avg_duration(durations: &Vec<Duration>) -> f64 {
    if durations.len() == 0 {
        return 0f64;
    }

    let sum: Duration = durations.iter().sum();
    let sum_nanos = sum.as_nanos() as f64;

    (sum_nanos / durations.len() as f64) / 1_000_000f64
}
