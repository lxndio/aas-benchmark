/// Calculates the suffix array for a given text in `O(n)` runtime.
///
/// This function calculates the suffix array in linear runtime using the
/// suffix array induced sorting (SAIS) algorithm.
pub fn fast(text: &[u8]) {
    // Add sentinel to text
    let mut text = text.iter().map(|x| *x).collect::<Vec<u8>>();
    text.push(0);
    let text = text.as_slice();

    // Generate types vector
    let types = types_vec(&text);
    let lms = lms_vec(&types);

    println!(
        "{}\n{}\n{:?}",
        String::from_utf8(
            text.iter()
                .map(|c| if c == &0 { '$' as u8 } else { *c })
                .collect::<Vec<u8>>()
                .to_vec()
        )
        .unwrap(),
        types
            .iter()
            .map(|x| if *x { 'S' } else { 'L' })
            .collect::<String>(),
        lms
    );
}

fn types_vec(text: &[u8]) -> BitVec {
    let mut types: BitVec<LocalBits, usize> = BitVec::with_capacity(text.len());

    // Sentinel is always S-type
    types.push(true);

    for i in (0..text.len() - 1).rev() {
        if text[i] > text[i + 1] {
            // Push L-type
            types.push(false);
        } else if text[i] < text[i + 1] {
            // Push S-type
            types.push(true);
        } else {
            // Unwrap is safe here because there is at least the sentinel's
            // type in the types vector
            types.push(*types.last().unwrap());
        }
    }

    // Reverse the vector because it is built from end to start,
    // reversing it makes its indices correspond with the text
    types.iter().rev().collect()
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