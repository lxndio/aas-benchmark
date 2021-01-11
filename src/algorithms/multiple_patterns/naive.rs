use crate::algorithms::single_pattern::naive::naive_all;

/// Returns occurrences of given patterns in a text.
///
/// Takes multiple patterns and a text, returning a vector containing
/// vectors with the positions of the found occurrences for each pattern.
///
/// Uses a naive approach by simply calling the naive single pattern algorithm
/// for each given pattern.
pub fn naive_multiple(patterns: &[&[u8]], text: &[u8]) -> Vec<Vec<usize>> {
    let mut matches = Vec::new();

    for pattern in patterns {
        matches.push(naive_all(pattern, text));
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_multiple() {
        let text = b"gccttaacattattacgccta";
        let patterns: &[&[u8]] = &[b"tta", b"catta", b"gcct", b"abc"];

        let matches = naive_multiple(patterns, text);

        let matches_correct = vec![vec![3, 9, 12], vec![7], vec![0, 16], vec![]];

        assert_eq!(matches, matches_correct);
    }
}
