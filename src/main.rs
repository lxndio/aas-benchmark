#[macro_use]
extern crate clap;

mod algorithms;
mod generate;
mod measure;

use clap::App;

use algorithms::single_pattern::naive::naive_all;
use generate::gen_rand_bytes;
use measure::{calculate_avg_duration, measure_multiple};

pub const ALGORITHMS: [&str; 1] = ["Naive"]; // TODO necessary?

fn main() {
    let clap_yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(clap_yaml).get_matches();

    let algorithm = matches
        .value_of("ALGORITHM")
        .unwrap_or("NonexistentAlgorithm");
    let executions: usize = matches
        .value_of("executions")
        .unwrap_or("1")
        .parse()
        .unwrap_or(0);

    let mut failure = false;

    if executions == 0 {
        println!("The -n argument needs to be a positive integer greater than 0.");
        failure = true;
    }

    let algorithm_fn = match algorithm.to_lowercase().as_str() {
        "naive" => Some(naive_all),
        _ => {
            println!("Unknown algorithm given.");
            failure = true;

            None
        }
    };

    if !failure {
        let text = gen_rand_bytes(1_000_000);
        let pattern = &text[20..25];

        // Unwrap is safe here because of the failure variable
        let durations = measure_multiple(pattern, &text, algorithm_fn.unwrap(), executions);

        if durations.len() != 0 {
            println!("Duration: {:?}", durations);
            println!("Average: {:?}", calculate_avg_duration(durations));
        }
    }
}
