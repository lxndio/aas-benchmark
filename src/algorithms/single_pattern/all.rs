pub fn all(
    pattern: &[u8],
    text: &[u8],
    algorithm: fn(&[u8], &[u8], usize) -> Option<usize>,
) -> Vec<usize> {
    let mut res = Vec::new();
    let mut i0 = 0;

    while let Some(occ) = algorithm(pattern, text, i0) {
        res.push(occ);

        i0 = occ + 1;
    }

    res
}
