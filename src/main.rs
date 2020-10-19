#[macro_use]
extern crate clap;

mod algorithms;
mod cli;
mod generate;
mod match_algorithm;
mod measure;

use cli::CLIParams;
use generate::{gen_rand_bytes, rand_pattern_from_bytes};
use match_algorithm::match_algorithm;
use measure::measure_multiple;
use measure::measure_result::MeasureResult;

fn main() {
    // Get CLI parameters using Clap
    let cli_params = CLIParams::new();

    // Only continue if all given parameters are valid, all unwraps are safe
    // here because of the checks done in cli_params.valid()
    if cli_params.valid() {
        let text = &gen_rand_bytes(cli_params.random_text_length);
        let pattern;

        if cli_params.random_pattern_from_text {
            pattern = rand_pattern_from_bytes(text, cli_params.random_pattern_from_text_length);
        } else {
            pattern = &[];
        }

        for algorithm in cli_params.algorithms {
            let algorithm_fn = match_algorithm(&algorithm);

            let durations =
                measure_multiple(pattern, text, algorithm_fn.unwrap(), cli_params.executions);

            MeasureResult::from(durations)
                .set_algorithm(&algorithm)
                .print(false);
        }
    }
}
