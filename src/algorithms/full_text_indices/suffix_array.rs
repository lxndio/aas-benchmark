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
    (0..text.len() + 1)
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

/// Calculates the Burrows-Wheeler-Transformation (BWT) in `O(n)` runtime.
///
/// The BWT is defined as `r |-> b_r := text[pos[r] - 1]`, for `r` with
/// `pos[r] = 0` be `b_r := s[n - 1]` (equal to the sentinel).
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

/// Calculates the `Occ[a, r]` vector specifying how often the letter `a` is
/// contained in the BWT's interval `[0, r]`.
///
/// As this two-dimensional vector is stored as a one-dimensional vector for
/// performance reasons, you can get the value `Occ[a, r]` by getting the
/// value at index `r * 256 + a` from the returned vector.
pub fn occ(bwt: &[u8]) -> Vec<usize> {
    let mut occ: Vec<usize> = vec![0; 256 * bwt.len()];

    occ[bwt[0] as usize] = 1;

    for r in 1..bwt.len() {
        for i in 0..255 {
            occ[r * 256 + i] = occ[(r - 1) * 256 + i];
        }

        occ[r * 256 + (bwt[r] as usize)] += 1;
    }

    occ
}

/// Calculates the `less[a]` vector specifying for a letter `a` how many
/// letters in the BWT are smaller than this letter.
pub fn less(bwt: &[u8]) -> Vec<usize> {
    let mut less: Vec<usize> = vec![0; 256];

    for c in bwt.iter() {
        for i in (*c as usize) + 1..256 {
            less[i] += 1;
        }
    }

    less
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::full_text_indices::sais::fast;
    use crate::generate::{gen_rand_bytes, rand_pattern_from_bytes};

    #[test]
    fn test_occ() {
        let text = "abcbbc\u{0}".as_bytes();

        let occ = occ(text);

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

        assert_eq!(occ, occ_correct);
    }
}
