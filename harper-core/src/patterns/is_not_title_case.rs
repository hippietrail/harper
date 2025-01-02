use crate::{make_title_case, Dictionary, Token, TokenStringExt};

use super::Pattern;

/// Will match full length of wrapped pattern __only if the matched
/// text is not already title case__.
pub struct IsNotTitleCase<D: Dictionary> {
    inner: Box<dyn Pattern>,
    dict: D,
}

impl<D: Dictionary> IsNotTitleCase<D> {
    pub fn new(inner: Box<dyn Pattern>, dict: D) -> Self {
        Self { inner, dict }
    }
}

impl<D: Dictionary> Pattern for IsNotTitleCase<D> {
    fn matches(&self, tokens: &[Token], source: &[char]) -> usize {
        let inner_match = self.inner.matches(tokens, source);

        if inner_match == 0 {
            return 0;
        }

        let matched_chars = tokens[0..inner_match].span().unwrap().get_content(source);

        if make_title_case(tokens, source, &self.dict) != matched_chars {
            inner_match
        } else {
            0
        }
    }
}
