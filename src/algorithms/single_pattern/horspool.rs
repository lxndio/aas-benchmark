pub fn horspool_shift(pattern: &[u8], alphabet_len: usize) -> Vec<usize> {
    let mut shift = vec![pattern.len(); alphabet_len];
    let m = pattern.len();

    // Iterate over 0..m - 1
    for (j, c) in pattern.iter().enumerate().take(m - 1) {
        shift[*c as usize] = m - 1 - j;
    }

    shift
}

pub fn horspool(pattern: &[u8], text: &[u8], i0: usize, alphabet_len: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let shift = horspool_shift(pattern, alphabet_len);
    let mut last = i0 + m - 1;
    let p_last = pattern[m - 1];

    loop {
        while last < n && text[last] != p_last {
            last += shift[text[last] as usize];
        }

        if last >= n {
            break;
        }

        if text[last - (m - 1)..last] == pattern[0..m - 1] {
            return Some(last - m + 1);
        }

        last += shift[p_last as usize];
    }

    None
}

pub fn horspool_all(pattern: &[u8], text: &[u8], alphabet: &[u8]) -> Vec<usize> {
    let alphabet_len = alphabet.len();

    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = horspool(pattern, text, i0, alphabet_len) {
        res.push(occ);

        i0 = occ + 1; // TODO or `+ m`?
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horspool_all() {
        let text = b"gccttaacattattacgccta";
        let pattern = b"tta";
        let alphabet = &['a' as u8, 'c' as u8, 'g' as u8, 't' as u8];

        let mut matches = horspool_all(pattern, text, alphabet);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
