use std::collections::HashMap;

use crate::algorithms::approximative::error_tolerant_shift_and::error_tolerant_shift_and;
use crate::algorithms::approximative::ukkonen::ukkonen;
use crate::algorithms::full_text_indices::sais::fast;
use crate::algorithms::full_text_indices::suffix_array::slow;
use crate::algorithms::full_text_indices::suffix_array_algorithms::{
    match_pattern, match_pattern_bwt,
};
use crate::algorithms::multiple_patterns::aho_corasick::aho_corasick;
use crate::algorithms::multiple_patterns::naive::naive_multiple;
use crate::algorithms::single_pattern::blim::blim;
use crate::algorithms::single_pattern::bndm::bndm;
use crate::algorithms::single_pattern::bom::bom;
use crate::algorithms::single_pattern::boyer_moore::{
    weak_boyer_moore_all, weak_memorizing_boyer_moore_all, weak_turbo_boyer_moore_all,
};
use crate::algorithms::single_pattern::double_window::double_window;
use crate::algorithms::single_pattern::horspool::horspool_all;
use crate::algorithms::single_pattern::kmp::{kmp_all, kmp_classic_all};
use crate::algorithms::single_pattern::naive::naive_all;
use crate::algorithms::single_pattern::shift_and::shift_and;

lazy_static! {
    /// List of existing algorithms and their internal names
    static ref ALGORITHMS: HashMap<&'static str, TypedAlgorithm> = hashmap! {
        "bndm" => TypedAlgorithm::SinglePatternAlgorithm(bndm),
        "horspool" => TypedAlgorithm::SinglePatternAlgorithm(horspool_all),
        "naive" => TypedAlgorithm::SinglePatternAlgorithm(naive_all),
        "wbm" => TypedAlgorithm::SinglePatternAlgorithm(weak_boyer_moore_all),
        "wmbm" => TypedAlgorithm::SinglePatternAlgorithm(weak_memorizing_boyer_moore_all),
        "wtbm" => TypedAlgorithm::SinglePatternAlgorithm(weak_turbo_boyer_moore_all),
        "kmp" => TypedAlgorithm::SinglePatternAlgorithm(kmp_all),
        "kmp-classic" => TypedAlgorithm::SinglePatternAlgorithm(kmp_classic_all),
        "shift-and" => TypedAlgorithm::SinglePatternAlgorithm(shift_and),
        "sa-match" => TypedAlgorithm::SuffixArrayAlgorithm(match_pattern),
        "bwt-match" => TypedAlgorithm::BWTAlgorithm(match_pattern_bwt),
        "ukkonen" => TypedAlgorithm::ApproximativeAlgorithm(ukkonen),
        "et-shift-and" => TypedAlgorithm::ApproximativeAlgorithm(error_tolerant_shift_and),
        "mp-naive" => TypedAlgorithm::MultiplePatternAlgorithm(naive_multiple),
        "aho-corasick" => TypedAlgorithm::MultiplePatternAlgorithm(aho_corasick),
        "bom" => TypedAlgorithm::SinglePatternAlgorithm(bom),
        "dw" => TypedAlgorithm::SinglePatternAlgorithm(double_window),
        "blim" => TypedAlgorithm::SinglePatternAlgorithm(blim),
    };

    /// List of suffix array generation algorithms and their internal names
    static ref SUFFIX_ARRAY_GEN_ALGORITHMS: HashMap<&'static str, SuffixArrayGenAlgorithm> = hashmap! {
        "naive" => slow as SuffixArrayGenAlgorithm,
        "sais" => fast as SuffixArrayGenAlgorithm,
    };

    /// List of algorithm names
    static ref ALGORITHM_NAMES: HashMap<&'static str, &'static str> = hashmap! {
        "bndm" => "BNDM",
        "horspool" => "Horspool",
        "naive" => "Naive",
        "wbm" => "Weak Boyer Moore",
        "wmbm" => "Weak Memorizing Boyer Moore",
        "wtbm" => "Weak Turbo Boyer Moore",
        "kmp" => "KMP",
        "kmp-classic" => "Classic KMP",
        "shift-and" => "Shift-And",
        "sa-match" => "SA Pattern Matching",
        "bwt-match" => "BWT Pattern Matching",
        "ukkonen" => "Ukkonen's DP Algorithm",
        "et-shift-and" => "Error Tolerant Shift-And",
        "mp-naive" => "Naive Multiple Patterns",
        "aho-corasick" => "Aho-Corasick",
        "bom" => "BOM",
        "dw" => "Double Window",
        "blim" => "BLIM",
    };
}

/// A single pattern algorithm.
pub type SinglePatternAlgorithm = fn(&[u8], &[u8]) -> Vec<usize>;

/// A multiple pattern algorithm.
pub type MultiplePatternAlgorithm = fn(&[Vec<u8>], &[u8]) -> Vec<Vec<usize>>;

/// A suffix array algorithm tuple, containing the algorithm itself and
/// the suffix array generation function to be used.
pub type SuffixArrayAlgorithm = fn(&[usize], &[u8], &[u8]) -> Vec<usize>;

/// A BWT algorithm.
pub type BWTAlgorithm = fn(&[usize], &[usize], &[usize], &[u8]) -> Vec<usize>;

/// An approximative algorithm.
pub type ApproximativeAlgorithm = fn(&[u8], &[u8], usize) -> Vec<(usize, usize)>;

/// A suffix array generation algorithm.
pub type SuffixArrayGenAlgorithm = fn(&[u8]) -> Vec<usize>;

#[derive(Clone, Copy)]
pub enum TypedAlgorithm {
    SinglePatternAlgorithm(SinglePatternAlgorithm),
    MultiplePatternAlgorithm(MultiplePatternAlgorithm),
    SuffixArrayAlgorithm(SuffixArrayAlgorithm),
    BWTAlgorithm(BWTAlgorithm),
    ApproximativeAlgorithm(ApproximativeAlgorithm),
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
pub fn match_algorithms(algorithm_names: &[String]) -> Vec<(String, TypedAlgorithm)> {
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

/// Returns the suffix array generation algorithm function matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by the user
/// as a CLI parameter.
///
/// It returns the suffix array generation algorithm function matching the name
/// or `None` if there is no algorithm with the given name.
pub(crate) fn match_suffix_array_gen_algorithm(algorithm: &str) -> Option<SuffixArrayGenAlgorithm> {
    if SUFFIX_ARRAY_GEN_ALGORITHMS.contains_key(algorithm) {
        Some(*SUFFIX_ARRAY_GEN_ALGORITHMS.get(algorithm).unwrap())
    } else {
        None
    }
}

/// Returns the pretty formatted name of an algorithm matching the given name.
///
/// The function takes a `&str` containing an algorithm name given by
/// the user as a CLI parameter.
///
/// It returns the nicely formatted name of the algorithm (containing spaces
/// etc.) or `"Unknown Algorithm"` if there is no algorithm with the given name.
pub fn algorithm_name(algorithm: &str) -> &str {
    if ALGORITHM_NAMES.contains_key(algorithm) {
        ALGORITHM_NAMES.get(algorithm).unwrap()
    } else {
        "Unknown Algorithm"
    }
}
