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
mod pattern;
mod range;
mod text;

use std::error::Error;

use cli::CLIParams;
use match_algorithm::match_algorithm;
use measure::measure_multiple_different_patterns;
use pattern::generate_patterns;
use text::generate_text;

fn main() -> Result<(), Box<dyn Error>> {
    // Get CLI parameters using Clap
    let cli_params = CLIParams::new();

    // Only continue if all given parameters are valid, all unwraps are safe
    // here because of the checks done in cli_params.valid()
    if cli_params.valid() {
        let text = generate_text(&cli_params);

        if text.is_ok() {
            let text = &text.unwrap();

            let patterns = generate_patterns(&cli_params, text);

            if patterns.is_ok() {
                let patterns = patterns.unwrap();

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
                            measure_result.print_csv(!csv_header_printed)?;

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
            } else {
                if let Err(err) = patterns {
                    println!("Error while generating pattern source: {}", err);
                }
            }
        } else {
            if let Err(err) = text {
                println!("Error while generating text source: {}", err);
            }
        }
    }

    Ok(())
}
