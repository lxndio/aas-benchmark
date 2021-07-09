use std::cmp::min;

/// Calcualtes the table of suffixes of string x with length m
/// According to "Algorithms on Strings, Chapter 3.3"
pub fn suffixes(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();

    // return empty list if pattern is empty
    if m == 0 {
        return Vec::new();
    }

    let mut suff = vec![0; m];
    let mut f = 0;
    let mut g = (m - 1) as isize;
    suff[m - 1] = m;

    for i in (0..m - 1).rev() {
        if i > g as usize && suff[i + m - 1 - f] != i - g as usize {
            suff[i] = min(suff[i + m - 1 - f], i - g as usize);
        } else {
            g = min(g, i as isize);
            f = i;

            while g >= 0 && pattern[g as usize] == pattern[g as usize + m - 1 - f] {
                g -= 1;
            }

            suff[i] = (f as isize - g) as usize;
        }
    }

    suff
}

pub fn good_suffixes(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();

    // Return empty list if pattern is empty
    if m == 0 {
        return Vec::new();
    }

    let mut good_suff = vec![0; m];
    let suff = suffixes(pattern);
    let mut j = 0;

    for i in (-1..m as isize - 1).rev() {
        if i == -1 || suff[i as usize] == (i + 1) as usize {
            while j < (m as isize - 1 - i) as usize {
                good_suff[j] = (m as isize - 1 - i) as usize;
                j = j + 1;
            }
        }
    }

    for i in 0..m - 1 {
        good_suff[m - 1 - suff[i]] = m - 1 - i;
    }

    good_suff
}

/// Calculated the smallest period of string
pub fn per(pattern: &[u8]) -> usize {
    if pattern.len() == 0 {
        panic!("Can't calculate period of an empty string");
    }

    let period = good_suffixes(pattern)[0];

    if period > 0 {
        period
    } else {
        pattern.len()
    }
}

/// modified w_memoryless_suffix_search form "Algorithms on Strings, Chapter 3"
pub fn weak_boyer_moore(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    let good_suff = good_suffixes(pattern);
    let mut j = i0 + m - 1;

    while j < n {
        let mut i = (m - 1) as isize;

        while i >= 0 && pattern[i as usize] == text[j + 1 + i as usize - m] {
            i -= 1;
        }

        if i < 0 {
            return Some(j + 1 - pattern.len());
        }

        if i < 0 {
            j = j + per(pattern);
        } else {
            j = j + good_suff[i as usize];
        }
    }

    None
}

/// modified w_memoryless_suffix_search form "Algorithms on Strings, Chapter 3"
pub fn weak_boyer_moore_all(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = weak_boyer_moore(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffixes_book_example() {
        // Example from Algorithms on Strings chapter 3.3

        let pattern = b"aaacababa";
        let suff = suffixes(pattern);

        assert_eq!(suff.len(), pattern.len());
        assert_eq!(suff, vec![1, 1, 1, 0, 1, 0, 3, 0, 9]);
    }

    #[test]
    fn test_suffixes_empty_string() {
        let pattern = b"";
        let suff = suffixes(pattern);

        assert_eq!(suff.len(), pattern.len());
        assert_eq!(suff, vec![]);
    }

    #[test]
    fn test_suffixes_two_chars() {
        let pattern = b"aa";
        let suff = suffixes(pattern);

        assert_eq!(suff.len(), pattern.len());
        assert_eq!(suff, vec![1, 2]);
    }

    #[test]
    fn test_suffixes_two_different_chars() {
        let pattern = b"ab";
        let suff = suffixes(pattern);

        assert_eq!(suff.len(), pattern.len());
        assert_eq!(suff, vec![0, 2]);
    }

    #[test]
    fn test_good_suffixes_book_example() {
        let pattern = b"aaacababa";
        let good_suff = good_suffixes(pattern);

        assert_eq!(good_suff.len(), pattern.len());
        assert_eq!(good_suff, vec![8, 8, 8, 8, 8, 2, 8, 4, 1]);
    }

    #[test]
    fn test_good_suffixes_empty_string() {
        let pattern = b"";
        let good_suff = good_suffixes(pattern);

        assert_eq!(good_suff.len(), pattern.len());
        assert_eq!(good_suff, vec![]);
    }

    #[test]
    fn test_period_example_book() {
        // Example from book "Algorithms on Strings" Chapter 1.2
        let pattern = b"aabaabaa";
        assert_eq!(per(pattern), 3);
    }

    #[test]
    fn test_period_example_book_35() {
        // Example from book "Algorithms on Strings" Chapter 3.5
        let pattern = b"ababababab";
        assert_eq!(per(pattern), 2);
    }

    #[test]
    fn test_period_simple() {
        let pattern = b"abab";
        assert_eq!(per(pattern), 2);
    }

    #[test]
    fn test_no_period() {
        let pattern = b"abcdef";
        assert_eq!(per(pattern), pattern.len());
    }

    #[test]
    fn test_period_long_with_short_period() {
        let pattern = b"abcabcabcabc";
        assert_eq!(per(pattern), 3);
    }

    #[test]
    fn test_weak_bm_good_suff_book_example() {
        // Example from Algorithms on Strings chapter 3.5 Figure 3.14 (b)
        let text = b"aaacabaaacabacaaacababa";
        let pattern = b"aaacababa";
        let matches = weak_boyer_moore_all(pattern, text);

        assert_eq!(matches, vec![14]);
    }

    #[test]
    fn test_weak_bm_good_suff_simple() {
        let text = b"abcaaabcabcabc";
        let pattern = b"abc";
        let matches = weak_boyer_moore_all(pattern, text);

        assert_eq!(matches, vec![0, 5, 8, 11]);
    }
}
