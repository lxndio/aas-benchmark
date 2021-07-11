use clap::{App, ArgMatches};

use crate::match_algorithm::match_algorithms;
use crate::pattern::PatternSource;
use crate::range::Range;
use crate::text::TextSource;

pub struct CLIParams {
    pub algorithms: Vec<String>,

    pub no_header: bool,

    pub executions: usize,
    pub seed: Option<u64>,

    pub pattern_source: PatternSource,
    pub text_source: TextSource,

    pub suffix_array_algorithm: String,
    pub occ_block_size: usize,
    pub maximum_error: Option<usize>,
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
        let suffix_array_algorithm = matches
            .value_of("suffix_array_algorithm")
            .unwrap_or("sais")
            .to_string();

        // Bool value parameters
        let no_header: bool = matches.is_present("no_header");

        // Number value parameters
        let executions: usize = matches
            .value_of("executions")
            .unwrap_or("1") // 1 so that if parameter is not given, the default of one execution is used
            .parse()
            .unwrap_or(0); // 0 so that if invalid parameter is given, validation fails
        let seed: Option<u64> = matches
            .value_of("seed")
            .unwrap_or("-1") // -1 so that parse fails if the argument is not set, resulting in seed being None
            .parse()
            .ok();
        let occ_block_size = matches
            .value_of("occ_block_size")
            .unwrap_or("1")
            .parse()
            .unwrap_or(0);
        let maximum_error = matches
            .value_of("maximum_error")
            .unwrap_or("0")
            .parse()
            .ok();

