use std::time::{Duration, SystemTime};

pub fn measure(pattern: &[u8], text: &[u8], f: fn(&[u8], &[u8]) -> Vec<usize>) -> Duration {
    let before = SystemTime::now();

    f(pattern, text);

    let duration = before.elapsed();

    duration.expect("Could not measure time.")
}

pub fn measure_multiple(pattern: &[u8], text: &[u8], f: fn(&[u8], &[u8]) -> Vec<usize>, n: usize) -> Vec<Duration> {
    let mut durations: Vec<Duration> = Vec::new();

    for _ in 0..n {

        let before = SystemTime::now();

        f(pattern, text);

        let duration = before.elapsed();
        durations.push(duration.expect("Could not measure time."));
    }

    durations
}

pub fn calculate_avg_duration(durations: Vec<Duration>) -> f64 {
    let sum: Duration = durations.iter().sum();
    let sum_nanos = sum.as_nanos() as f64;

    (sum_nanos / durations.len() as f64) / 1_000_000f64
}
