use std::cmp::min;

/// Calculates the suffix array for a given text in `O(n^2 log n)` runtime.
///
/// This function calculates the suffix array by sorting the indices of all
/// characters in the text (and therefore all start indices of suffixes)
/// alphabetically.
///
/// Make sure that the text contains a sentinel at the end which is a character
/// that is lexicographically smaller than any other character in the text.
pub fn slow(text: &[u8]) -> Vec<usize> {
    let mut pos: Vec<usize> = (0..text.len()).collect();

    pos.sort_by_key(|k| &text[*k..]);

    pos
}

/// Calculates the longest common prefix (lcp) array in `O(n^3)` runtime.
#[allow(unused)]
pub fn lcp_slow(text: &[u8], pos: &[usize]) -> Vec<isize> {
    (0..=text.len())
        .map(|r| lcp_slow_single(text, pos, r))
        .collect()
}

/// Calculates the longest common prefix (lcp) of the suffixes at positions
/// `r - 1` and `r` in the suffix array in `O(n^2)` runtime.
fn lcp_slow_single(text: &[u8], pos: &[usize], r: usize) -> isize {
    if r == 0 || r == text.len() {
        -1
    } else {
        // Get suffixes at positions r - 1 and r in the suffix array
        let pos_r1 = &text[pos[r - 1]..];
        let pos_r = &text[pos[r]..];

        let mut lcp = 0;

        // Count the length of the longest common prefix of those suffixes
        for i in 0..min(pos_r1.len(), pos_r.len()) {
            if pos_r1[i] == pos_r[i] {
                lcp += 1;
            } else {
                break;
            }
        }

        lcp
    }
}

/// Calculates the Burrows-Wheeler-Transformation (BWT) in `O(n)` runtime.
///
/// The BWT is defined as `r |-> b_r := text[pos[r] - 1]`, for `r` with
/// `pos[r] = 0` be `b_r := text[n - 1]` (equal to the sentinel).
pub fn bwt(text: &[u8], pos: &[usize]) -> Vec<u8> {
    (0..pos.len())
        .map(|r| {
            if pos[r] == 0 {
                text[text.len() - 1]
            } else {
                text[pos[r] - 1]
            }
        })
        .collect()
}

/// Calculates the `Occ[c, r]` vector specifying how often the letter `c` is
/// contained in the BWT's interval `[0, r]`.
///
/// As this two-dimensional vector is stored as a one-dimensional vector for
/// performance reasons, you can get the value `Occ[c, r]` by getting the
/// value at index `r * 256 + c` from the returned vector.
pub fn occ(bwt: &[u8]) -> Vec<usize> {
    let mut occ: Vec<usize> = Vec::with_capacity(256 * bwt.len());
    let counters: &mut [usize] = &mut [0; 256];

    for r in 0..bwt.len() {
        counters[bwt[r] as usize] += 1;

        occ.extend_from_slice(counters);
    }

    occ
}

pub fn occ_k(bwt: &[u8], k: usize) -> Vec<usize> {
    let mut occ: Vec<usize> = Vec::with_capacity(256 * ((bwt.len() / k) + 1));
    let counters: &mut [usize] = &mut [0; 256];

    for r in 0..=(bwt.len() / k) * k {
        counters[bwt[r] as usize] += 1;

        if r % k == 0 {
            occ.extend_from_slice(counters);
        }
    }

    occ
}

