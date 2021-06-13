pub type DeltaFunction = fn(isize, u8, &[u8]) -> isize;

pub type LpsDeltaFunction = fn(isize, u8, &[u8], &[isize]) -> isize;

/// Simulates a deterministic finite automaton (DFA) using a transition
/// function `delta`.
///
/// The function iterates over each character in the `text`, starting at `i0`,
/// and applies the delta function to each of those characters.
///
/// The algorithm terminates when the currently active state `q` is the last
/// state. It then returns for which character the final state has been reached
/// and therefore and occurrence has been found. If the final state could not
/// be reached by iterating over each character in `text`, `None` is returned.
#[allow(unused)]
pub fn dfa_with_delta(
    pattern: &[u8],
    text: &[u8],
    delta: DeltaFunction,
    i0: usize,
) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;

    // Iterate over i0..n
    for (i, c) in text.iter().enumerate().take(n).skip(i0) {
        q = delta(q, *c, pattern);

        if q == (m - 1) as isize {
            return Some(i - m + 1);
        }
    }

    None
}

/// Simulates a deterministic finite automation (DFA) using a transition
/// function `delta` which takes an lps table (used for KMP algorithm).
///
/// The function iterates over each character in the `text`, starting at `i0`,
/// and applies the delta function (with given lps table) to each
/// of those characters.
///
/// The algorithm terminates when the currently active state `q` is the last
/// state. It then returns for which character the final state has been reached
/// and therefore and occurrence has been found. If the finals tate could not
/// be reached by iterating over each character in `text`, `None` is returned.
///
/// Very similar to `fn dfa_with_delta`.
pub fn dfa_with_lps_delta(
    pattern: &[u8],
    text: &[u8],
    delta: LpsDeltaFunction,
    lps: &[isize],
    i0: usize,
) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;

    // Iterate over i0..n
    for (i, c) in text.iter().enumerate().take(n).skip(i0) {
        q = delta(q, *c, pattern, lps);

        if q == (m - 1) as isize {
            return Some(i - m + 1);
        }
    }

    None
}
