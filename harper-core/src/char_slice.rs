pub struct CharSlice<'a>(&'a [char]);

impl<'a> CharSlice<'a> {
    pub fn new(slice: &'a [char]) -> Self {
        CharSlice(slice)
    }

    /// Get the underlying character slice.
    pub fn as_slice(&self) -> &'a [char] {
        self.0
    }
}

/// Case-insensitive comparison with a character slice, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl PartialEq<&[char]> for CharSlice<'_> {
    fn eq(&self, other: &&[char]) -> bool {
        self.0.len() == other.len()
            && self
                .0
                .iter()
                .zip(other.iter())
                .all(|(a, b)| a.to_ascii_lowercase() == *b)
    }
}

/// Case-insensitive comparison with a character array, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl<const N: usize> PartialEq<&[char; N]> for CharSlice<'_> {
    fn eq(&self, other: &&[char; N]) -> bool {
        *self == &other[..]
    }
}

/// Case-insensitive comparison with a character array, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl<const N: usize> PartialEq<[char; N]> for CharSlice<'_> {
    fn eq(&self, other: &[char; N]) -> bool {
        *self == &other[..]
    }
}

/// Case-insensitive comparison with a string slice, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl PartialEq<&str> for CharSlice<'_> {
    fn eq(&self, other: &&str) -> bool {
        let mut chit = self.0.iter();
        let mut strit = other.chars();

        loop {
            let (c, s) = (chit.next(), strit.next());
            match (c, s) {
                (Some(c), Some(s)) => {
                    if c.to_ascii_lowercase() != s {
                        return false;
                    }
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}

// TODO I can't get this one to work.
// TODO It would be used in `harper-core/src/linting/plural_wrong_word_of_phrase.rs`
// impl PartialEq<String> for CharSlice<'_> {
//     fn eq(&self, other: &String) -> bool {
//         self == other.as_str()
//     }
// }

impl PartialEq<str> for &CharSlice<'_> {
    fn eq(&self, other: &str) -> bool {
        **self == other
    }
}

/// Case-insensitive comparison with a character slice, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl PartialEq<&[char]> for &CharSlice<'_> {
    fn eq(&self, other: &&[char]) -> bool {
        **self == *other
    }
}

/// Case-insensitive comparison with a character array, assuming the right-hand side is lowercase ASCII.
/// Only normalizes the left side to lowercase and avoids allocations.
impl<const N: usize> PartialEq<[char; N]> for &CharSlice<'_> {
    fn eq(&self, other: &[char; N]) -> bool {
        **self == &other[..]
    }
}
