use std::cmp::Ordering;
use std::convert::TryFrom;

use bitvec::prelude::*;

/// Calculates the suffix array for a given text in `O(n)` runtime.
///
/// This function calculates the suffix array in linear runtime using the
/// suffix array induced sorting (SAIS) algorithm.
///
/// Make sure that the text contains a sentinel at the end which is a character
/// that is lexicographically smaller than any other character in the text.
pub fn fast(text: &[u8]) -> Vec<usize> {
    let types = types_vec(&text);
    let lms = lms_vec(&types);

    let bucket_ptrs: &mut [usize; 512] = &mut [0; 512];
    bucket_pointers(text, bucket_ptrs);

    let sorted_lms: Vec<usize> = sort(text, &types, &lms, bucket_ptrs)
        .iter()
        .filter(|x| **x != -1)
        .map(|x| *x as usize)
        .collect();

    let pos = sort(text, &types, &sorted_lms, bucket_ptrs);

    // Casting all as usize shouldn't fail here because there shouldn't be
    // any undefined values left at this point
    pos.iter().map(|x| *x as usize).collect()
}

fn types_vec(text: &[u8]) -> BitVec {
    let mut types: BitVec<LocalBits, usize> = bitvec![0; text.len()];

    // Sentinel is always S-type
    types.set(text.len() - 1, true);

    for i in (0..text.len() - 1).rev() {
        match text[i].cmp(&text[i + 1]) {
            // Push S-type
            Ordering::Less => types.set(i, true),

            // Push same type as before
            // Unwrap is safe here because there is at least the sentinel's
            // type in the types vector
            Ordering::Equal => {
                let t = *types.get(i + 1).unwrap();
                types.set(i, t);
            }

            // Pushing an L-type is not required as all positions are initialized
            // as L-type positions, so the according case `text[i] > text[i + 1]`
            // is just not implemented
            _ => (),
        }
    }

    types
}

fn lms_vec(types: &BitVec) -> Vec<usize> {
    let mut lms: Vec<usize> = Vec::new();

    for i in (0..types.len() - 1).rev() {
        // If the current position is L-type and the next one is S-type,
        // make that position an LMS position
        if !types[i] && types[i + 1] {
            lms.push(i + 1);
        }
    }

    // Reverse the vector because it is built from end to start, reversing
    // it sorts the indices contained by it in ascending order
    lms.iter().rev().copied().collect()
}

fn bucket_pointers(text: &[u8], buckets_pointers: &mut [usize; 512]) {
    let bucket_lengths: &mut [usize] = &mut [0; 256];

    for c in text.iter() {
        bucket_lengths[*c as usize] += 1;
    }

    let mut sum = 0;

    for i in 0..bucket_lengths.len() {
        if bucket_lengths[i] != 0 {
            buckets_pointers[i * 2] = sum;
            buckets_pointers[i * 2 + 1] = sum + bucket_lengths[i] - 1;
            sum += bucket_lengths[i];
        }
    }
}

fn sort(text: &[u8], types: &BitVec, lms: &[usize], bucket_ptrs_orig: &[usize; 512]) -> Vec<isize> {
    let mut buckets: Vec<isize> = vec![-1; text.len()];
    let mut bucket_ptrs = *bucket_ptrs_orig;

    for substring in lms.iter().rev() {
        let curr_char = text[*substring] as usize;

        buckets[bucket_ptrs[curr_char * 2 + 1]] = *substring as isize;
        if bucket_ptrs[curr_char * 2 + 1] != 0 {
            bucket_ptrs[curr_char * 2 + 1] -= 1;
        }
    }

    // Induce sort from left to right
    for r in 0..buckets.len() {
        if buckets[r] != -1 {
            let r_minus_1 = usize::try_from(buckets[r] - 1).unwrap_or(text.len() - 1);

            // If the character text[r - 1] is L-type
            if !types[r_minus_1] {
                let curr_char = text[r_minus_1] as usize;

                buckets[bucket_ptrs[curr_char * 2]] = r_minus_1 as isize;
                bucket_ptrs[curr_char * 2] += 1;
            }
        }
    }

    // Reset pointers
    bucket_ptrs = *bucket_ptrs_orig;

    // Induce sort from right to left
    for r in (0..buckets.len()).rev() {
        // This time there is no need to check for -1, i. e. if the current
        // value is undefined, because this never happens here

        let r_minus_1 = usize::try_from(buckets[r] - 1).unwrap_or(text.len() - 1);

        // If the character text[r - 1] is S-type
        if types[r_minus_1] {
            let curr_char = text[r_minus_1] as usize;

            buckets[bucket_ptrs[curr_char * 2 + 1]] = r_minus_1 as isize;
            if bucket_ptrs[curr_char * 2 + 1] != 0 {
                bucket_ptrs[curr_char * 2 + 1] -= 1;
            }
        }
    }

    buckets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sais() {
        let text = "gccttaacattattacgccta\u{0}".as_bytes();

        let pos_correct = vec![
            21, 20, 5, 6, 14, 11, 8, 7, 17, 1, 15, 18, 2, 16, 0, 19, 4, 13, 10, 3, 12, 9,
        ];

        assert_eq!(fast(text), pos_correct);
    }
}
