use crate::algorithms::single_pattern::shift_and::shift_and_single_masks;

/// Returns occurrences of a pattern in a text given a maximum error.
///
/// This is a modified version of the Shift-And Algorithm, simulating multiple
/// NFAs to account for the pattern being able to have a given edit distance
/// from the patterns in the text (the error).
pub fn error_tolerant_shift_and(pattern: &[u8], text: &[u8], k: usize) -> Vec<(usize, usize)> {
    let m = pattern.len();

    let mut occurrences: Vec<(usize, usize)> = Vec::new();
    let mut active: Vec<usize> = vec![0; k + 1];
    let (mask, ones, accept) = shift_and_single_masks(pattern);

    for (pos, c) in text.iter().enumerate() {
        // For each character in the text, we have to simulate multiple NFAs
        // by first applying the active state function first on all states
        // descending, and then on all states ascending (including state 0)
        for i in (1..=k).rev() {
            active[i] = (((active[i] << 1) | ones) & mask[*c as usize]
                | (2usize.pow(i as u32) - 1))
                | active[i - 1]
                | (active[i - 1] << 1);
        }

        active[0] = ((active[0] << 1) | ones) & mask[*c as usize];

        // The occurrence_added variable is used to prevent a position to be
        // added multiple times with different errors; because the error values
        // are iterated ascendingly from here on, only the lowest error for each
        // position will be added to the occurrences
        let mut occurrence_added = false;
        if active[0] & accept != 0 {
            occurrences.push((pos - m + 1, 0));
            occurrence_added = true;
        }

        for i in 1..=k {
            active[i] |= active[i - 1] << 1;

            if active[i] & accept != 0 && !occurrence_added {
                occurrences.push((pos - m + 1, i));
                occurrence_added = true;
            }
        }
    }

    occurrences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_tolerant_shift_and() {
        let text = b"dddddabcddd";
        let pattern = b"abc";
        let k = 1;

        let matches = error_tolerant_shift_and(pattern, text, k);

        println!("{:?}", matches);

        let matches_correct = vec![(4, 1), (5, 0), (6, 1)];

        assert_eq!(matches, matches_correct);
    }
}
