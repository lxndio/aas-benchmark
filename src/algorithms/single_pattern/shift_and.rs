pub fn shift_and_single_masks(pattern: &[u8]) -> (Vec<usize>, usize, usize) {
    let mut masks = vec![0; 256];
    let mut bit = 1;

    for c in pattern {
        masks[*c as usize] |= bit;

        bit *= 2;
    }

    (masks, 1, bit / 2)
}

fn shift_and_with_masks(
    text: &[u8],
    masks: &[usize],
    ones: usize,
    accept: usize,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut active: usize = 0;

    for (i, c) in text.iter().enumerate() {
        active = ((active << 1) | ones) & masks[*c as usize];

        let found = active & accept;
        if found != 0 {
            res.push((i, found));
        }
    }

    res
}

pub fn shift_and(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let m = pattern.len();
    let (mask, ones, accept) = shift_and_single_masks(pattern);

    for (i, _) in shift_and_with_masks(text, &mask, ones, accept) {
        res.push(i - m + 1);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_and() {
        let text = b"gccttaacattattacgccta";
        let pattern = b"tta";

        let mut matches = shift_and(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
