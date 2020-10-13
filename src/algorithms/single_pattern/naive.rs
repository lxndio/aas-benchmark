/// The naive algorithm approach uses a simple loop to look for an occurrence
/// of a `pattern` in a `text`.
///
/// It does this by iterating over each index `i` of the text's characters
/// starting at index `i0` and comparing the following `m` characters with
/// the pattern, `m` being the length of the pattern.
///
/// After an occurrence has been found, the algorithm returns the index
/// marking the first character of the occurrence and therefore terminates.
/// If the pattern could not be found in the `text`, `None` is returned.
pub fn naive(pattern: &[u8], text: &[u8], i0: usize) -> Option<usize> {
    let m = pattern.len();
    let n = text.len();

    for i in i0..(n - m + 1) {
        if &text[i..i + m] == pattern {
            return Some(i);
        }
    }

    None
}
