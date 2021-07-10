/// Maps each character of a string to a new character in the given alphabet to
/// reduce alphabet size to the size of the actual alphabet.
///
/// Panics if alphabet contains more than 256 characters or if `s` contains
/// characters that are not in `alphabet`.
pub fn map_alphabet(s: &[u8], alphabet: &[u8]) -> Vec<u8> {
    match alphabet.len() {
        len if len < 256 => s
            .iter()
            .map(|c| {
                alphabet
                    .iter()
                    .position(|x| x == c)
                    .expect("Text or pattern contains characters that are not in alphabet")
                    as u8
            })
            .collect::<Vec<u8>>(),
        len if len == 256 => s.to_vec(),
        _ => panic!("Alphabet contains too many characters"),
    }
}

pub fn unmap_alphabet(s: &[u8], alphabet: &[u8]) -> Vec<u8> {
    match alphabet.len() {
        len if len < 256 => s
            .iter()
            .map(|c| {
                *alphabet
                    .get(*c as usize)
                    .expect("Text or pattern contains characters that are not in alphabet")
            })
            .collect::<Vec<u8>>(),
        len if len == 256 => s.to_vec(),
        _ => panic!("Alphabet contains too many characters"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_alphabet() {
        let s = b"gccttaacattattacgccta";
        let alphabet = &['a' as u8, 'c' as u8, 'g' as u8, 't' as u8];

        let mapped = map_alphabet(s, alphabet);
        let mapped_correct = &[
            2, 1, 1, 3, 3, 0, 0, 1, 0, 3, 3, 0, 3, 3, 0, 1, 2, 1, 1, 3, 0,
        ];

        assert_eq!(mapped, mapped_correct);
    }

    #[test]
    fn test_unmap_alphabet() {
        let s = &[
            2, 1, 1, 3, 3, 0, 0, 1, 0, 3, 3, 0, 3, 3, 0, 1, 2, 1, 1, 3, 0,
        ];
        let alphabet = &['a' as u8, 'c' as u8, 'g' as u8, 't' as u8];

        let unmapped = unmap_alphabet(s, alphabet);
        let unmapped_correct = b"gccttaacattattacgccta";

        assert_eq!(unmapped, unmapped_correct);
    }
}
