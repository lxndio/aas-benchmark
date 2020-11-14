/// Calculates the suffix array for a given text in `O(n^2 log n)` runtime.
///
/// This function calculates the suffix array by sorting the indices of all
/// characters in the text (and therefore all start indices of suffixes)
/// alphabetically.
pub fn slow(text: &[u8]) -> Vec<usize> {
    let mut pos: Vec<usize> = (0..text.len()).collect();

    pos.sort_by_key(|k| &text[*k..]);

    pos
}

/// Calculates the longest common prefix (lcp) array in `O(n^3)` runtime.
#[allow(unused)]
pub fn lcp_slow(text: &[u8], pos: &Vec<usize>) -> Vec<isize> {
    (0..text.len() + 1)
        .map(|r| lcp_slow_single(text, pos, r))
        .collect()
}

/// Calculates the longest common prefix (lcp) of the suffixes at positions
/// `r - 1` and `r` in the suffix array in `O(n^2)` runtime.
fn lcp_slow_single(text: &[u8], pos: &Vec<usize>, r: usize) -> isize {
    if r == 0 || r == text.len() {
        -1
    } else {
        // Get suffixes at positions r - 1 and r in the suffix array
        let pos_r1 = &text[pos[r - 1]..];
        let pos_r = &text[pos[r]..];

        let mut lcp = 0;

        // Count the length of the longest common prefix of those suffixes
        for i in 0..pos_r1.len().min(pos_r.len()) {
            if pos_r1[i] == pos_r[i] {
                lcp += 1;
            } else {
                break;
            }
        }

        lcp
    }
}
