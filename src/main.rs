#[macro_use]
extern crate clap;

mod algorithms;
mod generate;
mod match_algorithm;
mod measure;

use clap::App;

use generate::gen_rand_bytes;
use match_algorithm::match_algorithm;
use measure::measure_multiple;
use measure::measure_result::MeasureResult;

fn main() {
    let clap_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    let algorithm = matches
        .value_of("ALGORITHM")
        .unwrap_or("NonexistentAlgorithm");
    let executions: usize = matches
        .value_of("executions")
        .unwrap_or("1") // 1 so that if parameter is not given, the default of one execution is used
        .parse()
        .unwrap_or(0); // 0 so that if invalid parameter is given, failure is set to true below
    let compare: bool = matches.is_present("compare");
    let compare_algorithm = matches
        .value_of("compare")
        .unwrap_or("NonexistentAlgorithm");

    let mut failure = false;

    if executions == 0 {
        println!("The -n argument needs to be a positive integer greater than 0.");
        failure = true;
    }

    let algorithm_fn = match_algorithm(algorithm);
    let compare_algorithm_fn = match_algorithm(compare_algorithm);

    // Check if given algorithm exists
    if algorithm_fn.is_none() {
        println!("Unknown algorithm given.");
        failure = true;
    }

    // Check if given compare algorithm exists
    if compare && compare_algorithm_fn.is_none() {
        println!("Unknown compare algorithm given.");
        failure = true;
    }

    if !failure {
        let text = gen_rand_bytes(1_000_000);
        let pattern = &text[20..25];

        // Unwrap is safe here because of the failure variable
        let durations = measure_multiple(pattern, &text, algorithm_fn.unwrap(), executions);

        MeasureResult::from(durations)
            .set_algorithm(algorithm)
            .print(false);

        if compare {
            let text = gen_rand_bytes(1_000_000);
            let pattern = &text[20..25];

            // Unwrap is safe here because of the failure variable
            let durations =
                measure_multiple(pattern, &text, compare_algorithm_fn.unwrap(), executions);

            MeasureResult::from(durations)
                .set_algorithm(compare_algorithm)
                .print(false);
        }
    }
}
