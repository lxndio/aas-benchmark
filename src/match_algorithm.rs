use std::collections::HashMap;

use crate::algorithms::full_text_indices::suffix_array::slow;
use crate::algorithms::full_text_indices::suffix_array_algorithms::match_pattern;
use crate::algorithms::single_pattern::bndm::bndm;
use crate::algorithms::single_pattern::horspool::horspool_all;
use crate::algorithms::single_pattern::kmp::{kmp_all, kmp_classic_all};
use crate::algorithms::single_pattern::naive::naive_all;
use crate::algorithms::single_pattern::shift_and::shift_and;

lazy_static! {
    /// List of existing algorithms and their internal names
    static ref ALGORITHMS: HashMap<&'static str, TypedAlgorithm> = hashmap! {
        "bndm" => TypedAlgorithm::SinglePatternAlgorithm(bndm as SinglePatternAlgorithm),
        "horspool" => TypedAlgorithm::SinglePatternAlgorithm(horspool_all as SinglePatternAlgorithm),
        "naive" => TypedAlgorithm::SinglePatternAlgorithm(naive_all as SinglePatternAlgorithm),
        "kmp" => TypedAlgorithm::SinglePatternAlgorithm(kmp_all as SinglePatternAlgorithm),
        "kmp-classic" => TypedAlgorithm::SinglePatternAlgorithm(kmp_classic_all as SinglePatternAlgorithm),
        "shift-and" => TypedAlgorithm::SinglePatternAlgorithm(shift_and as SinglePatternAlgorithm),
        "sa_match_slow" => TypedAlgorithm::SlowSuffixArrayAlgorithm(match_pattern as SlowSuffixArrayAlgorithm),
    };
}

/// The signature of an algorithm matching a
pub type SinglePatternAlgorithm = fn(&[u8], &[u8]) -> Vec<usize>;

pub type SlowSuffixArrayAlgorithm = fn(Vec<usize>, &[u8], &[u8]) -> Vec<usize>;

#[derive(Clone, Copy)]
pub enum TypedAlgorithm {
    SinglePatternAlgorithm(SinglePatternAlgorithm),
    SlowSuffixArrayAlgorithm(SlowSuffixArrayAlgorithm),
}

/// Returns the algorithm function matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by
/// the user as a CLI parameter.
///
/// It returns the algorithm function matching the name or `None`
/// if there is no algorithm with the given name.
pub fn match_algorithm(algorithm: &str) -> Option<TypedAlgorithm> {
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
pub fn match_algorithms(algorithm_names: &Vec<String>) -> Vec<(String, TypedAlgorithm)> {
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
        "sa_match_slow" => "Slow SA Pattern Matching",
        _ => "Unknown Algorithm",
    }
}
