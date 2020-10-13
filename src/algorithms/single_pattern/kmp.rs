pub fn DFA_with_delta(
    pattern: &[u8],
    text: &[u8],
    delta: fn(isize, u8) -> isize,
    i0: usize,
) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;

    for i in i0..n {
        q = delta(q, text[i]);

        if q == (m - 1) as isize {
            return Some(i - m + 1);
        }
    }

    None
}
