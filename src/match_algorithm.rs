use crate::algorithms::single_pattern::kmp::kmp_classic_all;
use crate::algorithms::single_pattern::naive::naive_all;

pub fn match_algorithm(algorithm: &str) -> Option<fn(&[u8], &[u8]) -> Vec<usize>> {
    match algorithm.to_lowercase().as_str() {
        "naive" => Some(naive_all as fn(&[u8], &[u8]) -> Vec<usize>),
        "kmp-classic" => Some(kmp_classic_all as fn(&[u8], &[u8]) -> Vec<usize>),
        _ => None,
    }
}

pub fn algorithm_name(algorithm: &str) -> &str {
    match algorithm.to_lowercase().as_str() {
        "naive" => "Naive",
        "kmp-classic" => "Classic KMP",
        _ => "Unknown Algorithm",
    }
}
