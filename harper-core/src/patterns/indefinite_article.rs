use crate::Token;

use super::{Pattern, SequencePattern, WordSet};

pub struct IndefiniteArticle {
    inner: SequencePattern,
}

impl Default for IndefiniteArticle {
    fn default() -> Self {
        Self {
            inner: SequencePattern::default().then_word_set(WordSet::all(&["a", "an"])),
        }
    }
}

impl Pattern for IndefiniteArticle {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        self.inner.matches(tokens, source)
    }
}
