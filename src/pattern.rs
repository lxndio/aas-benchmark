use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::cli::CLIParams;
use crate::generate::{gen_rand_bytes, rand_pattern_from_bytes};
use crate::range::Range;

#[derive(Debug, PartialEq)]
pub enum PatternSource {
    FromArgument(Vec<String>),
    FromFile(String, bool),
    FromText(Range),
    FromTextRandom(Range),
    Random(Range),
    Error(&'static str),
}

/// Decides how a pattern should be generated based on the given CLI arguments
/// and calls the appropriate function.
#[cfg(not(tarpaulin_include))]
pub fn generate_patterns(cli_params: &'_ CLIParams, text: &[u8]) -> Result<Vec<Vec<u8>>, String> {
    match &cli_params.pattern_source {
        PatternSource::FromArgument(patterns) => {
            Ok(patterns.iter().map(|x| x.as_bytes().to_vec()).collect())
        }
        PatternSource::FromFile(file_name, false) => match load_pattern_from_file(file_name) {
            Ok(pattern) => Ok(vec![pattern]),
            Err(err) => Err(err.to_string()),
        },
        PatternSource::FromFile(file_name, true) => match load_patterns_from_file(file_name) {
            Ok(patterns) => Ok(patterns),
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

/// Loads pattern from a file.
fn load_pattern_from_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut pattern: Vec<u8> = Vec::new();

    reader.read_to_end(&mut pattern)?;

    Ok(pattern)
}

/// Load patterns from a file, one pattern per line.
fn load_patterns_from_file(file_name: &str) -> std::io::Result<Vec<Vec<u8>>> {
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);

    let mut patterns: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        patterns.push(line?.chars().map(|c| c as u8).collect());
    }

    Ok(patterns)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_load_pattern_from_file() -> std::io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        write!(temp_file, "gccttaacattattacgccta")?;
        temp_file.flush()?;

        let file_path = temp_file.into_temp_path();
        let file_name = file_path.to_str().unwrap();

        let pattern = load_pattern_from_file(file_name);
        let pattern_correct = b"gccttaacattattacgccta".to_vec();

        assert!(pattern.is_ok());
        assert_eq!(pattern.unwrap(), pattern_correct);

        Ok(())
    }

    #[test]
    fn test_load_patterns_from_file() -> std::io::Result<()> {
        let mut temp_file = NamedTempFile::new()?;
        writeln!(temp_file, "gccttaacattattacgccta")?;
        writeln!(temp_file, "tattacgccta")?;
        writeln!(temp_file, "gccttaacgccta")?;
        writeln!(temp_file, "gccc")?;
        writeln!(temp_file, "gccttaacattattacgcctagccttaacattattacgccta")?;
        temp_file.flush()?;

        let file_path = temp_file.into_temp_path();
        let file_name = file_path.to_str().unwrap();

        let patterns = load_patterns_from_file(file_name);
        let patterns_correct: Vec<Vec<u8>> = vec![
            b"gccttaacattattacgccta".to_vec(),
            b"tattacgccta".to_vec(),
            b"gccttaacgccta".to_vec(),
            b"gccc".to_vec(),
            b"gccttaacattattacgcctagccttaacattattacgccta".to_vec(),
        ];

        assert!(patterns.is_ok());
        assert_eq!(patterns.unwrap(), patterns_correct);

        Ok(())
    }
}
