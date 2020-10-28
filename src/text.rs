use crate::cli::CLIParams;
use crate::generate::gen_rand_bytes;

#[derive(PartialEq)]
pub enum TextSource {
    RandomText(usize),
    FromFile(String),
    FromFileBinary(String),
    Error(&'static str),
}

pub fn generate_text(cli_params: &CLIParams) -> Result<Vec<u8>, &str> {
    match cli_params.text_source {
        TextSource::RandomText(n) => Ok(gen_rand_bytes(n)),
        TextSource::FromFile(file_name) => Ok(load_text_from_file(file_name)),
        TextSource::FromFileBinary(file_name) => Ok(load_text_from_file_binary(file_name)),
        TextSource::Error(err) => Err(err),
    }
}

fn load_text_from_file(file_name: String) -> Vec<u8> {
    unimplemented!();
}

fn load_text_from_file_binary(file_name: String) -> Vec<u8> {
    unimplemented!();
}
