use crate::algorithms::single_pattern::horspool::horspool_shift;

fn d_table(pattern: &[u8]) -> Vec<usize> {
    let mut d = vec![pattern.len(); 256];
    let m = pattern.len();

    // Iterate over 0..m
    for (j, c) in pattern.iter().enumerate() {
        d[*c as usize] = m - 1 - j;
    }

    d
}

fn d2_table(pattern: &[u8], d: &[usize]) -> Vec<Vec<usize>> {
    let mut d2 = vec![vec![0; 256]; 256];
    let m = pattern.len();

    for i in 0..=255 {
        for j in 0..=255 {
            if !pattern.contains(&i) && !pattern.contains(&j) {
                d2[i as usize][j as usize] = 2 * m;
            } else if !pattern.contains(&i) && pattern.contains(&j) {
                d2[i as usize][j as usize] = m + d[j as usize];
            } else if pattern.contains(&i) && pattern[m - 1] != i {
                d2[i as usize][j as usize] = d[i as usize];
            }
        }
    }

    d2
}

pub fn double_window(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();
    let d = d_table(pattern);
    let d2 = d2_table(pattern, &d);
    let shift = horspool_shift(pattern);

    let mut res = Vec::new();
    let mut pos = m - 1;

    while pos < n {
        let r = d2[text[pos] as usize][*text.get(pos + m).unwrap_or(&pattern[0]) as usize];

        if r == 0 {
            if &text[pos - (m - 1)..pos + 1] == pattern {
                res.push(pos - (m - 1));
            }

            pos += shift[text[pos] as usize];
        } else {
            pos += r;
        }
    }

    res
}

fn d2_table_alt(pattern: &[u8], shift: &[usize], d: &[usize]) -> Vec<Vec<usize>> {
    let mut d2 = vec![vec![0; 256]; 256];
    let m = pattern.len();

    for i in 0..=255 {
        for j in 0..=255 {
            if !pattern.contains(&i) && !pattern.contains(&j) {
                d2[i as usize][j as usize] = 2 * m;
            } else if !pattern.contains(&i) && pattern.contains(&j) {
                d2[i as usize][j as usize] = m + d[j as usize];
            } else if pattern.contains(&i) {
                d2[i as usize][j as usize] = shift[i as usize];
            }
        }
    }

    d2
}

pub fn double_window_alt(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();
    let shift = horspool_shift(pattern);
    let d = d_table(pattern);
    let d2 = d2_table_alt(pattern, &shift, &d);

    let mut res = Vec::new();
    let mut pos = m - 1;

    while pos < n {
        let r = d2[text[pos] as usize][*text.get(pos + m).unwrap_or(&pattern[0]) as usize];

        if pattern[m - 1] == text[pos] && &text[pos - (m - 1)..pos + 1] == pattern {
            res.push(pos - (m - 1));
        }

        pos += r;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_window1() {
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

    #[test]
    fn test_double_window3() {
        let text = b"abbeabddaaaaaaaaaaaa";
        let pattern = b"abdd";

        let mut matches = double_window(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![4];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_double_window_edge_case_end() {
        let text = b"gccttaacattattacgcctagccttaacattattacgcctagctcctcga";
        let pattern = b"gctcctcga";

        let mut matches = double_window(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![42];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_double_window_alt2() {
        let text = b"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
        let pattern = b"ipsum";

        let mut matches = double_window_alt(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![6, 274, 302, 570];

        assert_eq!(matches, matches_correct);
    }
}
