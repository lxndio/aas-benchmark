use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::TryFrom;
use std::iter::FromIterator;

use bitvec::prelude::*;

/// Calculates the suffix array for a given text in `O(n)` runtime.
///
/// This function calculates the suffix array in linear runtime using the
/// suffix array induced sorting (SAIS) algorithm.
///
/// Make sure that the text contains a sentinel at the end which is a character
/// that is lexicographically smaller than any other character in the text.
pub fn fast(text: &[u8]) -> Vec<usize> {
    //let before = SystemTime::now();

    let types = types_vec(&text);
    //println!("Types: {}", before.elapsed().unwrap().as_millis());
    //let before = SystemTime::now();
    let lms = lms_vec(&types);

    //println!("LMS: {}", before.elapsed().unwrap().as_millis());
    //let before = SystemTime::now();

    let bucket_ptrs = bucket_pointers(text);

    //println!("Bucket pointers: {}", before.elapsed().unwrap().as_millis());
    //let before = SystemTime::now();

    let buckets = sort(text, &types, &lms, &bucket_ptrs);

    //println!("Buckets: {}", before.elapsed().unwrap().as_millis());
    //let before = SystemTime::now();

    // Use a set for faster contains check
    let lms_set: HashSet<isize> = HashSet::from_iter(lms.iter().map(|x| *x as isize));
    let mut sorted_lms: Vec<usize> = Vec::with_capacity(lms.len());

    for lms in buckets.iter() {
        if lms_set.contains(lms) {
            sorted_lms.push(*lms as usize);
        }
    }

    //println!("Sort LMS: {}", before.elapsed().unwrap().as_millis());
    //let before = SystemTime::now();

    let pos = sort(text, &types, &sorted_lms, &bucket_ptrs);

    //println!("Build pos: {}", before.elapsed().unwrap().as_millis());

    // Casting all as usize shouldn't fail here because there shouldn't be
    // any undefined values left at this point
    pos.iter().map(|x| *x as usize).collect()
}

fn types_vec(text: &[u8]) -> BitVec {
    let mut types: BitVec<LocalBits, usize> = bitvec![0; text.len()];

    // Sentinel is always S-type
    types.set(text.len() - 1, true);

    for i in (0..text.len() - 1).rev() {
        if text[i] > text[i + 1] {
            // Push L-type
            //types.push(false);
        } else if text[i] < text[i + 1] {
            // Push S-type
            types.set(i, true);
        } else {
            // Unwrap is safe here because there is at least the sentinel's
            // type in the types vector
            let t = *types.get(i + 1).unwrap();
            types.set(i, t);
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
    lms.iter().rev().map(|x| *x).collect()
}

fn bucket_pointers(text: &[u8]) -> HashMap<u8, (usize, usize)> {
    let bucket_lengths: BTreeMap<u8, usize> = text.iter().fold(BTreeMap::new(), |mut acc, c| {
        *acc.entry(*c).or_insert(0) += 1;
        acc
    });

    // TODO use HashMap here or BTreeMap as well? Is HashMap faster if order
    // isn't important?
    let mut bucket_pointers: HashMap<u8, (usize, usize)> = HashMap::new();
    let mut sum = 0;

    for (k, v) in bucket_lengths.iter() {
        bucket_pointers.insert(*k, (sum, sum + v - 1));
        sum += v;
    }

    bucket_pointers
}

fn sort(
    text: &[u8],
    types: &BitVec,
    lms: &Vec<usize>,
    bucket_ptrs_orig: &HashMap<u8, (usize, usize)>,
) -> Vec<isize> {
    let mut buckets: Vec<isize> = vec![-1; text.len()];
    let mut bucket_ptrs = bucket_ptrs_orig.clone();

    for substring in lms.iter().rev() {
        let curr_char = &text[*substring];

        buckets[bucket_ptrs[curr_char].1] = *substring as isize;
        bucket_ptrs.insert(
            *curr_char,
            (
                bucket_ptrs[curr_char].0,
                if bucket_ptrs[curr_char].1 != 0 {
                    bucket_ptrs[curr_char].1 - 1
                } else {
                    bucket_ptrs[curr_char].1
                },
            ),
        );
    }

    // Induce sort from left to right
    for r in 0..buckets.len() {
        if buckets[r] != -1 {
            let r_minus_1 = usize::try_from(buckets[r] - 1).unwrap_or(text.len() - 1);

            // If the character text[r - 1] is L-type
            if !types[r_minus_1] {
                buckets[bucket_ptrs[&text[r_minus_1]].0] = r_minus_1 as isize;
                bucket_ptrs.insert(
                    text[r_minus_1],
                    (
                        bucket_ptrs[&text[r_minus_1]].0 + 1,
                        bucket_ptrs[&text[r_minus_1]].1,
                    ),
                );
            }
        }
    }

    // Reset pointers
    bucket_ptrs = bucket_ptrs_orig.clone();

    // Induce sort from right to left
    for r in (0..buckets.len()).rev() {
        // This time there is no need to check for -1, i. e. if the current
        // value is undefined, because this never happens here

        let r_minus_1 = usize::try_from(buckets[r] - 1).unwrap_or(text.len() - 1);

        // If the character text[r - 1] is S-type
        if types[r_minus_1] {
            buckets[bucket_ptrs[&text[r_minus_1]].1] = r_minus_1 as isize;
            bucket_ptrs.insert(
                text[r_minus_1],
                (
                    bucket_ptrs[&text[r_minus_1]].0,
                    if bucket_ptrs[&text[r_minus_1]].1 != 0 {
                        bucket_ptrs[&text[r_minus_1]].1 - 1
                    } else {
                        bucket_ptrs[&text[r_minus_1]].1
                    },
                ),
            );
        }
    }

    buckets
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::full_text_indices::suffix_array::slow;
    use crate::generate::gen_rand_bytes;

    #[test]
    fn time() {
        let text = gen_rand_bytes(1_000_000, None);
        let text = text.as_slice();

        fast(text);
    }

    #[test]
    fn fixed_text() {
        let mut text = "gccttaacattattacgccta"
            .as_bytes()
            .iter()
            .map(|x| *x)
            .collect::<Vec<u8>>();
        text.push(0);
        let text = text.as_slice();

        assert_eq!(fast(text), slow(text));
    }

    #[test]
    fn random_texts() {
        for i in 0..10 {
            println!("Test {}", i);

            let mut text = gen_rand_bytes(100000, None);
            text.push(0);
            let text = text.as_slice();

            assert_eq!(fast(text), slow(text));
        }
    }
}
