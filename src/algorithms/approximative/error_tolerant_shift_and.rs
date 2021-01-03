use crate::algorithms::single_pattern::shift_and::shift_and_single_masks;

pub fn error_tolerant_shift_and(pattern: &[u8], text: &[u8], k: usize) -> Vec<(usize, usize)> {
    let m = pattern.len();

    let mut occurrences: Vec<(usize, usize)> = Vec::new();
    let mut active: Vec<usize> = vec![0; k + 1];
    let (mask, ones, accept) = shift_and_single_masks(pattern);

    for (pos, c) in text.iter().enumerate() {
        for i in (1..=k).rev() {
            active[i] = (((active[i] << 1) | ones) & mask[*c as usize]
                | (2usize.pow(i as u32) - 1))
                | active[i - 1]
                | (active[i - 1] << 1);
        }

        active[0] = ((active[0] << 1) | ones) & mask[*c as usize];

        if active[0] & accept != 0 {
            occurrences.push((pos - m + 1, 0));
        }

        for i in 1..=k {
            active[i] |= active[i - 1] << 1;

            if active[i] & accept != 0 {
                occurrences.push((pos - m + 1, i));
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
        let text = b"adbcacbabcdacd";
        let pattern = b"abc";
        let k = 1;

        let matches = error_tolerant_shift_and(pattern, text, k);

        let matches_correct = vec![(3, 1), (5, 1), (8, 1), (9, 0), (10, 1), (12, 1)];

        assert_eq!(matches, matches_correct);
    }
}
