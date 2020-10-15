use std::time::Duration;

use crate::match_algorithm::algorithm_name;
use crate::measure::calculate_avg_duration;

pub struct MeasureResult {
    algorithm_name: String,

    durations: Vec<Duration>,
    avg_duration: f64,
}

impl MeasureResult {
    #[allow(unused)]
    pub fn new(algorithm: &str, durations: Vec<Duration>) -> Self {
        let mut new = Self {
            algorithm_name: String::from(algorithm_name(algorithm)),

            durations,
            avg_duration: 0f64,
        };

        new.avg_duration = calculate_avg_duration(&new.durations);

        new
    }

    pub fn set_algorithm(&mut self, algorithm: &str) -> &mut Self {
        self.algorithm_name = String::from(algorithm_name(algorithm));

        self
    }

    pub fn print(&mut self, print_durations: bool) -> &mut Self {
        // Print algorithm name
        println!("===== {} =====", self.algorithm_name);

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

impl From<Vec<Duration>> for MeasureResult {
    fn from(durations: Vec<Duration>) -> Self {
        let mut from = MeasureResult {
            algorithm_name: String::new(),

            durations,
            avg_duration: 0f64,
        };

        from.avg_duration = calculate_avg_duration(&from.durations);

        from
    }
}