        // Return new CLIParams object
        Self {
            algorithms,

            no_header,

            executions,
            seed,

            pattern_source: Self::set_pattern_source(&matches),
            text_source: Self::set_text_source(&matches),

            suffix_array_algorithm,
            occ_block_size,
            maximum_error,
        }
    }

    /// Validates CLI arguments stored in this `CLIParams` object.
    pub fn valid(&self) -> bool {
        let mut valid = true;

        // String value parameters
        if match_algorithms(&self.algorithms).is_empty() {
            println!("Unknown algorithm given.\n");
            valid = false;
        }

        if self.algorithms.contains(&String::from("all")) && self.algorithms.len() != 1 {
            println!("You cannot specify multiple algorithms if you specify \"all\".");
            valid = false;
        }

        if self
            .algorithms
            .iter()
            .any(|x| x.starts_with("sa-") || x.starts_with("bwt-"))
            && !["naive", "sais"].contains(&self.suffix_array_algorithm.as_str())
        {
            println!(
                "You have to specify a valid algorithm for suffix array generation. \
                Options are: naive, sais.\nYou could also omit the parameter to use the \
                default algorithm `sais`.\n"
            );
            valid = false;
        }

        // Number value parameters
        if self.executions == 0 {
            println!("The -n argument needs to be a positive integer greater than 0.\n");
            valid = false;
        }

        if self.algorithms.contains(&String::from("bwt-match-k")) && self.occ_block_size == 0 {
            println!(
                "You have to enter a valid block size for the Occ array when \
                using the `bwt-match-k` algorithm or omit the parameter to use \
                the default block size of 1."
            );
            valid = false;
        }

        if self.maximum_error.is_none() {
            println!(
                "The --maxerror argument needs to be a positive integer.\nYou could \
                also omit the parameter to use the default value of 0.\n"
            );
            valid = false;
        }

        // Other type paramters
        if let PatternSource::Error(err) = self.pattern_source {
            println!("Error while parsing pattern source: {}", err);
            valid = false;
        }

        if let TextSource::Error(err) = self.text_source {
            println!("Error while parsing text source: {}", err);
            valid = false;
        }

        valid
    }

    fn set_pattern_source(matches: &ArgMatches) -> PatternSource {
        let pattern_from_argument: bool = matches.is_present("pattern_from_argument");
        let pattern_from_file: bool = matches.is_present("pattern_from_file");
        let pattern_from_text: bool = matches.is_present("pattern_from_text");
        let random_pattern: bool = matches.is_present("random_pattern");
        let random_pattern_from_text: bool = matches.is_present("random_pattern_from_text");
        let multiple_patterns_from_file: bool = matches.is_present("multiple_patterns_from_file");

        let sources = vec![
            pattern_from_argument,
            pattern_from_file,
            pattern_from_text,
            random_pattern,
            random_pattern_from_text,
            multiple_patterns_from_file,
        ];

        if none(&sources) {
            return PatternSource::Error("At least one pattern source has to be set.");
        }

        match only(&sources) {
            // Pattern from argument
            Some(0) => {
                if let Some(patterns) = matches.values_of("pattern_from_argument") {
                    PatternSource::FromArgument(patterns.map(|x| x.to_string()).collect())
                } else {
                    PatternSource::Error(
                        "The --patternfromarg argument requires a valid, non-empty pattern.",
                    )
                }
            }
            // Pattern from file
            Some(1) => {
                if let Some(file_name) = matches.value_of("pattern_from_file") {
                    PatternSource::FromFile(file_name.to_string(), false)
                } else {
                    PatternSource::Error("The --pf argument needs a valid parameter.")
                }
            }
            // Pattern from text
            Some(2) => {
                let pattern_from_text_range: Range = matches
                    .value_of("pattern_from_text")
                    .unwrap_or("0..0")
                    .parse()
                    .unwrap_or_default();

                // TODO better error handling, probably using ok_or() above
                if !pattern_from_text_range.is_empty() && pattern_from_text_range.step_size != 1 {
                    PatternSource::Error(
                        "The --patternfromtext argument does not take a step size.",
                    )
                } else if pattern_from_text_range.is_empty() {
                    PatternSource::Error(
                        "The --patternfromtext argument needs to be a valid, non-empty range.",
                    )
                } else {
                    PatternSource::FromText(pattern_from_text_range)
                }
            }
            // Random pattern
            Some(3) => {
                match matches.value_of("random_pattern").unwrap_or("-1").parse::<Range>() {
                    Ok(range) => {
                        if range.is_valid() {
                            PatternSource::Random(range)
                        } else {
                            PatternSource::Error("The --randompattern argument needs to be a valid, non-empty range or a positive integer greater than 0.")
                        }
                    }
                    Err(_) => PatternSource::Error("The --randompattern argument needs to be a valid, non-empty range or a positive integer greater than 0."),
                }
            }
            // Random pattern from text
            Some(4) => {
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
                    .unwrap_or_default();

                // TODO better error handling, probably using ok_or() above
                if random_pattern_from_text_length.is_valid() {
                    if let Some(length) = random_pattern_from_text_length.single() {
                        if length == 0 {
                            return PatternSource::Error(
                                "The -p argument needs to be a positive integer greater than 0.",
                            );
                        }
                    }

                    PatternSource::FromTextRandom(random_pattern_from_text_length)
                } else {
                    PatternSource::Error("The -p argument needs to be a valid, non-empty range or a positive integer greater than 0.")
                }
            }
            // Multiple patterns from file
            Some(5) => {
                if let Some(file_name) = matches.value_of("multiple_patterns_from_file") {
                    PatternSource::FromFile(file_name.to_string(), true)
                } else {
                    PatternSource::Error("The --pmf argument needs a valid parameter.")
                }
            }
            None => PatternSource::Error("You can only set one pattern source."),
            _ => PatternSource::Error("Internal error while processing the pattern source."),
        }
    }

    fn set_text_source(matches: &ArgMatches) -> TextSource {
        let random_text: bool = matches.is_present("random_text");
        let text_from_file: bool = matches.is_present("text_from_file");
        let text_from_file_binary: bool = matches.is_present("text_from_file_binary");

        let sources = vec![random_text, text_from_file, text_from_file_binary];

        if none(&sources) {
            return TextSource::Error("At least one text source has to be set.");
        }

        match only(&sources) {
            Some(0) => {
                let random_text_length: usize = matches
                    .value_of("random_text")
                    .unwrap_or("0") // 0 so that if no text source is set, validation fails
                    .parse()
                    .unwrap_or(0); // 0 so that if invalid parameter is given, validation fails

                // TODO better error handling, probably using ok_or() above
                if random_text_length > 0 {
                    TextSource::RandomText(random_text_length)
                } else {
                    TextSource::Error(
                        "The -t argument needs to be a positive integer greater than 0.",
                    )
                }
            }
            Some(1) => {
                let file_name = String::from(matches.value_of("text_from_file").unwrap_or(""));

                // TODO better error handling, probably using ok_or() above
                if !file_name.is_empty() {
                    TextSource::FromFile(file_name)
                } else {
                    TextSource::Error("The --tf argument needs a valid parameter.")
                }
            }
            Some(2) => {
                let file_name =
                    String::from(matches.value_of("text_from_file_binary").unwrap_or(""));

                // TODO better error handling, probably using ok_or() above
                if !file_name.is_empty() {
                    TextSource::FromFileBinary(file_name)
                } else {
                    TextSource::Error("The --tf argument needs a valid parameter.")
                }
            }
            None => TextSource::Error("You can only set one text source."),
            _ => TextSource::Error("Internal error while processing the text source."),
        }
    }
}

fn only_one(bools: &[bool]) -> bool {
    bools.iter().filter(|x| **x).count() == 1
}

fn none(bools: &[bool]) -> bool {
    bools.iter().filter(|x| **x).count() == 0
}

fn only(bools: &[bool]) -> Option<usize> {
    if only_one(bools) {
        Some(bools.iter().enumerate().find(|(_, x)| **x).unwrap().0)
    } else {
        None
    }
}
