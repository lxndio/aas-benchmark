use crate::algorithms::single_pattern::horspool::horspool_shift;

// TODO Couldn't this table be implemented on the fly?
fn d2_table(pattern: &[u8], shift: &[usize]) -> Vec<Vec<usize>> {
    let mut d2 = vec![vec![0; 256]; 256];
    let m = pattern.len();

    for i in 0..=255 {
        for j in 0..=255 {
            if !pattern.contains(&i) && !pattern.contains(&j) {
                d2[i as usize][j as usize] = 2 * m;
            } else if !pattern.contains(&i) && pattern.contains(&j) {
                d2[i as usize][j as usize] = m + (shift[j as usize] % m);
            } else if pattern.contains(&i) && pattern[m - 1] != i {
                d2[i as usize][j as usize] = shift[i as usize] % m;
            }
        }
    }

    d2
}

pub fn double_window(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();

    let mut res = Vec::new();
    let shift = horspool_shift(pattern);
    let d2 = d2_table(pattern, &shift);
    let m1 = m - 1;
    let mut pos = m - 1;

    while pos < n - m {
        let r = d2[text[pos] as usize][text[pos + m] as usize];

        if r == 0 {
            let mut j = 0;

            while j < m && text[pos - (m - 1) + j] == pattern[j] {
                j += 1;
            }

            if j == m {
                res.push(pos - (m - 1));
            }

            pos += shift[text[pos] as usize];
        } else {
            pos += r;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_window() {
        let text = b"gccttaacattattacgccta";
        let pattern = b"tta";

        let mut matches = double_window(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_double_window2() {
        let text = b"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let pattern = b"ipsum";

        let mut matches = double_window(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![6, 274, 302, 570];

        assert_eq!(matches, matches_correct);
    }
}
