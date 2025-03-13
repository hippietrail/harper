use std::hash::BuildHasher;

use foldhash::fast::FixedState;

use crate::CharStringExt;

/// An identifier for a particular word.
///
/// It works by hashing the word it represents, normalized to lowercase.
/// It is meant for situations where you need to refer to a word (or a collection of words),
/// without storing all of accompanying data (like spelling or metadata).
#[derive(Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Debug)]
pub struct WordId {
    hash: u64,
}

impl WordId {
    /// Create a Word ID from
    pub fn from_word_chars(chars: impl AsRef<[char]>) -> Self {
        let normalized = chars.as_ref().normalized();
        let lower = normalized.to_lower();
        let hash = FixedState::default().hash_one(lower);

        Self { hash }
    }
}
