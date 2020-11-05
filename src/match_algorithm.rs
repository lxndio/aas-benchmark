use std::collections::HashMap;

use crate::algorithms::single_pattern::horspool::horspool_all;
use crate::algorithms::single_pattern::kmp::{kmp_all, kmp_classic_all};
use crate::algorithms::single_pattern::naive::naive_all;
use crate::algorithms::single_pattern::shift_and::shift_and;

/// The signature of a pattern matching algorithm
pub type Algorithm = fn(&[u8], &[u8]) -> Vec<usize>;

lazy_static! {
    static ref ALGORITHMS: HashMap<&'static str, Algorithm> = hashmap! {
        "horspool" => horspool_all as Algorithm,
        "naive" => naive_all as Algorithm,
        "kmp" => kmp_all as Algorithm,
        "kmp-classic" => kmp_classic_all as Algorithm,
        "shift-and" => shift_and as Algorithm,
    };
}

/// Returns the algorithm function matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by
/// the user as a CLI parameter.
///
/// It returns the algorithm function matching the name or `None`
/// if there is no algorithm with the given name.
pub fn match_algorithm(algorithm: &str) -> Option<Algorithm> {
    if ALGORITHMS.contains_key(algorithm) {
        Some(*ALGORITHMS.get(algorithm).unwrap())
    } else {
        None
    }
}

pub fn match_algorithms(algorithm_names: &Vec<String>) -> Vec<(String, Algorithm)> {
    let mut algorithms = Vec::new();

    for algorithm_name in algorithm_names.iter() {
        // Special case for adding all algorithms
        if algorithm_name == "all" {
            for (algorithm_name, algorithm) in ALGORITHMS.iter() {
                algorithms.push((algorithm_name.to_string(), *algorithm));
            }
        } else if let Some(algorithm) = match_algorithm(algorithm_name) {
            algorithms.push((algorithm_name.to_string(), algorithm));
        }
    }

    algorithms
}

/// Returns the pretty formatted name of an algorithm matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by
/// the user as a CLI parameter.
///
/// It returns the pretty formatted name of the algorithm (containing spaces
/// etc.) or `"Unknown Algorithm"` if there is no
/// algorithm with the given name.
pub fn algorithm_name(algorithm: &str) -> &str {
    match algorithm.to_lowercase().as_str() {
        "horspool" => "Horspool",
        "naive" => "Naive",
        "kmp" => "KMP",
        "kmp-classic" => "Classic KMP",
        "shift-and" => "Shift-And",
        _ => "Unknown Algorithm",
    }
}
