#[macro_use]
extern crate clap;

mod algorithms;
mod cli;
mod generate;
mod match_algorithm;
mod measure;

use cli::CLIParams;
use generate::gen_rand_bytes;
use match_algorithm::match_algorithm;
use measure::measure_multiple;
use measure::measure_result::MeasureResult;

fn main() {
    // Get CLI parameters using Clap
    let cli_params = CLIParams::new();

    let mut failure = false;

    if cli_params.executions == 0 {
        println!("The -n argument needs to be a positive integer greater than 0.");
        failure = true;
    }

    let algorithm_fn = match_algorithm(&cli_params.algorithm);
    let compare_algorithm_fn = match_algorithm(&cli_params.compare_algorithm);

    // Check if given algorithm exists
    if algorithm_fn.is_none() {
        println!("Unknown algorithm given.");
        failure = true;
    }

    // Check if given compare algorithm exists
    if cli_params.compare && compare_algorithm_fn.is_none() {
        println!("Unknown compare algorithm given.");
        failure = true;
    }

    if !failure {
        let text = &gen_rand_bytes(1_000_000);
        let pattern = &text[20..25];

        // Unwrap is safe here because of the failure variable
        let durations =
            measure_multiple(pattern, text, algorithm_fn.unwrap(), cli_params.executions);

        MeasureResult::from(durations)
            .set_algorithm(&cli_params.algorithm)
            .print(false);

        if cli_params.compare {
            // Unwrap is safe here because of the failure variable
            let durations = measure_multiple(
                pattern,
                text,
                compare_algorithm_fn.unwrap(),
                cli_params.executions,
            );

            MeasureResult::from(durations)
                .set_algorithm(&cli_params.compare_algorithm)
                .print(false);
        }
    }
}
