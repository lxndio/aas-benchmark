use std::time::{Duration, SystemTime};

/// A function to measure the runtime of an algorithm.
///
/// It takes a `pattern` and a `text` and executes a function `f` using
/// the standard signature of the pattern matching algorithms
/// `(&[u8], &[u8]) -> Vec<usize>`.
///
/// It returns a `Duration`, the runtime of the execution given function.
pub fn measure(pattern: &[u8], text: &[u8], f: fn(&[u8], &[u8]) -> Vec<usize>) -> Duration {
    let before = SystemTime::now();

    f(pattern, text);

    let duration = before.elapsed();

    duration.expect("Could not measure time.")
}

/// A function to measure the runtimes of multiple executions of an algorithm.
///
/// It takes a `pattern` and a `text` and executes a function `f` using
/// the standard signature of the pattern matching algorithms
/// `(&[u8], &[u8]) -> Vec<usize>`.
///
/// It returns a `Vec<Duration>`, the runtimes of the exeuctions
/// of the given functions.
pub fn measure_multiple(
    pattern: &[u8],
    text: &[u8],
    f: fn(&[u8], &[u8]) -> Vec<usize>,
    n: usize,
) -> Vec<Duration> {
    let mut durations: Vec<Duration> = Vec::new();

    for _ in 0..n {
        durations.push(measure(pattern, text, f));
    }

    durations
}

/// A function to calculate the average duration of a `Vec<Duration>`
/// in milliseconds.
///
/// It takes a `Vec<Duration>` of multiple `Duration`s.
///
/// It returns the average duration in milliseconds as an `f64`.
pub fn calculate_avg_duration(durations: Vec<Duration>) -> f64 {
    let sum: Duration = durations.iter().sum();
    let sum_nanos = sum.as_nanos() as f64;

    (sum_nanos / durations.len() as f64) / 1_000_000f64
}
