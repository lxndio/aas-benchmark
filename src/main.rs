#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate regex;

mod algorithms;
#[cfg(not(tarpaulin_include))]
mod cli;
#[cfg(not(tarpaulin_include))]
mod generate;
#[cfg(not(tarpaulin_include))]
mod match_algorithm;
#[cfg(not(tarpaulin_include))]
mod measure;
#[cfg(not(tarpaulin_include))]
mod pattern;
mod range;
#[cfg(not(tarpaulin_include))]
mod text;

use std::error::Error;

use cli::CLIParams;
use match_algorithm::match_algorithms;
use measure::measure_multiple_different_patterns;
use pattern::generate_patterns;
use text::generate_text;

#[cfg(not(tarpaulin_include))]
fn main() -> Result<(), Box<dyn Error>> {
    // Get CLI parameters using Clap
    let cli_params = CLIParams::new();

    // Only continue if all given parameters are valid, all unwraps are safe
    // here because of the checks done in cli_params.valid()
    if cli_params.valid() {
        let text = generate_text(&cli_params, cli_params.seed);

        if let Ok(text) = &text {
            let patterns = generate_patterns(&cli_params, text);

            if let Ok(patterns) = patterns {
                let mut csv_header_printed = false;

                let algorithm_functions = match_algorithms(&cli_params.algorithms);

                for (algorithm, algorithm_fn) in algorithm_functions.iter() {
                    let measure_results = measure_multiple_different_patterns(
                        &algorithm,
                        &patterns,
                        text,
                        algorithm_fn,
                        &cli_params,
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
            } else if let Err(err) = patterns {
                println!("Error while generating pattern source: {}", err);
            }
        } else if let Err(err) = text {
            println!("Error while generating text source: {}", err);
        }
    }

    Ok(())
}
