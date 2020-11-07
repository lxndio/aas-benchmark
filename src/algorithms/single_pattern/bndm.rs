use std::collections::HashMap;

use crate::algorithms::single_pattern::shift_and::shift_and_single_masks;

/// An implementation of the Backward Nondeterministic DAWG Matching
/// algorithm (BNDM).
///
/// It uses the same function to generate shift masks as the Shift-And algorithm,
/// but reverses the pattern before passing it to the mentioned function.
///
/// After generating the shift masks, this function calls the actual
/// BNDM algorithm function.
pub fn bndm(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut pattern_rev = pattern.to_vec();
    pattern_rev.reverse();

    let (masks, _, accept) = shift_and_single_masks(&pattern_rev);

    bndm_with_masks(text, masks, accept, pattern.len())
}

/// An implementation of the Backward Nondeterminstic DAWG Matching
/// algorithm (BNDM) using already prepared shift masks.
fn bndm_with_masks(text: &[u8], masks: HashMap<u8, usize>, accept: usize, m: usize) -> Vec<usize> {
    let n = text.len();
    let mut window: usize = m;
    let mut active: usize;
    let mut j: usize;
    let mut lastsuffix: usize;

    let mut matches: Vec<usize> = Vec::new();

    while window <= n {
        active = (1 << m) - 1;
        j = 1;
        lastsuffix = 0;

        while active != 0 {
            // unwrap_or(&0) here, because the masks list should contain a 0
            // for every character that is not specifically set
            active &= masks.get(&text[window - j]).unwrap_or(&0);

            if active & accept != 0 {
                if j == m {
                    matches.push(window - m);
                    break;
                } else {
                    lastsuffix = j;
                }
            }

            j += 1;
            active <<= 1;
        }

        window += m - lastsuffix;
    }

    matches
}
