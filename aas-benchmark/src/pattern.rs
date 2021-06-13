use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::cli::CLIParams;
use crate::generate::{gen_rand_bytes, rand_pattern_from_bytes};
use crate::range::Range;

#[derive(Debug, PartialEq)]
pub enum PatternSource {
    FromArgument(String),
    FromFile(String),
    FromText(Range),
    FromTextRandom(Range),
    Random(Range),
    Error(&'static str),
}

/// Decides how a pattern should be generated based on the given CLI arguments
/// and calls the appropriate function.
pub fn generate_patterns<'a>(
    cli_params: &'a CLIParams,
    text: &[u8],
) -> Result<Vec<Vec<u8>>, String> {
    match &cli_params.pattern_source {
        PatternSource::FromArgument(pattern) => Ok(vec![pattern.as_bytes().to_vec()]),
        PatternSource::FromFile(file_name) => match load_pattern_from_file(file_name) {
            Ok(pattern) => Ok(vec![pattern]),
            Err(err) => Err(err.to_string()),
        },
        PatternSource::FromText(range) => {
            let start = range.start;
            let end = range.end;

            // This check is sufficient because the pattern source validation check
            // requires pattern_from_text_range to be a non-empty range,
            // i.e. start is less than end
            if end < text.len() {
                Ok(vec![(&text[start..end]).to_vec()])
            } else {
                // TODO This should be checked in set_pattern_source()
                Err(String::from("Pattern range end is out of text bounds."))
            }
        }
        PatternSource::FromTextRandom(range) => {
            let mut patterns = Vec::new();

            for length in range.iter() {
                patterns.push(rand_pattern_from_bytes(text, length, cli_params.seed).to_vec());
            }

            Ok(patterns)
        }
        PatternSource::Random(range) => {
            let mut patterns = Vec::new();

            for length in range.iter() {
                patterns.push(gen_rand_bytes(length, cli_params.seed));
            }

            Ok(patterns)
        }
        PatternSource::Error(err) => Err(String::from(*err)),
    }
}

/// Loads pattern from a file
fn load_pattern_from_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut pattern: Vec<u8> = Vec::new();

    reader.read_to_end(&mut pattern)?;

    Ok(pattern)
}
