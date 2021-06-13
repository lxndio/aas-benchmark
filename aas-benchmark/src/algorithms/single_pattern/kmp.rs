use benchmark_lists::aslice::ASlice;

use crate::algorithms::dfa::dfa_with_lps_delta;

/// Computes the lps function used by the KMP algorithm.
///
/// The lps function is defined as `lps(q)` being the length of the **l**ongest
/// **p**refix of the `pattern` that is a proper **s**uffix of `pattern[0..q]`
/// (or empty).
///
/// Returns a `Vec<isize>` containing the lps function values corresponding to
/// the indices.
pub fn kmp_compute_lps(pattern: &[u8]) -> Vec<isize> {
    let m = pattern.len();

    let mut q: isize = -1;
    let mut lps: Vec<isize> = vec![0; m];

    for i in 1..m {
        while q > -1 && pattern[(q + 1) as usize] != pattern[i] {
            q = lps[q as usize] - 1; // as usize is safe because while condition is q > -1
        }

        if pattern[(q + 1) as usize] == pattern[i] {
            q += 1;
        }

        lps[i] = q + 1;
    }

    lps
}

/// Simulates a DFA delta function using a given lps function.
///
/// It takes a state `q`, a character `c`, a `pattern` and a `Vec<isize>`
/// containing the values of an lps function.
///
/// Returns the new active state after transitioning by simulating a delta
/// function using the given lps function.
pub fn dfa_delta_lps(q: isize, c: u8, pattern: &[u8], lps: &[isize]) -> isize {
    let m = pattern.len();

    let mut q = q;

    while q == m as isize - 1 || (pattern[(q + 1) as usize] != c && q > -1) {
        q = lps[q as usize] - 1;
    }

    if pattern[(q + 1) as usize] == c {
        q += 1;
    }

    q
}

/// An implementation of the Knuth-Morris-Pratt algorithm (KMP).
///
/// It generates the lps function and then runs a DFA using `dfa_delta_lps`,
/// a delta function that takes the lps function to calculate states.
pub fn kmp(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let lps = kmp_compute_lps(pattern);

    dfa_with_lps_delta(pattern, text, dfa_delta_lps, &lps, i0)
}

/// Calculates all occurrences of a given pattern in a text by executing
/// the KMP algorithm multiple times.
///
/// The `i0` value starts at 0 and is increased after each execution to
/// start the next execution right after the current occurrence's position
/// in text.
pub fn kmp_all(pattern: &[u8], text: &mut ASlice<u8>) -> Vec<usize> {
    // TODO remove this
    let text: &[u8] = &text.to_vec();

    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = kmp(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}

/// The classic implementation of the Knuth-Morris-Pratt algorithm (KMP).
///
/// It searches for the first occurrence of `pattern` in `text` starting at
/// index `i0` of the text.
///
/// After an occurrence has been found, the algorithm returns the index
/// marking the first character of the occurrence and therefore terminates.
/// If the pattern could not be found in the `text`, `None` is returned.
pub fn kmp_classic(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;
    let lps = kmp_compute_lps(pattern);

    // Iterate over i0..n
    for (i, c) in text.iter().enumerate().take(n).skip(i0) {
        while q == m as isize - 1 || (pattern[(q + 1) as usize] != *c && q > -1) {
            q = lps[q as usize] - 1;
        }

        if pattern[(q + 1) as usize] == *c {
            q += 1;
        }

        if q == (m - 1) as isize {
            return Some(i + 1 - m);
        }
    }

    None
}

pub fn kmp_classic_all(pattern: &[u8], text: &mut ASlice<u8>) -> Vec<usize> {
    // TODO remove this
    let text: &[u8] = &text.to_vec();

    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = kmp_classic(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmp_all() {
        let text = b"gccttaacattattacgccta\0";
        let pattern = b"tta";

        let mut matches = kmp_all(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_kmp_classic_all() {
        let text = b"gccttaacattattacgccta\0";
        let pattern = b"tta";

        let mut matches = kmp_classic_all(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
