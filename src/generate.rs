use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

/// Generates a byte vector containing random bytes.
///
/// Can take a u64 as a seed for random generation.
/// Takes either an alphabet size or an alphabet for generation.
pub fn gen_rand_bytes(
    n: usize,
    seed: Option<u64>,
    alphabet_size: Option<u8>,
    alphabet: Option<&Vec<u8>>,
) -> Vec<u8> {
    match seed {
        Some(seed) => {
            let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

            if let Some(alphabet_size) = alphabet_size {
                (0..n)
                    .map(|_| rng.gen_range(1, alphabet_size + 1))
                    .collect()
            } else if let Some(alphabet) = alphabet {
                (0..n)
                    .map(|_| alphabet.choose(&mut rng).unwrap())
                    .copied()
                    .collect()
            } else {
                Vec::new() // TODO Handle error correctly
            }
        }
        None => {
            let mut rng = rand::thread_rng();

            if let Some(alphabet_size) = alphabet_size {
                (0..n)
                    .map(|_| rng.gen_range(1, alphabet_size + 1))
                    .collect()
            } else if let Some(alphabet) = alphabet {
                (0..n)
                    .map(|_| alphabet.choose(&mut rng).unwrap())
                    .copied()
                    .collect()
            } else {
                Vec::new() // TODO Handle error correctly
            }
        }
    }
}

pub fn rand_pattern_from_bytes(bytes: &[u8], length: usize, seed: Option<u64>) -> &[u8] {
    let left;

    match seed {
        Some(seed) => {
            let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

            left = rng.gen_range(0, bytes.len() - length);
        }
        None => {
            let mut rng = rand::thread_rng();

            left = rng.gen_range(0, bytes.len() - length);
        }
    }

    &bytes[left..left + length]
}
