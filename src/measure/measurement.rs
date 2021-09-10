use std::collections::HashMap;
use std::time::Duration;

use crate::cli::CLIParams;
use crate::match_algorithm::TypedAlgorithm;
use crate::measure::measurement_result::MeasurementResult;
use crate::measure::{Measure, MultiplePatternMeasure};

pub struct SingleMeasurement {
    preparation: Option<Duration>,
    execution: Duration,
    matches: usize,
    special_fields: Option<HashMap<String, usize>>,
}

pub struct Measurement {
    algorithms: Vec<(String, TypedAlgorithm)>,
    text: Vec<u8>,
    patterns: Vec<Vec<u8>>,
    cli_params: CLIParams,
    measurement_results: Vec<MeasurementResult>,
}

impl SingleMeasurement {
    pub fn new(
        preparation: Option<Duration>,
        execution: Duration,
        matches: usize,
        special_fields: Option<HashMap<String, usize>>,
    ) -> Self {
        Self {
            preparation,
            execution,
            matches,
            special_fields,
        }
    }
}

impl Measurement {
    pub fn new(
        algorithms: Vec<(String, TypedAlgorithm)>,
        text: Vec<u8>,
        patterns: Vec<Vec<u8>>,
        cli_params: CLIParams,
    ) -> Self {
        Self {
            algorithms,
            text,
            patterns,
            cli_params,
            measurement_results: Vec::new(),
        }
    }

    pub fn run_measurement(&mut self) -> &mut Self {
        let mut measurement_results = Vec::new();

        for (algorithm, algorithm_fn) in self.algorithms.iter() {
            // If the algorithm only takes a single pattern, run it once for
            // every given pattern. If the algorithm takes multiple patterns,
            // run the algorithm once with all given patterns.
            match algorithm_fn {
                TypedAlgorithm::SinglePatternAlgorithm(_)
                | TypedAlgorithm::SuffixArrayAlgorithm(_)
                | TypedAlgorithm::BWTAlgorithm(_)
                | TypedAlgorithm::ApproximativeAlgorithm(_) => {
                    for pattern in self.patterns.iter() {
                        let measurements =
                            measure_exeuctions(pattern, &self.text, algorithm_fn, &self.cli_params);

                        let preparation_durations =
                            measurements.iter().map(|x| x.preparation).collect();
                        let algorithm_durations =
                            measurements.iter().map(|x| x.execution).collect();
                        let comparisons = measurements.get(0).unwrap().comparisons;
                        let matches = measurements.get(0).unwrap().matches;

                        measurement_results.push(MeasurementResult::new(
                            algorithm,
                            self.text.len(),
                            pattern.len(),
                            preparation_durations,
                            algorithm_durations,
                            comparisons,
                            matches,
                        ));
                    }
                }
                TypedAlgorithm::MultiplePatternAlgorithm(f) => {
                    // Run given multi pattern algorithm for given number of
                    // executions
                    let measurements = (0..self.cli_params.executions)
                        .map(|_| f.measure(&self.patterns, &self.text, &self.cli_params))
                        .collect::<Vec<_>>();

                    let preparation_durations =
                        measurements.iter().map(|x| x.preparation).collect();
                    let algorithm_durations = measurements.iter().map(|x| x.execution).collect();
                    let comparisons = measurements.get(0).unwrap().comparisons;
                    let matches = measurements.get(0).unwrap().matches;

                    measurement_results.push(MeasurementResult::new(
                        algorithm,
                        self.text.len(),
                        // TODO does 0 make sense when benchmarking multiple pattern at once?
                        // Or rather use -1 or something like that?
                        0,
                        preparation_durations,
                        algorithm_durations,
                        comparisons,
                        matches,
                    ));
                }
            }
        }

        self.measurement_results = measurement_results;

        self
    }

    pub fn print_csv(&self) -> Result<(), String> {
        if !self.measurement_results.is_empty() {
            // Set to no_header so that there will be no header printed if
            // no_header is set, as the variable will already be true then
            let mut csv_header_printed = self.cli_params.no_header;

            for measurement_result in self.measurement_results.iter() {
                // TODO is there a nicer way to do this?
                match measurement_result.print_csv(!csv_header_printed) {
                    Ok(_) => (),
                    Err(error) => return Err(error.to_string()),
                }

                if !csv_header_printed {
                    csv_header_printed = true;
                }
            }

            Ok(())
        } else {
            Err(String::from(
                "Can't print CSV records before measurements have been taken",
            ))
        }
    }
}

/// A function to measure the runtimes of multiple executions of an algorithm.
///
/// It takes a `pattern` and a `text` and executes a function `f` using
/// the standard signature of the pattern matching algorithms
/// `(&[u8], &[u8]) -> Vec<usize>`.
///
/// It returns a `(Vec<Duration>, usize)`, the runtimes of the exeuctions
/// of the given functions and the number of matches.
fn measure_exeuctions(
    pattern: &[u8],
    text: &[u8],
    f: &TypedAlgorithm,
    cli_params: &CLIParams,
) -> Vec<SingleMeasurement> {
    let mut single_measurements: Vec<SingleMeasurement> = Vec::new();

    for _ in 0..cli_params.executions {
        single_measurements.push(match f {
            TypedAlgorithm::SinglePatternAlgorithm(f) => f.measure(pattern, text, cli_params),
            TypedAlgorithm::MultiplePatternAlgorithm(_) => {
                // This case should not occur here because of the matching in
                // run_measurement()
                unimplemented!();
            }
            TypedAlgorithm::SuffixArrayAlgorithm(f) => f.measure(pattern, text, cli_params),
            TypedAlgorithm::BWTAlgorithm(f) => f.measure(pattern, text, cli_params),
            TypedAlgorithm::ApproximativeAlgorithm(f) => f.measure(pattern, text, cli_params),
        });
    }

    single_measurements
}
