use clap::App;

use crate::match_algorithm::match_algorithm;

pub struct CLIParams {
    pub algorithm: String,
    pub compare_algorithm: String,

    pub compare: bool,
    pub random_text: bool,
    pub random_pattern_from_text: bool,

    pub executions: usize,
    pub random_text_length: usize,
    pub random_pattern_from_text_length: usize,
}

impl CLIParams {
    pub fn new() -> Self {
        let clap_yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(clap_yaml).get_matches();

        // String value parameters
        let algorithm = String::from(
            matches
                .value_of("ALGORITHM")
                .unwrap_or("NonexistentAlgorithm"),
        );
        let compare_algorithm = String::from(
            matches
                .value_of("compare")
                .unwrap_or("NonexistentAlgorithm"),
        );

        // Bool value parameters
        let compare: bool = matches.is_present("compare");
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

        Self {
            algorithm,
            compare_algorithm,

            compare,
            random_text,
            random_pattern_from_text,

            executions,
            random_text_length,
            random_pattern_from_text_length,
        }
    }

    pub fn valid(&self) -> bool {
        let mut valid = true;

        if match_algorithm(&self.algorithm).is_none() {
            println!("Unknown algorithm given.\n");
            valid = false;
        }
        if self.compare && match_algorithm(&self.compare_algorithm).is_none() {
            println!("Unknown compare algorithm given.\n");
            valid = false;
        }

        if !self.random_text {
            println!("At least one text source has to be set. \
                You could for example set `-t 1000000` to generate a random text with a length of 1_000_000 characters.\n");
            valid = false;
        }
        if !self.random_pattern_from_text {
            println!("At least one pattern source has to be set. \
                You could for example set `-p 5` to take a random pattern of length 5 from the text.\n");
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

        valid
    }
}
