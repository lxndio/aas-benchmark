use rand::Rng;

use crate::cli::CLIParams;

/// Generates a `Vec<u8>` containing `n` random bytes.
pub fn gen_rand_bytes(n: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let res: Vec<u8> = (0..n).map(|_| rng.gen_range(0, 255)).collect();

    res
}

#[allow(unused)]
pub fn gen_rand_bytes_seed(n: usize, seed: usize) -> Vec<u8> {
    unimplemented!();
}

fn rand_pattern_from_bytes(bytes: &[u8], length: usize) -> &[u8] {
    let mut rng = rand::thread_rng();

    let left = rng.gen_range(0, bytes.len() - length);

    &bytes[left..left + length]
}

pub fn gen_pattern<'a>(text: &'a [u8], cli_params: &CLIParams) -> Option<&'a [u8]> {
    let pattern;

    if cli_params.random_pattern_from_text {
        pattern = Some(rand_pattern_from_bytes(
            text,
            cli_params.random_pattern_from_text_length,
        ));
    } else if cli_params.pattern_from_text {
        let start = cli_params.pattern_from_text_range.start;
        let end = cli_params.pattern_from_text_range.end;

        // This check is sufficient because the CLIParams validation check
        // requires pattern_from_text_range to be a non-empty range,
        // i.e. start is less than end
        if end < text.len() {
            pattern = Some(&text[start..end]);
        } else {
            pattern = None;
        }
    } else {
        pattern = None;
    }

    pattern
}