/// Calculates the `less[c]` vector specifying for a letter `c` how many
/// letters in the BWT are smaller than this letter.
pub fn less(bwt: &[u8]) -> Vec<usize> {
    let mut less: Vec<usize> = vec![0; 256];

    for c in bwt.iter() {
        for less_i in less.iter_mut().take(256).skip((*c as usize) + 1) {
            *less_i += 1;
        }
    }

    less
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slow() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();

        let pos_correct = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];

        assert_eq!(slow(text), pos_correct);
    }

    #[test]
    fn test_lcp_slow() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();
        let pos = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];

        let lcp_correct = vec![
            -1, 0, 1, 1, 2, 1, 4, 0, 1, 3, 1, 1, 2, 0, 4, 0, 2, 2, 2, 1, 3, 3, -1,
        ];

        assert_eq!(lcp_slow(text, &pos), lcp_correct);
    }

    #[test]
    fn test_bwt() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();
        let pos = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];

        let bwt_correct = "attattcaggaccc\u{0}ctttcaa".as_bytes();

        assert_eq!(bwt(text, &pos), bwt_correct);
    }

    #[test]
    fn test_occ() {
        let text = "abcbbc\u{0}".as_bytes();

        let mut occ_correct = vec![0; 256 * text.len()];
        occ_correct[0 * 256 + 'a' as usize] = 1;
        occ_correct[1 * 256 + 'a' as usize] = 1;
        occ_correct[1 * 256 + 'b' as usize] = 1;
        occ_correct[2 * 256 + 'a' as usize] = 1;
        occ_correct[2 * 256 + 'b' as usize] = 1;
        occ_correct[2 * 256 + 'c' as usize] = 1;
        occ_correct[3 * 256 + 'a' as usize] = 1;
        occ_correct[3 * 256 + 'b' as usize] = 2;
        occ_correct[3 * 256 + 'c' as usize] = 1;
        occ_correct[4 * 256 + 'a' as usize] = 1;
        occ_correct[4 * 256 + 'b' as usize] = 3;
        occ_correct[4 * 256 + 'c' as usize] = 1;
        occ_correct[5 * 256 + 'a' as usize] = 1;
        occ_correct[5 * 256 + 'b' as usize] = 3;
        occ_correct[5 * 256 + 'c' as usize] = 2;
        occ_correct[6 * 256 + 0] = 1;
        occ_correct[6 * 256 + 'a' as usize] = 1;
        occ_correct[6 * 256 + 'b' as usize] = 3;
        occ_correct[6 * 256 + 'c' as usize] = 2;

        assert_eq!(occ(text), occ_correct);
    }

    #[test]
    fn test_occ_k() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();
        let k = 4;

        let mut occ_correct = vec![0; 256 * ((text.len() / k) + 1)];
        occ_correct[0 * 256 + 'g' as usize] = 1;
        occ_correct[1 * 256 + 'c' as usize] = 2;
        occ_correct[1 * 256 + 'g' as usize] = 1;
        occ_correct[1 * 256 + 't' as usize] = 2;
        occ_correct[2 * 256 + 'a' as usize] = 3;
        occ_correct[2 * 256 + 'c' as usize] = 3;
        occ_correct[2 * 256 + 'g' as usize] = 1;
        occ_correct[2 * 256 + 't' as usize] = 2;
        occ_correct[3 * 256 + 'a' as usize] = 4;
        occ_correct[3 * 256 + 'c' as usize] = 3;
        occ_correct[3 * 256 + 'g' as usize] = 1;
        occ_correct[3 * 256 + 't' as usize] = 5;
        occ_correct[4 * 256 + 'a' as usize] = 5;
        occ_correct[4 * 256 + 'c' as usize] = 4;
        occ_correct[4 * 256 + 'g' as usize] = 2;
        occ_correct[4 * 256 + 't' as usize] = 6;
        occ_correct[5 * 256 + 'a' as usize] = 6;
        occ_correct[5 * 256 + 'c' as usize] = 6;
        occ_correct[5 * 256 + 'g' as usize] = 2;
        occ_correct[5 * 256 + 't' as usize] = 7;

        assert_eq!(occ_k(text, k), occ_correct);
    }

    #[test]
    fn test_less() {
        // text: "gccttaacattattacgccta\u{0}"
        let bwt_vec = "attattcaggaccc\u{0}ctttcaa".as_bytes();

        let mut less_correct = vec![0; 256];
        &less_correct[1..='a' as usize]
            .iter_mut()
            .for_each(|c| *c = 1);
        less_correct['b' as usize..='c' as usize]
            .iter_mut()
            .for_each(|c| *c = 7);
        &less_correct['d' as usize..='g' as usize]
            .iter_mut()
            .for_each(|c| *c = 13);
        &less_correct['h' as usize..='t' as usize]
            .iter_mut()
            .for_each(|c| *c = 15);
        &less_correct['u' as usize..=255]
            .iter_mut()
            .for_each(|c| *c = 22);

        assert_eq!(less(bwt_vec), less_correct);
    }
}
