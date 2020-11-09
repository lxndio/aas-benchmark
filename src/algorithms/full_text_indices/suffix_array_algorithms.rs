use crate::algorithms::full_text_indices::suffix_array::slow;

pub fn match_pattern(pos: Vec<usize>, pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut ls: Vec<usize> = (0..n)
        .filter(|r| less_eq_m(pattern, &text[pos[*r]..], m))
        .collect();
    ls.push(n);

    let mut rs: Vec<isize> = (0..n)
        .filter(|r| greater_eq_m(pattern, &text[pos[*r]..], m))
        .map(|x| x as isize)
        .collect();
    rs.push(-1);

    let l = *ls.iter().min().unwrap();
    let r = *rs.iter().max().unwrap();

    if r >= l as isize {
        (l..=r as usize).map(|x| pos[x]).collect()
    } else {
        Vec::new()
    }
}

// TODO only for testing purposes?
pub fn match_pattern_slow_pos(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    // Add sentinel to text
    let mut text = text.iter().map(|x| *x).collect::<Vec<u8>>();
    text.push(0);
    let text = text.as_slice();

    // Generate suffix array
    let pos = slow(text);

    // Run pattern matching
    match_pattern(pos, pattern, text)
}

fn less_eq_m(a: &[u8], b: &[u8], m: usize) -> bool {
    let mut m = m;
    if a.len().min(b.len()) < m {
        m = a.len().min(b.len());
    }

    a[0..m] <= b[0..m]
}

fn greater_eq_m(a: &[u8], b: &[u8], m: usize) -> bool {
    let mut m = m;
    if a.len().min(b.len()) < m {
        m = a.len().min(b.len());
    }

    a[0..m] >= b[0..m]
}
