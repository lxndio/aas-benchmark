use crate::cli::CLIParams;
use crate::generate::rand_pattern_from_bytes;
use crate::range::Range;

#[derive(Debug, PartialEq)]
pub enum PatternSource {
    FromText(Range),
    FromTextRandom(Range),
    Error(&'static str),
}

/// Decides how a pattern should be generated based on the given CLI arguments
/// and calls the appropriate function.
pub fn generate_patterns<'a>(
    cli_params: &CLIParams,
    text: &'a [u8],
) -> Result<Vec<&'a [u8]>, String> {
    match &cli_params.pattern_source {
        PatternSource::FromText(range) => {
            let start = range.start;
            let end = range.end;

            // This check is sufficient because the pattern source validation check
            // requires pattern_from_text_range to be a non-empty range,
            // i.e. start is less than end
            if end < text.len() {
                Ok(vec![&text[start..end]])
            } else {
                // TODO This should be checked in set_pattern_source()
                Err(String::from("Pattern range end is out of text bounds."))
            }
        }
        PatternSource::FromTextRandom(range) => {
            let mut patterns = Vec::new();

            for length in range.iter() {
                patterns.push(rand_pattern_from_bytes(text, length));
            }

            Ok(patterns)
        }
        PatternSource::Error(err) => Err(String::from(*err)),
    }
}
