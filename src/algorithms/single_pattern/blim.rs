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

fn example_mask(pattern: &[u8]) -> Vec<Vec<usize>> {
    let m = pattern.len();
    let mut mask = vec![vec![0b1111_1111; 8 + m - 1]; 4];

    for i in 0..8 {
        let tmp = 1 << i;

        for (j, c) in pattern.iter().enumerate() {
            for a in 0..4 {
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

// fn compute_scan_order(m: usize) -> Vec<usize> {
//     let mut scan_order = vec![0; WORD_SIZE + m - 1];
//     let mut i = 0;
//     let mut k;

//     for j in (0..m).rev() {
//         k = j;

//         while k < WORD_SIZE + m - 1 {
//             scan_order[i] = k;
//             k += m;
//             i += 1;
//         }
//     }

//     scan_order
// }

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

fn compute_scan_order_alex(m: usize) -> Vec<usize> {
    let mut scan_order = Vec::new();
    let mut k;

    for j in (0..m).rev() {
        k = j;

        while k < WORD_SIZE + m - 1 {
            scan_order.push(k);
            k += m;
        }
    }

    scan_order
}

pub fn blim(pattern: &[u8], text: &[u8]) -> Vec<usize> {
    let m = pattern.len();
    let n = text.len();
    let mask = compute_mask_matrix(pattern);
    let shift = compute_shift_vector(pattern);
    let mut scan_order = compute_scan_order(m, WORD_SIZE + m - 1);

    let mut res = Vec::new();
    let mut i = 0;
    let mut ws = WORD_SIZE + m - 1;
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
            for j in 0..std::cmp::min(WORD_SIZE, n - i) {
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

fn compute_mask_matrix_alex(pattern: &[u8]) -> Vec<Vec<usize>> {
    let m = pattern.len();
    let mut mask = vec![vec![usize::MAX; WORD_SIZE + m - 1]; 256];

    for a in 0..256 {
        // TODO make this 0..=255 and remove as u8 below
        for pos in 0..WORD_SIZE + m - 1 {
            for i in 0..WORD_SIZE {
                // Setting b_i of mask[a][pos]
                if pos >= i && pos - i < m && a as u8 != pattern[pos - i] {
                    mask[a][pos] &= !(1 << i);
                }
            }
        }
    }

    mask
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_mask() {
        let pattern = &[3, 2, 2, 1, 0]; // gccta

        let mask = example_mask(pattern);

        println!("  | 0  1  2  3  4  5  6  7  8  9  10 11");
        println!("--+------------------------------------");
        for (i, row) in mask.iter().enumerate() {
            print!("{:} | ", i);
            for entry in row {
                print!("{:X} ", entry);
            }
            println!();
        }
    }

    // #[test]
    // fn test_compute_mask_matrix() {
    //     let pattern = b"abaab";

    //     let mask = compute_mask_matrix(pattern);
    //     let mask_correct = vec![
    //         vec![0xFF, 0xFE, 0xFD, 0xFB, 0xF6, 0xED, 0xDB, 0xB7, 0x6F, 0xDF, 0xBF, 0x7F],
    //         vec![0xFE, 0xFD, 0xFA, 0xF4, 0xE9, 0xD3, 0xA7, 0x4F, 0x9F, 0x3F, 0x7F, 0xFF],
    //         vec![0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC1, 0x83, 0x07, 0x0F, 0x1F, 0x3F, 0x7F],
    //         vec![0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC1, 0x83, 0x07, 0x0F, 0x1F, 0x3F, 0x7F],
    //     ];

    //     assert_eq!(mask, mask_correct);
    // }

    // #[test]
    // fn test_compute_scan_order() {
    //     let m = 5;

    //     let scan_order = compute_scan_order(m);
    //     let scan_order_alex = compute_scan_order_alex(m);

    //     println!("{:?}", scan_order);

    //     assert_eq!(scan_order_alex, scan_order);
    // }

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

    // #[test]
    // fn test_compute_mask_matrix_alex() {
    //     let pattern = &[0, 1, 0, 0, 1];

    //     let mask = compute_mask_matrix(pattern);
    //     let mask_alex = compute_mask_matrix_alex(pattern);

    //     assert_eq!(mask_alex, mask);

    //     println!("  | 0  1  2  3  4  5  6  7  8  9  10 11");
    //     println!("--+------------------------------------");
    //     for (i, row) in mask.iter().enumerate() {
    //         print!("{:} | ", i);
    //         for entry in row {
    //             print!("{:X} ", entry);
    //         }
    //         println!();
    //     }
    // }
}
