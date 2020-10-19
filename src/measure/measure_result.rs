use std::error::Error;
use std::io;
use std::time::Duration;

use csv::WriterBuilder;

use crate::match_algorithm::algorithm_name;
use crate::measure::calculate_avg_duration;
use crate::measure::csv_record::CSVRecord;

/// A struct containg the measurement results of one or multiple
/// algorithm executions of the same algorithm.
pub struct MeasureResult {
    algorithm_name: String,

    text_length: usize,
    pattern_length: usize,

    matches: usize,

    durations: Vec<Duration>,
    avg_duration: f64,
}

impl MeasureResult {
    /// Initalizes a new `MeasureResult` and calculates the average duration
    /// of the given durations.
    ///
    /// It takes the CLI paramter name of an algorithm and the durations of
    /// the measured executions.
    pub fn new(
        algorithm: &str,
        text_length: usize,
        pattern_length: usize,
        matches: usize,
        durations: Vec<Duration>,
    ) -> Self {
        let mut new = Self {
            algorithm_name: String::from(algorithm_name(algorithm)),

            text_length,
            pattern_length,

            matches,

            durations,
            avg_duration: 0f64,
        };

        new.avg_duration = calculate_avg_duration(&new.durations);

        new
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

    pub fn print_csv(&self, print_header: bool) -> Result<(), Box<dyn Error>> {
        let mut wtr = WriterBuilder::new()
            .has_headers(print_header)
            .from_writer(io::stdout());

        for (execution, duration) in self.durations.iter().enumerate() {
            let time_ms = duration.as_millis();

            wtr.serialize(CSVRecord::new(
                &self.algorithm_name,
                self.text_length,
                self.pattern_length,
                execution,
                time_ms,
            ))?;
        }

        wtr.flush()?;

        Ok(())
    }
}
