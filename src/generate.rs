use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

/// Generates a byte vector containing random bytes.
///
/// Can take a u64 as a seed for random generation.
pub fn gen_rand_bytes(n: usize, seed: Option<u64>, alphabet_size: u8) -> Vec<u8> {
    match seed {
        Some(seed) => {
            let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

            (0..n)
                .map(|_| rng.gen_range(1, alphabet_size + 1))
                .collect()
        }
        None => {
            let mut rng = rand::thread_rng();

            (0..n)
                .map(|_| rng.gen_range(1, alphabet_size + 1))
                .collect()
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
