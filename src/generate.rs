use rand::Rng;

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

pub fn rand_pattern_from_bytes(bytes: &[u8], length: usize) -> &[u8] {
    let mut rng = rand::thread_rng();

    let left = rng.gen_range(0, bytes.len() - length);

    &bytes[left..left + length]
}
