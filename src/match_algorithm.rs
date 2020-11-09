use std::collections::HashMap;

use crate::algorithms::full_text_indices::suffix_array_algorithms::match_pattern_slow_pos;
use crate::algorithms::single_pattern::bndm::bndm;
use crate::algorithms::single_pattern::horspool::horspool_all;
use crate::algorithms::single_pattern::kmp::{kmp_all, kmp_classic_all};
use crate::algorithms::single_pattern::naive::naive_all;
use crate::algorithms::single_pattern::shift_and::shift_and;

/// The signature of a pattern matching algorithm
pub type Algorithm = fn(&[u8], &[u8]) -> Vec<usize>;

lazy_static! {
    /// List of existing algorithms and their internal names
    static ref ALGORITHMS: HashMap<&'static str, Algorithm> = hashmap! {
        "bndm" => bndm as Algorithm,
        "horspool" => horspool_all as Algorithm,
        "naive" => naive_all as Algorithm,
        "kmp" => kmp_all as Algorithm,
        "kmp-classic" => kmp_classic_all as Algorithm,
        "shift-and" => shift_and as Algorithm,
        "suffix-array" => match_pattern_slow_pos as Algorithm,
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

/// Returns the algorithm functions and names matching the given names.
///
/// The functions takes a `&Vec<String>` containing algorithm names given by
/// the user as a CLI parameter.
///
/// It returns a Vec of tuples containing the names and algorithm functions
/// of the algorithms matched by the given `algorithm_names`.
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
        "bndm" => "BNDM",
        "horspool" => "Horspool",
        "naive" => "Naive",
        "kmp" => "KMP",
        "kmp-classic" => "Classic KMP",
        "shift-and" => "Shift-And",
        "suffix-array" => "Suffix Array",
        _ => "Unknown Algorithm",
    }
}
