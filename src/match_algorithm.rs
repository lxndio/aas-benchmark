use std::collections::HashMap;

use crate::algorithms::multiple_patterns::aho_corasick::aho_corasick;
use crate::algorithms::single_pattern::bndm::bndm;
use crate::algorithms::single_pattern::horspool::horspool_all;
use crate::algorithms::single_pattern::kmp::{kmp_all, kmp_classic_all};
use crate::algorithms::single_pattern::naive::naive_all;
use crate::algorithms::single_pattern::shift_and::shift_and;

/// The signature of a pattern matching algorithm
pub type SinglePatternAlgorithm = fn(&[u8], &[u8]) -> Vec<usize>;

pub type MultiplePatternsAlgorithm = fn(Vec<&[u8]>, &[u8]) -> Vec<Vec<usize>>;

pub enum TypedAlgorithm {
    SinglePattern(SinglePatternAlgorithm),
    MultiplePatterns(MultiplePatternsAlgorithm),
}

lazy_static! {
    /// List of existing algorithms and their internal names
    static ref SINGLE_PATTERN_ALGORITHMS: HashMap<&'static str, SinglePatternAlgorithm> = hashmap! {
        "bndm" => bndm as SinglePatternAlgorithm,
        "horspool" => horspool_all as SinglePatternAlgorithm,
        "naive" => naive_all as SinglePatternAlgorithm,
        "kmp" => kmp_all as SinglePatternAlgorithm,
        "kmp-classic" => kmp_classic_all as SinglePatternAlgorithm,
        "shift-and" => shift_and as SinglePatternAlgorithm,
    };

    static ref MULTIPLE_PATTERNS_ALGORITHMS: HashMap<&'static str, MultiplePatternsAlgorithm> = hashmap! {
        "aho-corasick" => aho_corasick as MultiplePatternsAlgorithm,
    };
}

/// Returns the algorithm function matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by
/// the user as a CLI parameter.
///
/// It returns the algorithm function matching the name or `None`
/// if there is no algorithm with the given name.
pub fn match_algorithm(algorithm: &str) -> Option<TypedAlgorithm> {
    if SINGLE_PATTERN_ALGORITHMS.contains_key(algorithm) {
        Some(TypedAlgorithm::SinglePattern(
            *SINGLE_PATTERN_ALGORITHMS.get(algorithm).unwrap(),
        ))
    } else if MULTIPLE_PATTERNS_ALGORITHMS.contains_key(algorithm) {
        Some(TypedAlgorithm::MultiplePatterns(
            *MULTIPLE_PATTERNS_ALGORITHMS.get(algorithm).unwrap(),
        ))
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
        // TODO with multiple pattern algorithms now added, all becomes more
        // complicated, so currently it does only add single pattern algorithms
        if algorithm_name == "all" {
            for (algorithm_name, algorithm) in SINGLE_PATTERN_ALGORITHMS.iter() {
                algorithms.push((
                    algorithm_name.to_string(),
                    TypedAlgorithm::SinglePattern(*algorithm),
                ));
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
/// etc.) or `"Unknown SinglePatternAlgorithm"` if there is no
/// algorithm with the given name.
pub fn algorithm_name(algorithm: &str) -> &str {
    match algorithm.to_lowercase().as_str() {
        "aho-corasick" => "Aho-Corasick",
        "bndm" => "BNDM",
        "horspool" => "Horspool",
        "naive" => "Naive",
        "kmp" => "KMP",
        "kmp-classic" => "Classic KMP",
        "shift-and" => "Shift-And",
        _ => "Unknown Algorithm",
    }
}
