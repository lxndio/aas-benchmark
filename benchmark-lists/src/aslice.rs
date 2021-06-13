#[derive(Debug)]
pub struct ASlice<'a, T: Clone + PartialEq> {
    slice: &'a [T],
    access_count: usize,
}

impl<'a, T: Clone + PartialEq> ASlice<'a, T> {
    #[cfg(feature = "accesscount")]
    pub fn slice(&mut self, range: std::ops::Range<usize>) -> &[T] {
        self.access_count += range.len();

        &self.slice[range]
    }

    #[cfg(not(feature = "accesscount"))]
    pub fn slice(&mut self, range: std::ops::Range<usize>) -> &[T] {
        &self.slice[range]
    }

    pub fn aslice(&'a mut self, range: std::ops::Range<usize>) -> ASlice<'a, T> {
        ASlice {
            slice: &self.slice[range],
            access_count: self.access_count,
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.slice.to_vec()
    }

    pub fn len(&self) -> usize {
        // TODO Should this add n to access_count?

        self.slice.len()
    }

    pub fn access_count(&self) -> usize {
        self.access_count
    }
}

impl<'a, T: Clone + PartialEq> From<&'a [T]> for ASlice<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        ASlice {
            slice,
            access_count: 0,
        }
    }
}

impl<'a, T: Clone + PartialEq> From<ASlice<'a, T>> for &'a [T] {
    fn from(aslice: ASlice<'a, T>) -> Self {
        aslice.slice
    }
}

impl<'a, T: Clone + PartialEq> PartialEq for ASlice<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.slice
            .iter()
            .fold(true, |acc, x| acc & other.slice.contains(x))
            && other
                .slice
                .iter()
                .fold(true, |acc, x| acc & self.slice.contains(x))
    }
}

impl<'a, T: Clone + PartialEq> Eq for ASlice<'a, T> {}

impl<'a, T: Clone + PartialEq> IntoIterator for ASlice<'a, T> {
    type Item = T;
    type IntoIter = std::slice::Iter<'a, Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.slice.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let slice: &[u8] = &[1, 2, 3, 4, 5];

        let aslice_correct: ASlice<u8> = ASlice {
            slice: &[1, 2, 3, 4, 5],
            access_count: 0,
        };

        assert_eq!(ASlice::from(slice), aslice_correct);
    }

    #[test]
    #[cfg(feature = "accesscount")]
    fn test_slice() {
        let slice: &[u8] = &[1, 2, 3, 4, 5];
        let mut aslice = ASlice::from(slice);
        let part: &[u8] = aslice.slice(0..3);

        let part_correct = &slice[0..3];

        assert_eq!(part, part_correct);
        assert_eq!(aslice.access_count(), 3);
    }
}
