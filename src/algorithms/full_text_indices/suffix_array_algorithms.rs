use crate::algorithms::full_text_indices::suffix_array::slow;

pub fn match_pattern(pos: Vec<usize>, text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut ls: Vec<usize> = (0..pos.len())
        .filter(|r| greater_m(pattern, &text[pos[*r]..], m))
        .collect();
    ls.push(n);

    let mut rs: Vec<isize> = (0..pos.len())
        .filter(|r| less_m(pattern, &text[pos[*r]..], m))
        .map(|x| x as isize)
        .collect();
    rs.push(-1);

    let l = *ls.iter().min().unwrap();
    let r = *ls.iter().min().unwrap();

    if r >= l {
        (l..=r as usize).map(|x| pos[x]).collect()
    } else {
        Vec::new()
    }
}

// TODO only for testing purposes?
pub fn match_pattern_slow_pos(text: &[u8], pattern: &[u8]) -> Vec<usize> {
    let pos = slow(text);

    match_pattern(pos, text, pattern)
}

fn less_m(a: &[u8], b: &[u8], m: usize) -> bool {
    a[0..m] < b[0..m]
}

fn greater_m(a: &[u8], b: &[u8], m: usize) -> bool {
    a[0..m] > b[0..m]
}
