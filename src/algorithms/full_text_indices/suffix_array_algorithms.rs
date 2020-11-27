use std::cmp::Ordering;
use std::time::SystemTime;

use crate::match_algorithm::SuffixArrayAlgorithm;
use crate::measure::{Measurement, SingleMeasurement};

/// Measurement for slow suffix array algorithms as the preparation time,
/// i. e. the time it takes to generate the suffix array, should also
/// be measured.
///
/// The **slow** suffix array algorithms measurement uses the slow approach
/// to generating the suffix array.
impl Measurement for SuffixArrayAlgorithm {
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

        let matches = f.0(pos, pattern, text).len();

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
pub fn match_pattern(pos: Vec<usize>, pattern: &[u8], text: &[u8]) -> Vec<usize> {
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
                    interval_l = l1.min(interval_l);
                    interval_r = r2.max(interval_r);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::full_text_indices::sais::fast;
    use crate::algorithms::single_pattern::naive::naive_all;
    use crate::generate::{gen_rand_bytes, rand_pattern_from_bytes};

    #[test]
    fn test_match_pattern() {
        for i in 0..10 {
            println!("Test {}", i);

            let mut text = gen_rand_bytes(1_000_000, None);
            text.push(0);
            let text = text.as_slice();

            let pattern = rand_pattern_from_bytes(text, 2, None);

            let pos = fast(text);

            let mut matches1 = match_pattern(pos, pattern, text);
            let mut matches2 = naive_all(pattern, text);

            // Sort both lists because order could be different resulting
            // in a failure of this test
            matches1.sort_unstable();
            matches2.sort_unstable();

            println!("{:?}, {:?}", matches1, matches2);

            assert_eq!(matches1, matches2);
        }
    }
}
