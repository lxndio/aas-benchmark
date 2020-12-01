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

    bndm_with_masks(text, &masks, accept, pattern.len())
}

/// An implementation of the Backward Nondeterminstic DAWG Matching
/// algorithm (BNDM) using already prepared shift masks.
fn bndm_with_masks(text: &[u8], masks: &[usize], accept: usize, m: usize) -> Vec<usize> {
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
            active &= masks[text[window - j] as usize];

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bndm() {
        let text = "gccttaacattattacgccta".as_bytes();
        let pattern = "tta".as_bytes();

        let mut matches = bndm(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
