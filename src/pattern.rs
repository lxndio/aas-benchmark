use crate::cli::CLIParams;
use crate::range::Range;

#[derive(PartialEq)]
pub enum PatternSource {
    FromText(Range),
    FromTextRandom(Range),
    Error(&'static str),
}

/// Decides how a pattern should be generated based on the given CLI arguments
/// and calls the appropriate function.
pub fn generate_pattern<'a>(
    cli_params: &CLIParams,
    text: &'a [u8],
) -> Result<Vec<&'a [u8]>, String> {
    unimplemented!();
}
