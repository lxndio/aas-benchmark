use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use crate::cli::CLIParams;
use crate::generate::gen_rand_bytes;

#[derive(Debug, PartialEq)]
pub enum TextSource {
    RandomText(usize),
    FromFile(String),
    FromFileBinary(String),
    Error(&'static str),
}

/// Decides how a text should be generated based on the given CLI arguments
/// and calls the appropriate function.
pub fn generate_text(cli_params: &CLIParams) -> Result<Vec<u8>, String> {
    match &cli_params.text_source {
        TextSource::RandomText(n) => Ok(gen_rand_bytes(
            *n,
            cli_params.seed,
            Some(cli_params.alphabet_size),
            None,
        )),
        TextSource::FromFile(file_name) => match load_text_from_file(file_name) {
            Ok(text) => Ok(text),
            Err(err) => Err(err.to_string()),
        },
        TextSource::FromFileBinary(file_name) => Ok(load_text_from_file_binary(file_name)),
        TextSource::Error(err) => Err(String::from(*err)),
    }
}

/// Loads text from a file
fn load_text_from_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);

    let mut text: Vec<u8> = Vec::new();

    reader.read_to_end(&mut text)?;

    // Remove \n at the end if existing
    if *text.last().unwrap() == b'\n' {
        text.pop();
    }

    Ok(text)
}

fn load_text_from_file_binary(_file_name: &str) -> Vec<u8> {
    unimplemented!();
}
