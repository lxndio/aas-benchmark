use crate::algorithms::dfa::dfa_with_delta;

pub fn kmp_compute_lps(pattern: &[u8]) -> Vec<isize> {
    let m = pattern.len();

    let mut q: isize = -1;
    let mut lps: Vec<isize> = vec![0; m];

    for i in 1..m {
        while q > -1 && pattern[q as usize + 1] != pattern[i] {
            q = lps[q as usize] - 1; // as usize is safe because while condition is q > -1
        }

        if pattern[q as usize + 1] == pattern[i] {
            q += 1;
        }

        lps[i] = q + 1;
    }

    lps
}

pub fn dfa_delta_lps(q: isize, c: u8, pattern: &[u8], lps: Vec<isize>) -> isize {
    let m = pattern.len();

    let mut q = q;

    while q == m as isize - 1 || (pattern[q as usize + 1] != c && q > -1) {
        q = lps[q as usize] - 1;
    }

    if pattern[q as usize + 1] == c {
        q += 1;
    }

    q
}

pub fn kmp(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let lps = kmp_compute_lps(pattern);
    let delta = dfa_delta_lps;

    unimplemented!(); // TODO
                      //dfa_with_delta(pattern.len(), text, delta, i0);
}

pub fn kmp_classic(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut q: isize = -1;
    let mut lps = kmp_compute_lps(pattern);

    for i in i0..n {
        while q == m as isize - 1 || (pattern[q as usize + 1] != text[i] && q > -1) {
            q = lps[q as usize] - 1;
        }

        if pattern[q as usize + 1] == text[i] {
            q += 1;
        }

        if q == (m - 1) as isize {
            return Some(i + 1 - m);
        }
    }

    None
}

pub fn kmp_classic_all(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = kmp_classic(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}
