use std::collections::HashMap;

/// The Backward Oracle Matching algorithm (BOM).
pub fn bom(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let delta = bom_delta_table(pattern);

    bom_with_delta(text, &delta, pattern.len())
}

fn bom_with_delta(text: &[u8], delta: &HashMap<(usize, u8), usize>, m: usize) -> Vec<usize> {
    let n = text.len();
    let mut window = m;

    let mut matches: Vec<usize> = Vec::new();

    while window <= n {
        let mut q = Some(&0);
        let mut j = 1;

        while j <= m && q.is_some() {
            q = delta.get(&(*q.unwrap(), text[window - j]));
            j += 1;
        }

        if q.is_some() {
            matches.push(window - m);
        }

        window += (m as isize - j as isize + 2) as usize;
    }

    matches
}

fn bom_delta_table(pattern: &[u8]) -> HashMap<(usize, u8), usize> {
    // Get the reverse pattern
    let mut pattern_rev = pattern.to_vec();
    pattern_rev.reverse();

    let m = pattern_rev.len();
    let mut delta: HashMap<(usize, u8), usize> = HashMap::new();
    let mut suff: Vec<Option<usize>> = vec![None; m + 1];

    for i in 1..m + 1 {
        let a = pattern_rev[i - 1];
        let mut k = suff[i - 1];
        delta.insert((i - 1, a), i);

        while k.is_some() && !delta.contains_key(&(k.unwrap(), a)) {
            delta.insert((k.unwrap(), a), i);
            k = suff[k.unwrap()];
        }

        suff[i] = Some(if let Some(k) = k { delta[&(k, a)] } else { 0 });
    }

    delta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bom() {
        let text = b"gccttaacattattacgccta";
        let pattern = b"tta";

        let mut matches = bom(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }
}
