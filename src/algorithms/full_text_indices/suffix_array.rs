use bitvec::prelude::*;

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

/// Calculates the suffix array for a given text in `O(n)` runtime.
///
/// This function calculates the suffix array in linear runtime using the
/// suffix array induced sorting (SAIS) algorithm.
pub fn fast(text: &[u8]) {
    // Add sentinel to text
    let mut text = text.iter().map(|x| *x).collect::<Vec<u8>>();
    text.push(0);
    let text = text.as_slice();

    // Generate types and LMS vector
    let (types, lms) = types_lms_vec(&text);

    println!(
        "{}\n{}\n{}",
        String::from_utf8(text.to_vec()).unwrap(),
        types
            .iter()
            .map(|x| if *x { 'S' } else { 'L' })
            .collect::<String>(),
        lms.iter()
            .map(|x| if *x { '*' } else { ' ' })
            .collect::<String>(),
    );


}

fn types_lms_vec(text: &[u8]) -> (BitVec, BitVec) {
    let mut types = BitVec::<LocalBits, usize>::with_capacity(text.len());
    let mut lms = BitVec::<LocalBits, usize>::with_capacity(text.len());

    // Sentinel is always S-type
    types.push(true);

    for i in (0..text.len() - 2).rev() {
        if text[i] > text[i + 1] {
            // Push L-type
            types.push(false);
            lms.push(true);
        } else if text[i] < text[i + 1] {
            // Push S-type
            types.push(true);
            lms.push(false);
        } else {
            // Unwrap is safe here because there is at least the sentinel's
            // type in the types vector
            types.push(*types.last().unwrap());
            lms.push(false);
        }
    }

    // Add last LMS position because LMS vector is always
    // one bit behind the types vector
    // TODO is this correct?
    if *types.last().unwrap() {
        lms.push(true);
    } else {
        lms.push(false);
    }

    // Reverse both vectors because they are built from end
    // to start, reversing them makes their indices correspond
    // with the text
    (types.iter().rev().collect(), lms.iter().rev().collect())
}
