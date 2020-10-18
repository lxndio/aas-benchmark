use std::time::Duration;

use crate::match_algorithm::algorithm_name;
use crate::measure::calculate_avg_duration;

/// A struct containg the measurement results of one or multiple
/// algorithm executions of the same algorithm.
pub struct MeasureResult {
    algorithm_name: String,

    matches: usize,

    durations: Vec<Duration>,
    avg_duration: f64,
}

impl MeasureResult {
    #[allow(unused)]
    /// Initalizes a new `MeasureResult` and calculates the average duration
    /// of the given durations.
    ///
    /// It takes the CLI paramter name of an algorithm and the durations of
    /// the measured executions.
    pub fn new(algorithm: &str, matches: usize, durations: Vec<Duration>) -> Self {
        let mut new = Self {
            algorithm_name: String::from(algorithm_name(algorithm)),

            matches,

            durations,
            avg_duration: 0f64,
        };

        new.avg_duration = calculate_avg_duration(&new.durations);

        new
    }

    /// Set the algorithm name of the `MeasureResult`.
    ///
    /// It takes the CLI parameter name of an algorithm.
    pub fn set_algorithm(&mut self, algorithm: &str) -> &mut Self {
        self.algorithm_name = String::from(algorithm_name(algorithm));

        self
    }

    /// Prints a summary of the `MeasureResult` containing statistical
    /// facts.
    ///
    /// If `print_durations` is `true`, it also prints a list containing
    /// the durations of each execution.
    pub fn print(&mut self, print_durations: bool) -> &mut Self {
        // Print algorithm name
        println!("===== {} =====", self.algorithm_name);

        println!("Matches: {}", self.matches);

        // Print average runtime
        let average = self.avg_duration;
        if average != 0f64 {
            println!("Average: {} ms", average);
        } else {
            println!("No executions");
        }

        // Print new line at the end
        println!();

        // If print_durations is set, print a list of each duration
        if print_durations {
            println!("{:?}\n", self.durations);
        }

        self
    }
}

impl From<(Vec<Duration>, usize)> for MeasureResult {
    fn from(from_params: (Vec<Duration>, usize)) -> Self {
        let durations = from_params.0;
        let matches = from_params.1;

        let mut from = MeasureResult {
            algorithm_name: String::new(),

            matches,

            durations,
            avg_duration: 0f64,
        };

        from.avg_duration = calculate_avg_duration(&from.durations);

        from
    }
}
