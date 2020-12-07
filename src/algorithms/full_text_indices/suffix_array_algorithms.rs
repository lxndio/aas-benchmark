use std::cmp::{max, min, Ordering};
use std::time::SystemTime;

use crate::algorithms::full_text_indices::suffix_array::{bwt, less, occ};
use crate::match_algorithm::{BWTAlgorithm, SuffixArrayAlgorithm};
use crate::measure::{Measurement, SingleMeasurement};

impl Measurement for SuffixArrayAlgorithm {
    /// A function to measure the runtime of an algorithm that requires a
    /// suffix array to work.
    ///
    /// It separately measures both the preparation time, i. e. the time it takes
    /// to generate the suffix array using the given suffix array generation
    /// function and the execution time, i. e. the time it takes to execute
    /// the actual algorithm itself.
    #[cfg(not(tarpaulin_include))]
    fn measure(pattern: &[u8], text: &[u8], f: &Self) -> SingleMeasurement {
        // Add sentinel to text
        let mut text = text.iter().copied().collect::<Vec<u8>>();
        text.push(0);
        let text = text.as_slice();

        // Measure time it takes to generate the suffix array
        let before = SystemTime::now();

        let pos = f.1(text);

        let preparation_duration = before.elapsed();

        // Measure time it takes to run the actual algorithm
        let before = SystemTime::now();

        let matches = f.0(&pos, pattern, text).len();

        let algorithm_duration = before.elapsed();

        (
            Some(preparation_duration.expect("Could not measure preparation time.")),
            algorithm_duration.expect("Could not measure time."),
            matches,
        )
    }
}

impl Measurement for BWTAlgorithm {
    /// A function to measure the runtime of an algorithm that requires a
    /// suffix array to work.
    ///
    /// It separately measures both the preparation time, i. e. the time it takes
    /// to generate the suffix array using the given suffix array generation
    /// function and the execution time, i. e. the time it takes to execute
    /// the actual algorithm itself.
    #[cfg(not(tarpaulin_include))]
    fn measure(pattern: &[u8], text: &[u8], f: &Self) -> SingleMeasurement {
        // Add sentinel to text
        let mut text = text.iter().copied().collect::<Vec<u8>>();
        text.push(0);
        let text = text.as_slice();

        // Measure time it takes to generate the suffix array
        let before = SystemTime::now();

        let pos = f.1(text);
        let bwt_vec = bwt(text, &pos);
        let occ_vec = occ(&bwt_vec);
        let less_vec = less(&bwt_vec);

        let preparation_duration = before.elapsed();

        // Measure time it takes to run the actual algorithm
        let before = SystemTime::now();

        let matches = f.0(&pos, &occ_vec, &less_vec, pattern).len();

        let algorithm_duration = before.elapsed();

        (
            Some(preparation_duration.expect("Could not measure preparation time.")),
            algorithm_duration.expect("Could not measure time."),
            matches,
        )
    }
}

/// Returns occurrences of a pattern in a text using a predefined suffix array.
///
/// The algorithm uses a modified binary search to find an interval in the
/// given suffix array including those suffixes which have a prefix equal
/// to the sought pattern. Using that interval, it then extracts the beginning
/// positions of the occurrences of the pattern in the text from the suffix array.
pub fn match_pattern(pos: &[usize], pattern: &[u8], text: &[u8]) -> Vec<usize> {
    // Define the binary search function as a local function
    // because it is only needed here
    fn binary_search(
        pos: &[usize],
        pattern: &[u8],
        text: &[u8],
        mut l: usize,
        mut r: usize,
    ) -> (usize, usize) {
        // At the beginning, the left interval bound is set as high as possible,
        // while the right bound is set as low as possible; this allows the
        // min and max functions below to be used to replace those bounds with
        // the correct ones or to detect that the interval is empty, if the left
        // bound is still larger than the right bound at the end
        let mut interval_l: usize = pos.len() - 1;
        let mut interval_r: usize = 0;

        // This is a normal binary search apart from the equal case
        while l <= r {
            let m = l + ((r - l) / 2);
            // Let prefix_n be the prefix of the pattern's length of the suffix
            // at the m-th position in the suffix array; can be shorter if the
            // suffix's length is shorter than the pattern's length
            let prefix_n = text[pos[m]..].iter().take(pattern.len());

            match prefix_n.cmp(pattern) {
                Ordering::Less => l = m + 1,
                Ordering::Greater => r = m - 1,
                Ordering::Equal => {
                    // As a match has now been found, include it in the interval
                    // by settings the according bounds to include the current value
                    if m < interval_l {
                        interval_l = m
                    }
                    if m > interval_r {
                        interval_r = m
                    }

                    // Because there can be multiple positions matching the
                    // search criterion, search to the left and to the right
                    // for more occurrences
                    let (l1, _) = binary_search(pos, pattern, text, l, m - 1);
                    let (_, r2) = binary_search(pos, pattern, text, m + 1, r);

                    // Now take the left-most left interval bound and
                    // the right-most right interval bound
                    interval_l = min(l1, interval_l);
                    interval_r = max(r2, interval_r);

                    return (interval_l, interval_r);
                }
            }
        }

        (interval_l, interval_r)
    }

    // Call the binary search function over the entire suffix array
    let (l, r) = binary_search(&pos, pattern, text, 0, pos.len() - 1);

    // Return empty vector if interval is empty (right bound is lower than left bound)
    if l <= r {
        pos[l..=r].to_vec()
    } else {
        Vec::new()
    }
}

pub fn match_pattern_bwt(
    pos: &[usize],
    occ: &[usize],
    less: &[usize],
    pattern: &[u8],
) -> Vec<usize> {
    let m = pattern.len();
    let n = occ.len() / 256;

    let mut c = pattern[m - 1];
    let mut left: usize = less[c as usize];
    let mut right: usize = less[c as usize] + occ[(n - 1) * 256 + c as usize] - 1;

    for i in (0..m - 1).rev() {
        c = pattern[i];

        left = less[c as usize] + occ[(left - 1) * 256 + c as usize];
        right = less[c as usize] + occ[right * 256 + c as usize] - 1;
    }

    pos[left..=right].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::algorithms::full_text_indices::suffix_array::{less, occ};

    #[test]
    fn test_match_pattern() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();
        let pos = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];
        let pattern = "tta".as_bytes();

        let mut matches = match_pattern(&pos, pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_match_pattern_nonexistent() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();
        let pos = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];
        let pattern = "abc".as_bytes();

        let mut matches = match_pattern(&pos, pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_match_pattern_bwt() {
        // Text: gccttaacattattacgccta\u{0}
        let pattern = "tta".as_bytes();

        let pos = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];
        let bwt_vec = "attattcaggaccc\u{0}ctttcaa".as_bytes();
        let occ_vec = occ(&bwt_vec);
        let less_vec = less(&bwt_vec);

        let mut matches = match_pattern_bwt(&pos, &occ_vec, &less_vec, pattern);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
