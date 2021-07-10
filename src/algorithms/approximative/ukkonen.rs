use std::cmp::{min, Ordering};
use std::mem::swap;

/// Returns occurrences of a pattern in a text given a maximum error.
///
/// The runtime is `O(kn)` given a maximum error `k` and a text with length `n`.
pub fn ukkonen(pattern: &[u8], text: &[u8], k: usize, _: &[u8]) -> Vec<(usize, usize)> {
    let m = pattern.len();
    let n = text.len();

    let mut occurrences: Vec<(usize, usize)> = Vec::new();
    #[allow(unused_assignments)]
    let mut d_o: Vec<usize> = Vec::with_capacity(m + 1);
    #[allow(unused_assignments)]
    let mut d_j: Vec<usize> = Vec::with_capacity(m + 1);
    let mut last_k = min(k, m);

    d_o = vec![k + 1; m + 1];
    d_j = (0..m + 1).collect();

    for j in 1..n + 1 {
        swap(&mut d_o, &mut d_j);
        d_j[0] = 0;
        last_k = min(last_k + 1, m);

        for i in 1..last_k + 1 {
            d_j[i] = *vec![
                d_o[i] + 1,
                d_j[i - 1] + 1,
                d_o[i - 1] + ukkonen_cost_one(&pattern[i - 1], &text[j - 1]),
            ]
            .iter()
            .min()
            .unwrap();
        }

        while d_j[last_k] > k {
            last_k -= 1;
        }

        if last_k == m {
            occurrences.push((j - m, d_j[m]));
        }
    }

    occurrences
}

fn ukkonen_cost_one(a: &u8, b: &u8) -> usize {
    match a.cmp(b) {
        Ordering::Equal => 0,
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ukkonen() {
        let text = b"dddddabcddd";
        let pattern = b"abc";
        let k = 1;

        let matches = ukkonen(pattern, text, k, &[]);

        println!("{:?}", matches);

        let matches_correct = vec![(4, 1), (5, 0), (6, 1)];

        assert_eq!(matches, matches_correct);
    }
}
