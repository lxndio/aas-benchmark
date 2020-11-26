use std::collections::HashMap;

fn horspool_shift(pattern: &[u8]) -> HashMap<u8, usize> {
    let mut shift = HashMap::new();
    let m = pattern.len();

    // Iterate over 0..m - 1
    for (j, c) in pattern.iter().enumerate().take(m - 1) {
        shift.insert(*c, m - 1 - j);
    }

    shift
}

pub fn horspool(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let shift = horspool_shift(pattern);
    let mut last = i0 + m - 1;
    let p_last = pattern[m - 1];

    loop {
        while last < n && text[last] != p_last {
            last += shift.get(&text[last]).unwrap_or(&m);
        }

        if last >= n {
            break;
        }

        if text[last - (m - 1)..last] == pattern[0..m - 1] {
            return Some(last - m + 1);
        }

        last += shift.get(&p_last).unwrap_or(&m);
    }

    None
}

pub fn horspool_all(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = horspool(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1; // TODO or `+ m`?
    }

    res
}
