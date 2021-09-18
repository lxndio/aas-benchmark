use std::cmp::min;

const WORD_SIZE: usize = usize::BITS as usize;

fn compute_mask_matrix(pattern: &[u8]) -> Vec<Vec<usize>> {
    let m = pattern.len();
    let mut mask = vec![vec![usize::MAX; WORD_SIZE + m - 1]; 256];

    for i in 0..WORD_SIZE {
        let tmp = 1 << i;

        for (j, c) in pattern.iter().enumerate() {
            for a in 0..256 {
                mask[a][i + j] &= !tmp;
            }

            mask[*c as usize][i + j] |= tmp;
        }
    }

    mask
}

fn compute_shift_vector(pattern: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let mut shift = vec![WORD_SIZE + m; 256];

    for (j, c) in pattern.iter().enumerate() {
        shift[*c as usize] = WORD_SIZE + m - 1 - j;
    }

    shift
}

fn compute_scan_order(m: usize, ws: usize) -> Vec<usize> {
    let mut scan_order = vec![0; ws];
    let mut i = 0;

    for mut j in (0..m).rev() {
        while j < ws {
            scan_order[i] = j;
            j += m;
            i += 1;
        }
    }

    scan_order
}

pub fn blim(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();
    let mask = compute_mask_matrix(pattern);
    let shift = compute_shift_vector(pattern);
    let mut scan_order = compute_scan_order(m, min(WORD_SIZE + m - 1, n));

    let mut res = Vec::new();
    let mut i = 0;
    let mut ws = min(WORD_SIZE + m - 1, n);
    let mut flag;

    while i < n {
        flag = mask[text[i + scan_order[0]] as usize][scan_order[0]];

        for j in 1..ws {
            flag &= mask[text[i + scan_order[j]] as usize][scan_order[j]];

            if flag == 0 {
                break;
            }
        }

        if flag != 0 {
            for j in 0..min(WORD_SIZE, n - i) {
                if flag & (1 << j) != 0 {
                    res.push(i + j);
                }
            }
        }

        if i + ws < n {
            i += shift[text[i + ws] as usize];
        } else {
            break;
        }

        // Check if the new search window still can have size ws,
        // otherwise make it smaller
        if n - i < WORD_SIZE + m - 1 {
            ws = n - i;
            scan_order = compute_scan_order(m, ws);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_shift_vector() {
        let pattern = b"abaab";

        let shift = compute_shift_vector(pattern);

        let mut shift_correct = vec![WORD_SIZE + pattern.len() - 1 + 1; 256];
        shift_correct['a' as usize] = WORD_SIZE + pattern.len() - 1 - 3;
        shift_correct['b' as usize] = WORD_SIZE + pattern.len() - 1 - 4;

        assert_eq!(shift, shift_correct);
    }

    #[test]
    fn test_blim() {
        let text = b"gccttaacattattacgcctagccttaacattattacgcctagccttaacattattacgcctagcccgaatta";
        let pattern = b"tta";

        let mut matches = blim(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![3, 9, 12, 24, 30, 33, 45, 51, 54, 70];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_blim_edge_case_end() {
        let text =
            b"gccttaacattatcattattacgcctagccttaacattatttacgcctagccttaacattattacgcctagctcctcga";
        let pattern = b"gctcctcga";

        let mut matches = blim(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![70];

        assert_eq!(matches, matches_correct);
    }

    #[test]
    fn test_blim_edge_short_text() {
        let text = b"ggctcctcgaatcattattacgccgctcctcgaaa";
        let pattern = b"gctcctcga";

        let mut matches = blim(pattern, text);
        matches.sort_unstable();

        let matches_correct = vec![1, 24];

        assert_eq!(matches, matches_correct);
    }
}
