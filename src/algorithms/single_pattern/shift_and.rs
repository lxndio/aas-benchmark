use std::collections::HashMap;

fn shift_and_single_masks(pattern: &[u8]) -> (HashMap<u8, usize>, usize, usize) {
    let mut masks = HashMap::new();
    let mut bit = 1;

    for c in pattern {
        let entry = masks.entry(*c).or_insert(0);
        *entry |= bit;

        bit *= 2;
    }

    (masks, 1, bit / 2)
}

fn shift_and_with_masks(
    text: &[u8],
    masks: HashMap<u8, usize>,
    ones: usize,
    accept: usize,
) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let mut active: usize = 0;

    for (i, c) in text.iter().enumerate() {
        active = ((active << 1) | ones) & masks.get(c).unwrap_or(&0);

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

    for (i, _) in shift_and_with_masks(text, mask, ones, accept) {
        res.push(i - m + 1);
    }

    res
}
