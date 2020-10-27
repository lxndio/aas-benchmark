#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod algorithms;
mod cli;
mod generate;
mod match_algorithm;
mod measure;
mod range;

use std::error::Error;

use cli::CLIParams;
use generate::{gen_patterns, gen_rand_bytes};
use match_algorithm::match_algorithm;
use measure::measure_multiple_different_patterns;

fn main() -> Result<(), Box<dyn Error>> {
    // Get CLI parameters using Clap
    let cli_params = CLIParams::new();

    // Only continue if all given parameters are valid, all unwraps are safe
    // here because of the checks done in cli_params.valid()
    if cli_params.valid() {
        let text = &gen_rand_bytes(cli_params.random_text_length);
        let patterns = gen_patterns(text, &cli_params).expect("Could not generate pattern."); // TODO better error handling

        let mut csv_header_printed = false;

        for algorithm in cli_params.algorithms {
            let algorithm_fn = match_algorithm(&algorithm);

            let measure_results = measure_multiple_different_patterns(
                &algorithm,
                &patterns,
                text,
                algorithm_fn.unwrap(),
                cli_params.executions,
            );

            if !cli_params.human_readble {
                for measure_result in measure_results {
                    measure_result.print_csv(!csv_header_printed);

                    if !csv_header_printed {
                        csv_header_printed = true;
                    }
                }
            } else {
                for measure_result in measure_results {
                    measure_result.print(false);
                }
            }
        }
    }

    Ok(())
}
