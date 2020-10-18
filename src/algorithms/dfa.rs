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
pub fn dfa_with_delta(
    pattern: &[u8],
    text: &[u8],
    delta: fn(isize, u8, &[u8]) -> isize,
    i0: usize,
) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;

    for i in i0..n {
        q = delta(q, text[i], pattern);

        if q == (m - 1) as isize {
            return Some(i - m + 1);
        }
    }

    None
}
