use clap::App;

use crate::match_algorithm::match_algorithm;
use crate::range::Range;

pub struct CLIParams {
    pub algorithms: Vec<String>,

    pub pattern_from_text: bool,
    pub print_csv: bool,
    pub random_text: bool,
    pub random_pattern_from_text: bool,

    pub executions: usize,
    pub random_text_length: usize,
    pub random_pattern_from_text_length: usize,

    pub pattern_from_text_range: Range,
}

impl CLIParams {
    pub fn new() -> Self {
        let clap_yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(clap_yaml).get_matches();

        // String value parameters
        let algorithms: Vec<String> = String::from(
            matches
                .value_of("ALGORITHMS")
                .unwrap_or("NonexistentAlgorithm"),
        )
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|algorithm| String::from(*algorithm))
        .collect();

        // Bool value parameters
        let pattern_from_text: bool = matches.is_present("pattern_from_text");
        let print_csv: bool = matches.is_present("print_csv");
        let random_text: bool = matches.is_present("random_text");
        let random_pattern_from_text: bool = matches.is_present("random_pattern_from_text");

        // Number value parameters
        let executions: usize = matches
            .value_of("executions")
            .unwrap_or("1") // 1 so that if parameter is not given, the default of one execution is used
            .parse()
            .unwrap_or(0); // 0 so that if invalid parameter is given, validation fails
        let random_text_length: usize = matches
            .value_of("random_text")
            .unwrap_or("0") // 0 so that if no text source is set, validation fails
            .parse()
            .unwrap_or(0); // 0 so that if invalid parameter is given, validation fails
        let random_pattern_from_text_length: usize = matches
            .value_of("random_pattern_from_text")
            .unwrap_or("0") // 0 so that if no pattern source is set, validation fails
            .parse()
            .unwrap_or(0); // 0 so that if invalid parameter is given, validation fails

        // Other type paramters
        let pattern_from_text_range: Range = matches
            .value_of("pattern_from_text")
            .unwrap_or("0..0")
            .parse()
            .unwrap_or(Range::new(0, 0));

        Self {
            algorithms,

            pattern_from_text,
            print_csv,
            random_text,
            random_pattern_from_text,

            executions,
            random_text_length,
            random_pattern_from_text_length,

            pattern_from_text_range,
        }
    }

    pub fn valid(&self) -> bool {
        let mut valid = true;

        for algorithm in &self.algorithms {
            if match_algorithm(&algorithm).is_none() {
                println!("Unknown algorithm given: {}.\n", algorithm);
                valid = false;
            }
        }

        if !self.random_text {
            println!("At least one text source has to be set. \
                You could for example set `-t 1000000` to generate a random text with a length of 1_000_000 characters.\n");
            valid = false;
        }
        if !(self.random_pattern_from_text || self.pattern_from_text) {
            println!("At least one pattern source has to be set. \
                You could for example set `-p 5` to take a random pattern of length 5 from the text.\n");
            valid = false;
        }
        if self.random_pattern_from_text && self.pattern_from_text {
            println!("You can only set one pattern source at a time.\n");
            valid = false;
        }

        if self.executions == 0 {
            println!("The -n argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }
        if self.random_text && self.random_text_length == 0 {
            println!("The -t argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }
        if self.random_pattern_from_text && self.random_pattern_from_text_length == 0 {
            println!("The -p argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }

        if self.pattern_from_text && self.pattern_from_text_range.is_empty() {
            println!("The --patternfromtext argument needs a valid, non-empty range.\n");
            valid = false;
        }

        valid
    }
}
