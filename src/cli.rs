use clap::App;

use crate::match_algorithm::match_algorithm;
use crate::range::Range;

pub struct CLIParams {
    pub algorithms: Vec<String>,

    pub human_readble: bool,
    pub pattern_from_text: bool,
    pub random_text: bool,
    pub random_pattern_from_text: bool,

    pub executions: usize,
    pub random_text_length: usize,
    pub random_pattern_from_text_length: Range,

    pub pattern_from_text_range: Range,
}

impl CLIParams {
    /// Reads CLI arguments, parses them using Clap and
    /// returns a new `CLIParams` object.
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
        let human_readble: bool = matches.is_present("human_readble");
        let pattern_from_text: bool = matches.is_present("pattern_from_text");
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

        // Other type paramters
        let pattern_from_text_range: Range = matches
            .value_of("pattern_from_text")
            .unwrap_or("0..0")
            .parse()
            .unwrap_or(Range::new(0, 0, 0));

        let random_pattern_from_text_arg =
            matches.value_of("random_pattern_from_text").unwrap_or("-1");
        let random_pattern_from_text_length: Range = random_pattern_from_text_arg
            .parse()
            .or_else(|e| {
                // If the user input could not be parsed as a Range, it might
                // just be a positive integer so try that; otherwise just
                // return an invalid Range (in unwrap_or)
                if let Ok(length) = random_pattern_from_text_arg.parse::<usize>() {
                    Ok(Range::new(length, length + 1, 1))
                } else {
                    Err(e)
                }
            })
            .unwrap_or(Range::new(0, 0, 0));

        // Return new CLIParams object
        Self {
            algorithms,

            human_readble,
            pattern_from_text,
            random_text,
            random_pattern_from_text,

            executions,
            random_text_length,
            random_pattern_from_text_length,

            pattern_from_text_range,
        }
    }

    /// Validates CLI arguments stored in this `CLIParams` object.
    pub fn valid(&self) -> bool {
        let mut valid = true;

        // String value parameters
        for algorithm in &self.algorithms {
            if match_algorithm(&algorithm).is_none() {
                println!("Unknown algorithm given: {}.\n", algorithm);
                valid = false;
            }
        }

        // Bool value parameters
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

        // Number value parameters
        if self.executions == 0 {
            println!("The -n argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }
        if self.random_text && self.random_text_length == 0 {
            println!("The -t argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }

        // Other type paramters
        if self.pattern_from_text && self.pattern_from_text_range.is_empty() {
            println!("The --patternfromtext argument needs to be a valid, non-empty range.\n");
            valid = false;
        } else if self.pattern_from_text && self.pattern_from_text_range.step_size != 1 {
            println!("The --patternfromtext argument does not take a step size.\n");
            valid = false;
        }
        if self.random_pattern_from_text && self.random_pattern_from_text_length.is_empty() {
            println!("The -p argument needs to be a valid, non-empty range or a positive integer greater than 0.");
            valid = false;
        } else if self.pattern_from_text {
            if let Some(length) = self.random_pattern_from_text_length.single() {
                if length == 0 {
                    println!("The -p argument needs to be a positive integer greater than 0.\n");
                    valid = false;
                }
            }
        }

        valid
    }
}
